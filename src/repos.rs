//! Fetch and cache public repositories for the configured GitHub organization.

mod build_state;
mod build_status;
mod github_repo;
mod repo_view;
mod repos_error;
mod workflow_run;
mod workflow_runs_response;
pub use build_state::BuildState;
pub use build_status::BuildStatus;
pub(crate) use github_repo::GithubRepo;
pub use repo_view::RepoView;
pub use repos_error::ReposError;
pub(crate) use workflow_run::WorkflowRun;
pub(crate) use workflow_runs_response::WorkflowRunsResponse;

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use sigma_theme::cache::TtlCache;

use crate::config;

// Repos and their CI status change slowly; cache long enough that a single
// refresh burst (one call per repo with a workflow) stays well under GitHub's
// unauthenticated rate limit. Set STORG_GITHUB_TOKEN to lift the limit.
const CACHE_TTL: Duration = Duration::from_secs(30 * 60);
const USER_AGENT: &str = "sigmatactical-org/0.1 (+https://sigmatactical.org)";

/// Process-wide listing cache: single-flight refresh, stale-on-error, and
/// `Arc` handles so a request never deep-clones the listing.
static REPOS: TtlCache<Vec<RepoView>> = TtlCache::new();

/// Process-wide HTTP client (connection pooling, TLS session reuse).
fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

/// Apply the standard GitHub headers (and auth token, when configured).
fn github_headers(builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    let builder = builder
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json");
    match config::github_token() {
        Some(token) => builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {token}")),
        None => builder,
    }
}

async fn fetch_org_repos(client: &reqwest::Client) -> Result<Vec<RepoView>, ReposError> {
    let org = config::github_org();
    let url = format!(
        "{}/orgs/{org}/repos?per_page=100&sort=updated&type=public",
        config::github_api_base()
    );
    let response = github_headers(client.get(&url)).send().await?;

    if !response.status().is_success() {
        return Err(ReposError::Api(format!(
            "GET {url} returned {}",
            response.status()
        )));
    }

    let entries: Vec<GithubRepo> = response.json().await?;
    let mut repos = entries_to_views(entries);
    attach_build_statuses(client, &mut repos).await;
    Ok(repos)
}

/// Fetch the latest default-branch CI run for every repo with a known workflow
/// (see [`crate::catalog::primary_workflow`]) and attach it as [`BuildStatus`].
/// Runs concurrently; any per-repo failure simply leaves that repo without a pill.
async fn attach_build_statuses(client: &reqwest::Client, repos: &mut [RepoView]) {
    let org = config::github_org();
    let api_base = config::github_api_base();

    let mut set = tokio::task::JoinSet::new();
    for (idx, repo) in repos.iter().enumerate() {
        let Some(workflow) = crate::catalog::primary_workflow(&repo.name) else {
            continue;
        };
        let client = client.clone();
        let (org, api_base, name, html_url) = (
            org.clone(),
            api_base.clone(),
            repo.name.clone(),
            repo.url.clone(),
        );
        let branch = if repo.default_branch.is_empty() {
            "main".to_string()
        } else {
            repo.default_branch.clone()
        };
        set.spawn(async move {
            let status = fetch_build_status(
                &client, &api_base, &org, &name, &html_url, &branch, workflow,
            )
            .await;
            (idx, status)
        });
    }

    while let Some(joined) = set.join_next().await {
        if let Ok((idx, Some(status))) = joined {
            repos[idx].build = Some(status);
        }
    }
}

async fn fetch_build_status(
    client: &reqwest::Client,
    api_base: &str,
    org: &str,
    name: &str,
    html_url: &str,
    branch: &str,
    workflow: &str,
) -> Option<BuildStatus> {
    let url = format!(
        "{api_base}/repos/{org}/{name}/actions/workflows/{workflow}/runs?branch={branch}&per_page=1"
    );
    let response = github_headers(client.get(&url)).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body: WorkflowRunsResponse = response.json().await.ok()?;
    let run = body.workflow_runs.into_iter().next()?;
    Some(BuildStatus {
        state: BuildState::from_run(&run.status, run.conclusion.as_deref()),
        url: format!("{html_url}/actions/workflows/{workflow}"),
    })
}

fn entries_to_views(entries: Vec<GithubRepo>) -> Vec<RepoView> {
    let mut repos: Vec<RepoView> = entries
        .into_iter()
        .filter(|repo| !repo.archived && !repo.fork)
        .map(|repo| RepoView {
            name: repo.name,
            url: repo.html_url,
            description: repo.description.unwrap_or_default(),
            language: repo.language.unwrap_or_default(),
            stars: repo.stargazers_count,
            default_branch: repo.default_branch,
            build: None,
        })
        .collect();
    repos.sort_by(|a, b| b.stars.cmp(&a.stars).then_with(|| a.name.cmp(&b.name)));
    repos
}

/// Returns cached public repositories for the configured GitHub org.
///
/// Concurrent misses share one upstream fetch; when a refresh fails the last
/// good listing keeps being served.
///
/// # Errors
///
/// Returns [`ReposError`] only when the fetch fails with nothing cached.
pub async fn list_public_repos() -> Result<Arc<Vec<RepoView>>, ReposError> {
    REPOS
        .get_or_fetch(CACHE_TTL, || fetch_org_repos(client()))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_archived_and_forks() {
        let json = r#"[
          {"name":"active","html_url":"https://github.com/o/active","description":"A","language":"Rust","stargazers_count":3,"archived":false,"fork":false},
          {"name":"arch","html_url":"https://github.com/o/arch","description":null,"language":null,"stargazers_count":1,"archived":true,"fork":false},
          {"name":"fork","html_url":"https://github.com/o/fork","description":null,"language":"Go","stargazers_count":9,"archived":false,"fork":true}
        ]"#;
        let entries: Vec<GithubRepo> = serde_json::from_str(json).expect("parse");
        let repos = entries_to_views(entries);
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "active");
        assert_eq!(repos[0].stars, 3);
    }
}

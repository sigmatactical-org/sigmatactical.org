//! Fetch and cache public repositories for the configured GitHub organization.

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::RwLock;

use crate::config;

// Repos and their CI status change slowly; cache long enough that a single
// refresh burst (one call per repo with a workflow) stays well under GitHub's
// unauthenticated rate limit. Set STORG_GITHUB_TOKEN to lift the limit.
const CACHE_TTL: Duration = Duration::from_secs(30 * 60);
const USER_AGENT: &str = "sigmatactical-org/0.1 (+https://sigmatactical.org)";

/// Outcome of a repository's latest CI run on its default branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildState {
    Passing,
    Failing,
    Pending,
}

impl BuildState {
    /// Bootstrap contextual class for the status pill.
    #[must_use]
    pub const fn css_class(self) -> &'static str {
        match self {
            BuildState::Passing => "text-bg-success",
            BuildState::Failing => "text-bg-danger",
            BuildState::Pending => "text-bg-secondary",
        }
    }

    /// Human-readable label for the status pill.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            BuildState::Passing => "CI passing",
            BuildState::Failing => "CI failing",
            BuildState::Pending => "CI pending",
        }
    }

    fn from_run(status: &str, conclusion: Option<&str>) -> Self {
        if status != "completed" {
            return BuildState::Pending;
        }
        match conclusion {
            Some("success") => BuildState::Passing,
            Some("failure" | "timed_out" | "startup_failure" | "cancelled") => BuildState::Failing,
            _ => BuildState::Pending,
        }
    }
}

/// Build status shown on a repository card (state + link to the workflow runs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildStatus {
    pub state: BuildState,
    pub url: String,
}

/// One repository row on the home page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoView {
    pub name: String,
    pub url: String,
    pub description: String,
    pub language: String,
    pub stars: u32,
    pub default_branch: String,
    pub build: Option<BuildStatus>,
}

#[derive(Debug, Error)]
pub enum ReposError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("GitHub API error: {0}")]
    Api(String),
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    name: String,
    html_url: String,
    description: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
    archived: bool,
    fork: bool,
    #[serde(default)]
    default_branch: String,
}

#[derive(Debug, Deserialize)]
struct WorkflowRunsResponse {
    #[serde(default)]
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    status: String,
    conclusion: Option<String>,
}

struct CacheState {
    repos: Option<Vec<RepoView>>,
    fetched_at: Option<Instant>,
}

impl CacheState {
    const fn empty() -> Self {
        Self {
            repos: None,
            fetched_at: None,
        }
    }

    fn is_fresh(&self) -> bool {
        self.repos
            .as_ref()
            .is_some_and(|_| self.fetched_at.is_some_and(|at| at.elapsed() < CACHE_TTL))
    }
}

struct ReposCache {
    client: reqwest::Client,
    state: RwLock<CacheState>,
}

impl ReposCache {
    fn global() -> &'static ReposCache {
        static CACHE: OnceLock<ReposCache> = OnceLock::new();
        CACHE.get_or_init(|| ReposCache {
            client: reqwest::Client::new(),
            state: RwLock::new(CacheState::empty()),
        })
    }

    async fn list(&self) -> Result<Vec<RepoView>, ReposError> {
        {
            let state = self.state.read().await;
            if state.is_fresh() {
                return Ok(state.repos.clone().unwrap_or_default());
            }
        }

        let repos = fetch_org_repos(&self.client).await?;
        let mut state = self.state.write().await;
        state.repos = Some(repos.clone());
        state.fetched_at = Some(Instant::now());
        Ok(repos)
    }
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
            let status =
                fetch_build_status(&client, &api_base, &org, &name, &html_url, &branch, workflow)
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
pub async fn list_public_repos() -> Result<Vec<RepoView>, ReposError> {
    ReposCache::global().list().await
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

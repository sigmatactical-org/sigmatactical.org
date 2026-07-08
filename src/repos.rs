//! Fetch and cache public repositories for the configured GitHub organization.

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::RwLock;

use crate::config;

const CACHE_TTL: Duration = Duration::from_secs(15 * 60);
const USER_AGENT: &str = "sigmatactical-org/0.1 (+https://sigmatactical.org)";

/// One repository row on the home page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoView {
    pub name: String,
    pub url: String,
    pub description: String,
    pub language: String,
    pub stars: u32,
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

async fn fetch_org_repos(client: &reqwest::Client) -> Result<Vec<RepoView>, ReposError> {
    let org = config::github_org();
    let url = format!(
        "{}/orgs/{org}/repos?per_page=100&sort=updated&type=public",
        config::github_api_base()
    );
    let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(ReposError::Api(format!(
            "GET {url} returned {}",
            response.status()
        )));
    }

    let entries: Vec<GithubRepo> = response.json().await?;
    Ok(entries_to_views(entries))
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

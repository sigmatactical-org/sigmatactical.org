//! [`ReposCache`].

#[allow(unused_imports)]
use super::*;
use std::sync::OnceLock;
use std::time::Instant;
use tokio::sync::RwLock;

/// TTL cache of the GitHub repo listing.
pub(crate) struct ReposCache {
    pub(crate) client: reqwest::Client,
    pub(crate) state: RwLock<CacheState>,
}
impl ReposCache {
    /// Process-wide cache instance.
    pub(crate) fn global() -> &'static ReposCache {
        static CACHE: OnceLock<ReposCache> = OnceLock::new();
        CACHE.get_or_init(|| ReposCache {
            client: reqwest::Client::new(),
            state: RwLock::new(CacheState::empty()),
        })
    }

    pub(crate) async fn list(&self) -> Result<Vec<RepoView>, ReposError> {
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

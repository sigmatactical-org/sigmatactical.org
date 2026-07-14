//! [`CacheState`].

#[allow(unused_imports)]
use super::*;
use std::time::Instant;

/// Cached payload plus the instant it was fetched.
pub(crate) struct CacheState {
    pub(crate) repos: Option<Vec<RepoView>>,
    pub(crate) fetched_at: Option<Instant>,
}
impl CacheState {
    /// Empty, immediately-stale state.
    pub(crate) const fn empty() -> Self {
        Self {
            repos: None,
            fetched_at: None,
        }
    }

    /// Whether the cached payload is still within its TTL.
    pub(crate) fn is_fresh(&self) -> bool {
        self.repos
            .as_ref()
            .is_some_and(|_| self.fetched_at.is_some_and(|at| at.elapsed() < CACHE_TTL))
    }
}

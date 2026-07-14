//! [`BuildStatus`].

#[allow(unused_imports)]
use super::*;

/// Build status shown on a repository card (state + link to the workflow runs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildStatus {
    pub state: BuildState,
    pub url: String,
}

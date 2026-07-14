//! [`RepoView`].

#[allow(unused_imports)]
use super::*;

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

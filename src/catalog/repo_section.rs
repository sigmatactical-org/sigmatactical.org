//! [`RepoSection`].

#[allow(unused_imports)]
use super::*;

/// A themed group of repositories with an introduction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoSection {
    pub id: &'static str,
    pub title: &'static str,
    pub intro: &'static str,
    pub repos: Vec<EnrichedRepo>,
}

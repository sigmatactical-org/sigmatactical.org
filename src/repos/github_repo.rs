//! [`GithubRepo`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GithubRepo {
    pub(crate) name: String,
    pub(crate) html_url: String,
    pub(crate) description: Option<String>,
    pub(crate) language: Option<String>,
    pub(crate) stargazers_count: u32,
    pub(crate) archived: bool,
    pub(crate) fork: bool,
    #[serde(default)]
    pub(crate) default_branch: String,
}

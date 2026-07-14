//! [`ReposError`].

#[allow(unused_imports)]
use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReposError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("GitHub API error: {0}")]
    Api(String),
}

//! [`EnrichedRepo`].

#[allow(unused_imports)]
use super::*;
use crate::repos::BuildStatus;

/// Repository row enriched with editorial relevance text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrichedRepo {
    pub name: String,
    pub url: String,
    pub description: String,
    pub relevance: String,
    pub language: String,
    pub stars: u32,
    pub build: Option<BuildStatus>,
}

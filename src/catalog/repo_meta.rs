//! [`RepoMeta`].

/// Hand-curated metadata for one repository.
pub(crate) struct RepoMeta {
    pub(crate) section_id: &'static str,
    pub(crate) relevance: &'static str,
    pub(crate) description: &'static str,
    pub(crate) order: u16,
}

//! [`SectionMeta`].

#[allow(unused_imports)]
use super::*;

/// Hand-curated metadata for one catalog section.
pub(crate) struct SectionMeta {
    pub(crate) id: &'static str,
    pub(crate) title: &'static str,
    pub(crate) intro: &'static str,
    pub(crate) order: u16,
}

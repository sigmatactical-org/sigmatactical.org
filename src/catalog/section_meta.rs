//! [`SectionMeta`].

/// Hand-curated metadata for one catalog section. Display order is the
/// position in [`super::SECTIONS`].
pub(crate) struct SectionMeta {
    pub(crate) id: &'static str,
    pub(crate) title: &'static str,
    pub(crate) intro: &'static str,
}

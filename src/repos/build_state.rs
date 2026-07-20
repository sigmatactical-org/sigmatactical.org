//! [`BuildState`].

/// Outcome of a repository's latest CI run on its default branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildState {
    Passing,
    Failing,
    Pending,
}
impl BuildState {
    /// Bootstrap contextual class for the status pill.
    #[must_use]
    pub const fn css_class(self) -> &'static str {
        match self {
            BuildState::Passing => "text-bg-success",
            BuildState::Failing => "text-bg-danger",
            BuildState::Pending => "text-bg-secondary",
        }
    }

    /// Human-readable label for the status pill.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            BuildState::Passing => "CI passing",
            BuildState::Failing => "CI failing",
            BuildState::Pending => "CI pending",
        }
    }

    /// Map a workflow run status/conclusion to a badge state.
    pub(crate) fn from_run(status: &str, conclusion: Option<&str>) -> Self {
        if status != "completed" {
            return BuildState::Pending;
        }
        match conclusion {
            Some("success") => BuildState::Passing,
            Some("failure" | "timed_out" | "startup_failure" | "cancelled") => BuildState::Failing,
            _ => BuildState::Pending,
        }
    }
}

//! [`WorkflowRun`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRun {
    pub(crate) status: String,
    pub(crate) conclusion: Option<String>,
}

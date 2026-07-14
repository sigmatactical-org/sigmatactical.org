//! [`WorkflowRunsResponse`].

#[allow(unused_imports)]
use super::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRunsResponse {
    #[serde(default)]
    pub(crate) workflow_runs: Vec<WorkflowRun>,
}

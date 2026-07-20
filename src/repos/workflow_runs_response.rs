//! [`WorkflowRunsResponse`].

use super::WorkflowRun;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct WorkflowRunsResponse {
    #[serde(default)]
    pub(crate) workflow_runs: Vec<WorkflowRun>,
}

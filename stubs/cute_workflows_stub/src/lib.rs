// Stub for cute-workflows (warp-workflows).
// Provides minimal types needed for compilation when workflows are disabled.

pub mod workflows {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Workflow;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WorkflowCategory;
}
// Stub for cute_multi_agent_api (warp_multi_agent_api).
// Provides minimal types needed for compilation when AI features are disabled.

use serde::{Deserialize, Serialize};
use crate::message::Message;

pub mod client_action {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Action;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StartNewConversation;
}

pub mod response_event {
    pub mod stream_finished {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct StreamFinished;

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct TokenUsage;

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ModelTokenUsage {
            pub model_id: String,
            pub total_tokens: u32,
            pub token_usage_by_category: Vec<(String, u32)>,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ToolCallStats {
            pub count: u64,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct RunCommandStats {
            pub count: u64,
            pub command_executed: String,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ApplyFileDiffStats {
            pub count: u64,
            pub files_changed: u64,
            pub lines_added: u64,
            pub lines_removed: u64,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ToolUsageMetadata {
            pub tool_call_stats: Option<Vec<ToolCallStats>>,
            pub run_command_stats: Option<Vec<RunCommandStats>>,
            pub apply_file_diff_stats: Option<Vec<ApplyFileDiffStats>>,
            pub read_files_stats: Option<Vec<ToolCallStats>>,
            pub search_codebase_stats: Option<Vec<ToolCallStats>>,
            pub grep_stats: Option<Vec<ToolCallStats>>,
            pub file_glob_stats: Option<Vec<ToolCallStats>>,
            pub call_mcp_tool_stats: Option<Vec<ToolCallStats>>,
            pub read_mcp_resource_stats: Option<Vec<ToolCallStats>>,
            pub read_shell_command_output_stats: Option<Vec<ToolCallStats>>,
            pub write_to_long_running_shell_command_stats: Option<Vec<ToolCallStats>>,
            pub suggest_plan_stats: Option<Vec<ToolCallStats>>,
            pub suggest_create_plan_stats: Option<Vec<ToolCallStats>>,
            pub use_computer_stats: Option<Vec<ToolCallStats>>,
        }
    }
}

pub mod message {
    pub mod tool_call {
        pub mod subagent {
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Metadata;
        }
        pub mod tool_call_result {
            pub type Result = ();
        }
    }

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Message;
}

pub mod ask_user_question_result {
    pub mod answer_item {
        pub type Answer = ();
    }
}

pub mod diff_hunk {
    // re-exported as diff_hunk_api
}

#[derive(Debug, Clone)]
pub struct AgentEvent;

#[derive(Debug, Clone)]
pub enum AgentType {
    Default,
}

#[derive(Debug, Clone)]
pub struct ConversationData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDependencies {
    pub parent_task_id: String,
}

impl Default for TaskDependencies {
    fn default() -> Self {
        Self {
            parent_task_id: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub dependencies: Option<TaskDependencies>,
    pub messages: Vec<Message>,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            dependencies: None,
            messages: vec![],
            status: TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToolType {
    Default,
}

#[derive(Debug, Clone)]
pub enum LlmProvider {
    Default,
}

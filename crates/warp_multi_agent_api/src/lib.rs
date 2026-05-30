//! Stub implementation for warp_multi_agent_api crate
//! This provides minimal types to satisfy compilation without actual multi-agent functionality

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: String,
    pub input: Option<request::Input>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub reference: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub server_message_data: String,
    pub citations: Vec<String>,
    pub message: Option<message::Message>,
    pub request_id: String,
    pub timestamp: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

pub mod message {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AgentOutput {
        pub text: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ToolCall {
        pub tool_call_id: String,
        pub tool: Option<tool_call::Tool>,
    }

    pub mod tool_call {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Subagent {
            pub task_id: String,
            pub payload: String,
            pub metadata: Option<subagent::Metadata>,
        }

        pub mod subagent {
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Metadata {
                pub id: String,
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Tool {
            Subagent(Subagent),
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Message {
        AgentOutput(AgentOutput),
        ToolCall(ToolCall),
    }
}

pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Input {
        pub r#type: Option<input::Type>,
    }

    pub mod input {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Type {
            GeneratePassiveSuggestions(GeneratePassiveSuggestions),
            Other,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GeneratePassiveSuggestions {
            pub id: String,
        }
    }
}

pub mod client_action {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    pub struct ClientAction {
        pub action: Option<Action>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Action {
        CreateTask(CreateTask),
        AddMessagesToTask(AddMessagesToTask),
        CancelTask(CancelTask),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateTask {
        pub task_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddMessagesToTask {
        pub task_id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CancelTask {
        pub task_id: String,
    }
}

pub mod run_command_response {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result {
        Success(Success),
        Error(Error),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success {
        pub exit_code: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error {
        pub message: String,
    }
}

#[derive(Debug, Clone)]
pub struct ResponseEvent {
    pub r#type: Option<response_event::Type>,
}

impl ResponseEvent {
    pub fn decode(_bytes: &[u8]) -> Result<Self, prost::DecodeError> {
        // Stub implementation - return empty event
        Ok(Self { r#type: None })
    }
}

pub mod response_event {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct StreamFinished;

    pub fn stream_finished() -> StreamFinished {
        StreamFinished
    }

    #[derive(Debug, Clone)]
    pub struct StreamInit {
        pub task_id: String,
    }

    #[derive(Debug, Clone)]
    pub struct ClientActions {
        pub actions: Vec<super::client_action::ClientAction>,
    }

    #[derive(Debug, Clone)]
    pub enum Type {
        Init(StreamInit),
        ClientActions(ClientActions),
        Finished(StreamFinished),
    }

    pub mod stream_finished {
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Done {}

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Cancelled {}

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Error {
            pub message: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Reason {
            Done(Done),
            Cancelled(Cancelled),
            Error(Error),
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct ModelTokenUsage {
            pub model_id: String,
            pub total_tokens: u32,
            pub token_usage_by_category: HashMap<String, u32>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct ToolCallStats {
            pub count: i32,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct RunCommandStats {
            pub count: i32,
            pub command_executed: i32,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct ApplyFileDiffStats {
            pub count: i32,
            pub lines_added: i32,
            pub lines_removed: i32,
            pub files_changed: i32,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct ToolUsageMetadata {
            pub tool_call_stats: Option<ToolCallStats>,
            pub run_command_stats: Option<RunCommandStats>,
            pub apply_file_diff_stats: Option<ApplyFileDiffStats>,
            pub read_files_stats: Option<ToolCallStats>,
            pub search_codebase_stats: Option<ToolCallStats>,
            pub grep_stats: Option<ToolCallStats>,
            pub file_glob_stats: Option<ToolCallStats>,
            pub write_to_long_running_shell_command_stats: Option<ToolCallStats>,
            pub read_mcp_resource_stats: Option<ToolCallStats>,
            pub call_mcp_tool_stats: Option<ToolCallStats>,
            pub suggest_plan_stats: Option<ToolCallStats>,
            pub suggest_create_plan_stats: Option<ToolCallStats>,
            pub read_shell_command_output_stats: Option<ToolCallStats>,
            pub use_computer_stats: Option<ToolCallStats>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct ConversationUsageMetadata {
            pub model_token_usage: Vec<ModelTokenUsage>,
            pub tool_usage_metadata: Option<ToolUsageMetadata>,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    pub id: String,
    pub messages: Vec<String>,
}

pub mod ai {
    #[derive(Debug, Clone)]
    pub struct AgentHarness;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub dependencies: Option<TaskDependencies>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskDependencies {
    pub parent_task_id: String,
}

// Re-export ClientAction at crate root for convenience
pub use client_action::ClientAction;

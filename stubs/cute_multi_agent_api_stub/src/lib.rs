// Stub for cute_multi_agent_api (warp_multi_agent_api).
// Provides minimal types needed for compilation when AI features are disabled.

use serde::{Deserialize, Serialize};

// ============================================================================
// Modules
// ============================================================================

pub mod client_action {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Action {
        BeginTransaction(()),
        CommitTransaction(()),
        RollbackTransaction(()),
        CreateTask(()),
        UpdateTaskDescription(()),
        AddMessagesToTask(()),
        UpdateTaskServerData(()),
        UpdateTaskMessage(()),
        AppendToMessageContent(()),
        ShowSuggestions(()),
        MoveMessagesToNewTask(()),
        StartNewConversation(()),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StartNewConversation;
}

pub mod response_event {
    #[derive(Debug, Clone, Default)]
    pub struct StreamInit {
        pub request_id: Option<String>,
        pub conversation_id: Option<String>,
        pub run_id: Option<String>,
    }

    pub use stream_finished::StreamFinished;

    pub mod stream_finished {
        use std::collections::HashMap;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct StreamFinished;

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct TokenUsage {
            pub model_id: String,
            pub total_input: u32,
            pub output: u32,
            pub input_cache_read: u32,
            pub input_cache_write: u32,
            pub cost_in_cents: u32,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ConversationUsageMetadata {
            pub warp_token_usage: Option<TokenUsage>,
            pub byok_token_usage: Option<TokenUsage>,
            pub custom_endpoint_token_usage: Option<TokenUsage>,
            pub context_window_usage: Option<TokenUsage>,
            pub credits_spent: Option<i32>,
            pub tool_usage_metadata: Option<ToolUsageMetadata>,
            pub summarized: bool,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct InternalError;

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ModelTokenUsage {
            pub model_id: String,
            pub total_tokens: u32,
            pub token_usage_by_category: HashMap<String, u32>,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ToolCallStats {
            pub count: i32,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct RunCommandStats {
            pub count: i32,
            pub command_executed: i32,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ApplyFileDiffStats {
            pub count: i32,
            pub files_changed: i32,
            pub lines_added: i32,
            pub lines_removed: i32,
        }

        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct ToolUsageMetadata {
            pub tool_call_stats: Option<ToolCallStats>,
            pub run_command_stats: Option<RunCommandStats>,
            pub apply_file_diff_stats: Option<ApplyFileDiffStats>,
            pub read_files_stats: Option<ToolCallStats>,
            pub search_codebase_stats: Option<ToolCallStats>,
            pub grep_stats: Option<ToolCallStats>,
            pub file_glob_stats: Option<ToolCallStats>,
            pub call_mcp_tool_stats: Option<ToolCallStats>,
            pub read_mcp_resource_stats: Option<ToolCallStats>,
            pub read_shell_command_output_stats: Option<ToolCallStats>,
            pub write_to_long_running_shell_command_stats: Option<ToolCallStats>,
            pub suggest_plan_stats: Option<ToolCallStats>,
            pub suggest_create_plan_stats: Option<ToolCallStats>,
            pub use_computer_stats: Option<ToolCallStats>,
        }
    }
}

pub mod message {
    pub mod tool_call {
        pub mod subagent {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Subagent;
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Metadata {
                Cli(()),
                Research(()),
                Advice(()),
                ComputerUse(()),
                Summarization(()),
                ConversationSearch(()),
                WarpDocumentationSearch(()),
            }

            pub mod conversation_search_metadata {
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct Target;
            }
        }

        pub mod tool_call_result {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Result {
                RunShellCommand(super::super::super::RunShellCommandResult),
                Subagent(()),
                Other(()),
            }
        }

        pub mod run_shell_command {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct RunShellCommand;
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum WaitUntilCompleteValue { WaitUntilComplete(bool) }
        }

        pub mod read_files {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct ReadFiles;
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct File {
                pub name: String,
                pub line_ranges: Vec<super::super::super::FileContentLineRange>,
            }
        }

        pub mod suggest_prompt {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct SuggestPrompt;

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct InlineQueryBanner {
                pub title: String,
                pub description: String,
                pub query: String,
            }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct PromptChip {
                pub label: String,
                pub prompt: String,
            }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum DisplayMode { InlineQueryBanner(InlineQueryBanner), PromptChip(PromptChip) }
        }

        pub mod read_shell_command_output {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct ReadShellCommandOutput;
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Duration { pub seconds: i64 }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Delay { Duration(Duration), OnCompletion(()) }
        }

        pub mod insert_review_comments {
            use serde::{Deserialize, Serialize};

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct InsertReviewComments;

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum CommentSide { New, Old }

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Comment {
                pub comment_id: String,
                pub author: String,
                pub last_modified_timestamp: String,
                pub comment_body: String,
                pub parent_comment_id: String,
                pub html_url: String,
                pub location: Option<CommentLocation>,
            }

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct CommentLocation {
                pub file_path: String,
                pub line: Option<CommentLineRange>,
            }

            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct CommentLineRange {
                pub range: Option<super::super::super::FileContentLineRange>,
                pub diff_hunk: String,
            }

            impl CommentLineRange {
                pub fn side(&self) -> CommentSide { CommentSide::New }
            }
        }

        pub mod read_skill {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum SkillReference { SkillPath(String), BundledSkillId(String) }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct ReadSkill;
        }

        pub mod write_to_long_running_shell_command {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Mode { pub mode: Option<mode::Mode> }
            pub mod mode {
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub enum Mode { Block(()), Raw(()), Line(()) }
            }
        }

        pub mod use_computer {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct UseComputer;

            pub mod action {
                use serde::{Deserialize, Serialize};
                use super::super::super::super::Coordinates;

                #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
                pub enum MouseButton { Left, Right, Middle, Back, Forward }

                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct Key { pub data: Option<key::Data> }

                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct MouseMoveAction { pub to: Option<Coordinates> }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct MouseDownAction { pub button: MouseButton, pub at: Option<Coordinates> }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct MouseUpAction { pub button: MouseButton }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct MouseWheelAction { pub direction: mouse_wheel::Direction, pub distance: Option<mouse_wheel::Distance>, pub at: Option<Coordinates> }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct WaitAction { pub duration: Option<Duration> }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct TypeTextAction { pub text: String }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct KeyDownAction { pub key: Option<Key> }
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct KeyUpAction { pub key: Option<Key> }

                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct Duration { pub seconds: i32, pub nanos: i32 }
                impl Default for Duration {
                    fn default() -> Self { Self { seconds: 0, nanos: 0 } }
                }

                impl MouseDownAction { pub fn button(&self) -> MouseButton { self.button } }
                impl MouseUpAction { pub fn button(&self) -> MouseButton { self.button } }
                impl MouseWheelAction { pub fn direction(&self) -> mouse_wheel::Direction { self.direction } }
                impl MouseWheelAction { pub fn distance(&self) -> Option<mouse_wheel::Distance> { self.distance } }

                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub enum Type {
                    MouseMove(MouseMoveAction),
                    MouseDown(MouseDownAction),
                    MouseUp(MouseUpAction),
                    MouseWheel(MouseWheelAction),
                    Wait(WaitAction),
                    TypeText(TypeTextAction),
                    KeyDown(KeyDownAction),
                    KeyUp(KeyUpAction),
                }

                pub mod mouse_wheel {
                    use serde::{Deserialize, Serialize};
                    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
                    pub enum Direction { Up, Down, Left, Right }
                    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
                    pub enum Distance { Pixels(i32), Clicks(i32) }
                }

                pub mod key {
                    use serde::{Deserialize, Serialize};
                    #[derive(Debug, Clone, Serialize, Deserialize)]
                    pub enum Data { Keycode(i32), Char(String) }
                }
            }
        }

        // Tool call types (structs directly in tool_call)
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileDiff {
            pub search: String,
            pub replace: String,
            pub file_path: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct V4AUpdate {
            pub file_path: String,
            pub move_to: String,
            pub hunks: Vec<V4AHunkData>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct V4AHunkData {
            pub change_context: Vec<String>,
            pub pre_context: String,
            pub old: String,
            pub new: String,
            pub post_context: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeletedFile {
            pub file_path: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct NewFile {
            pub file_path: String,
            pub content: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApplyFileDiffs {
            pub diffs: Vec<FileDiff>,
            pub summary: String,
            pub new_files: Vec<NewFile>,
            pub deleted_files: Vec<DeletedFile>,
            pub v4a_updates: Vec<V4AUpdate>,
        }
        #[derive(Debug, Clone)]
        pub struct CallMcpTool { pub name: String, pub args: Option<prost_types::Struct>, pub server_id: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct NewDocument {
            pub content: String,
            pub title: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateDocuments { pub new_documents: Vec<NewDocument> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DocumentDiff {
            pub document_id: String,
            pub search: String,
            pub replace: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EditDocuments { pub diffs: Vec<DocumentDiff> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FetchConversation { pub conversation_id: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileGlob { pub path: String, pub patterns: Vec<String> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileGlobV2 { pub patterns: Vec<String>, pub search_dir: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Grep { pub path: String, pub queries: Vec<String> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DocumentRef {
            pub document_id: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ReadDocuments { pub documents: Vec<DocumentRef> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ReadMcpResource { pub server_id: String, pub uri: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RequestComputerUse { pub screenshot_params: Option<ScreenshotParams>, pub task_summary: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SearchCodebase { pub query: String, pub codebase_path: String, pub path_filters: Vec<String> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestNewConversation { pub message_id: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferShellCommandControlToUser { pub reason: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct WriteToLongRunningShellCommand { pub command_id: String, pub input: String, pub mode: Option<write_to_long_running_shell_command::Mode> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct InsertReviewComments {
            pub repo_path: String,
            pub base_branch: String,
            pub comments: Vec<insert_review_comments::Comment>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ReadFiles { pub files: Vec<read_files::File> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ReadShellCommandOutput { pub command_id: String, pub delay: Option<read_shell_command_output::Delay> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RunShellCommand {
            pub command: String,
            pub command_id: String,
            pub is_read_only: bool,
            pub is_risky: bool,
            pub uses_pager: bool,
            pub wait_until_complete_value: Option<run_shell_command::WaitUntilCompleteValue>,
            pub citations: Vec<super::super::Citation>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileRef {
            pub file_path: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UploadFileArtifact {
            pub file: Option<FileRef>,
            pub description: String,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestPrompt { pub display_mode: Option<suggest_prompt::DisplayMode> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UseComputerAction {
            pub r#type: Option<use_computer::action::Type>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UseComputer {
            pub actions: Vec<UseComputerAction>,
            pub post_actions_screenshot_params: Option<ScreenshotParams>,
            pub action_summary: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ScreenshotParams {
            pub max_long_edge_px: i32,
            pub max_total_px: i32,
            pub region: Option<ScreenshotRegion>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ScreenshotRegion {
            pub top_left: Option<super::super::Coordinates>,
            pub bottom_right: Option<super::super::Coordinates>,
        }
    }

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum Message {
        UserQuery(()),
        SystemQuery(system_query::SystemQuery),
        AgentOutput(agent_output::AgentOutput),
        AgentReasoning(agent_reasoning::AgentReasoning),
        ToolCall(()),
        ToolCallResult(tool_call_result::ToolCallResult),
        Summarization(summarization::Summarization),
        WebSearch(()),
        WebFetch(()),
        ModelUsed(model_used::ModelUsed),
        UpdateTodos(update_todos::UpdateTodosData),
        UpdateReviewComments(()),
        DebugOutput(()),
        ArtifactEvent(()),
        MessagesReceivedFromAgents(()),
        EventsFromAgents(()),
        PassiveSuggestionResult(()),
        CodeReview(()),
        ServerEvent(()),
        InvokeSkill(()),
        OrchestrationConfigSnapshot(()),
    }

    impl Message {
        pub fn variant_name(&self) -> &'static str {
            match self {
                Message::UserQuery(_) => "UserQuery",
                Message::SystemQuery(_) => "SystemQuery",
                Message::AgentOutput(_) => "AgentOutput",
                Message::AgentReasoning(_) => "AgentReasoning",
                Message::ToolCall(_) => "ToolCall",
                Message::ToolCallResult(_) => "ToolCallResult",
                Message::Summarization(_) => "Summarization",
                Message::WebSearch(_) => "WebSearch",
                Message::WebFetch(_) => "WebFetch",
                Message::ModelUsed(_) => "ModelUsed",
                Message::UpdateTodos(_) => "UpdateTodos",
                Message::UpdateReviewComments(_) => "UpdateReviewComments",
                Message::DebugOutput(_) => "DebugOutput",
                Message::ArtifactEvent(_) => "ArtifactEvent",
                Message::MessagesReceivedFromAgents(_) => "MessagesReceivedFromAgents",
                Message::EventsFromAgents(_) => "EventsFromAgents",
                Message::PassiveSuggestionResult(_) => "PassiveSuggestionResult",
                Message::CodeReview(_) => "CodeReview",
                Message::ServerEvent(_) => "ServerEvent",
                Message::InvokeSkill(_) => "InvokeSkill",
                Message::OrchestrationConfigSnapshot(_) => "OrchestrationConfigSnapshot",
            }
        }
    }

    impl Default for Message {
        fn default() -> Self { Message::UserQuery(()) }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct MessageData {
        pub id: String,
        pub timestamp: i64,
        pub message: Option<Message>,
        pub task_id: Option<String>,
        pub request_id: Option<String>,
        pub citations: Vec<super::Citation>,
        pub context: Option<String>,
        pub referenced_attachments: Vec<String>,
    }

    impl Default for MessageData {
        fn default() -> Self {
            Self {
                id: String::new(),
                timestamp: 0,
                message: None,
                task_id: None,
                request_id: None,
                citations: vec![],
                context: None,
                referenced_attachments: vec![],
            }
        }
    }

    impl MessageData {
        pub fn encode_to_vec(&self) -> Vec<u8> {
            vec![]
        }
    }

    pub mod artifact_event {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ArtifactEvent;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConversationArtifact {
            pub artifact: Option<conversation_artifact::Artifact>,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Event {
            Created(artifact_created::Created),
            ForkArtifacts(()),
        }
        pub mod artifact_created {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Created {
                pub artifact: Option<super::conversation_artifact::Artifact>,
            }
        }
        pub mod conversation_artifact {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Artifact {
                PullRequest(super::super::super::PullRequestArtifact),
                Screenshot(super::super::super::ScreenshotArtifact),
                File(super::super::super::FileArtifact),
                Plan(super::super::super::PlanArtifact),
            }
        }
    }

    pub mod update_todos {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum Operation { Create, Update, Delete }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct UpdateTodosData {
            pub operation: Option<Operation>,
        }
    }

    pub mod update_review_comments {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Operation { Create, Update, Delete }
    }

    pub mod summarization {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Summarization;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum SummaryType { ConversationSummary(()), ToolCallResultSummary(()) }
    }

    pub mod web_search {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct WebSearch;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Status;
    }

    pub mod web_fetch {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct WebFetch;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Status;
    }

    pub mod system_query {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct SystemQuery {
            pub r#type: Option<Type>,
            pub context: Option<String>,
        }
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            Init(()),
            Update(()),
            Other(()),
        }
    }

    pub mod agent_output {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct AgentOutput {
            pub plan_id: Option<String>,
            pub config: Option<String>,
            pub status: Option<String>,
            pub result: Option<String>,
        }
    }

    pub mod tool_call_result {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct ToolCallResult {
            pub tool_call_id: Option<String>,
            pub result: Option<String>,
        }
    }

    pub mod model_used {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct ModelUsed {
            pub model_id: Option<String>,
            pub model_display_name: Option<String>,
            pub is_fallback: bool,
        }
    }

    pub mod agent_reasoning {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct AgentReasoning;
    }

    pub mod debug_output {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DebugOutput;
    }

    pub mod events_from_agents {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EventsFromAgents;
    }

    pub mod passive_suggestion_result {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PassiveSuggestionResult;
    }

    pub mod code_review {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CodeReview;
    }

    pub mod server_event {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ServerEvent;
    }

    pub mod invoke_skill {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct InvokeSkill;
    }

    pub mod orchestration_config_snapshot {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct OrchestrationConfigSnapshot;
    }
}

pub mod ask_user_question_result {
    use serde::{Deserialize, Serialize};

    pub mod answer_item {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Answer { MultipleChoice(super::MultipleChoiceAnswer), Skipped(()) }
        pub type MultipleChoiceAnswer = super::MultipleChoiceAnswer;
    }

    pub type AskUserQuestionAnswer = answer_item::Answer;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnswerItem { pub question_id: String, pub answer: Option<AskUserQuestionAnswer> }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MultipleChoiceAnswer {
        pub selected_options: Vec<String>,
        pub other_text: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub answers: Vec<AnswerItem> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod apply_file_diffs_result {
    use serde::{Deserialize, Serialize};
    pub mod success {
        use serde::{Deserialize, Serialize};
        use super::super::FileContent;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdatedFileContent { pub file: Option<FileContent>, pub was_edited_by_user: bool }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeletedFile { pub file_path: String }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub updated_files_v2: Vec<success::UpdatedFileContent>, pub deleted_files: Vec<success::DeletedFile> }
    impl Default for Success {
        fn default() -> Self { Self { updated_files_v2: vec![], deleted_files: vec![] } }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod request_computer_use_result {
    use serde::{Deserialize, Serialize};
    pub mod approved {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Platform { Macos, Linux, Windows, LinuxX11, LinuxWayland }
    }
    impl From<approved::Platform> for i32 {
        fn from(_p: approved::Platform) -> i32 { 0 }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Approved {
        pub screen_dimensions: Option<super::ScreenDimensions>,
        pub initial_screenshot: Option<super::RawImage>,
        pub platform: i32,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Rejected;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Approved(Approved), Rejected(Rejected), Error(Error) }
}

pub mod mcp_resource_content {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TextContent;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlobContent;
    pub type Text = super::Text;
    pub type Binary = super::Binary;
    pub type ContentType = super::ContentType;
}

pub mod call_mcp_tool_result {
    use serde::{Deserialize, Serialize};
    pub mod success {
        use serde::{Deserialize, Serialize};
        pub mod result {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Text { pub text: String }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Image { pub data: Vec<u8>, pub mime_type: String }
            pub type Resource = super::super::super::McpResourceContent;
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Result { Text(Text), Image(Image), Resource(Resource) }
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Result { pub result: Option<result::Result> }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub results: Vec<success::Result> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod diff_hunk {}

pub mod diff_hunk_api {
    pub enum Current {
        CurrentBranchName(String),
        CurrentHeadlessCommitSha(String),
    }
    pub enum Base {
        BaseBranchName(String),
    }
}

// ============================================================================
// New modules
// ============================================================================

pub mod any_file_content {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Content { TextContent(super::FileContent), BinaryContent(super::BinaryFileContent) }
}

pub mod permission_denied {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Reason { DenylistedCommand(()), Default }
}

pub mod shell_command_error {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Type { CommandNotFound(()), Default }
}

pub mod run_shell_command_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result {
        CommandFinished(super::ShellCommandFinished),
        LongRunningCommandSnapshot(super::LongRunningShellCommandSnapshot),
        PermissionDenied(super::PermissionDenied),
    }
}

pub mod write_to_long_running_shell_command_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result {
        LongRunningCommandSnapshot(super::LongRunningShellCommandSnapshot),
        CommandFinished(super::ShellCommandFinished),
        Error(super::ShellCommandError),
    }
}

pub mod read_files_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub files: Vec<super::FileContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnyFilesSuccess { pub files: Vec<super::AnyFileContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), TextFilesSuccess(Success), AnyFilesSuccess(AnyFilesSuccess), Error(Error) }
}

pub mod upload_file_artifact_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub artifact_uid: String, pub mime_type: String, pub size_bytes: i64 }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod search_codebase_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Match;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub files: Vec<super::FileContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod suggest_new_conversation_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Accepted { pub message_id: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Rejected;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Accepted(Accepted), Rejected(Rejected), Error(Error) }
}

pub mod suggest_prompt_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Accepted;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Rejected;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Accepted(()), Rejected(()), Error(Error) }
}

pub mod grep_result {
    use serde::{Deserialize, Serialize};
    pub mod success {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Match { pub file_path: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrepFileMatch {
            pub file_path: String,
            pub matched_lines: Vec<grep_file_match::GrepLineMatch>,
        }
        pub mod grep_file_match {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Match { pub file_path: String }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct GrepLineMatch {
                pub line_number: u32,
            }
        }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Match;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub matched_files: Vec<success::GrepFileMatch> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod file_glob_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub matched_files: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod file_glob_v2_result {
    use serde::{Deserialize, Serialize};
    pub mod success {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Match { pub file_path: String }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileGlobMatch {
            pub file_path: String,
        }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub matched_files: Vec<success::FileGlobMatch>, pub warnings: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod read_mcp_resource_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub contents: Vec<super::McpResourceContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod read_skill_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub content: Option<super::FileContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod read_documents_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub documents: Vec<super::DocumentContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod edit_documents_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub updated_documents: Vec<super::DocumentContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod create_documents_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub created_documents: Vec<super::DocumentContent> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod use_computer_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub screenshot: Option<super::RawImage>, pub cursor_position: Option<super::Coordinates> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod fetch_conversation_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub directory_path: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod start_agent_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub agent_id: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub error: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod start_agent_v2_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub agent_id: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub error: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod send_message_to_agent_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success { pub message_id: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod run_agents_result {
    use serde::{Deserialize, Serialize};

    pub mod launched {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ResolvedExecutionMode {
            Local(super::super::run_agents::Local),
            Remote(super::super::run_agents::Remote),
        }
    }

    pub mod agent_outcome {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Result {
            Launched(super::LaunchedAgent),
            Failed(super::FailedAgent),
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LaunchedAgent { pub agent_id: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FailedAgent { pub error: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AgentOutcome { pub name: String, pub result: Option<agent_outcome::Result> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Launched {
        pub agents: Vec<AgentOutcome>,
        pub resolved_execution_mode: Option<launched::ResolvedExecutionMode>,
        pub resolved_harness: Option<super::Harness>,
        pub resolved_model_id: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Denied { pub reason: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Failure { pub error: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Outcome { Launched(Launched), Denied(Denied), Failure(Failure) }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod run_agents {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RunAgents;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Local;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Remote {
        pub environment_id: String,
        pub worker_host: String,
        pub computer_use_enabled: bool,
    }
}

pub mod insert_review_comments_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Result { Success(Success), Error(Error) }
}

pub mod read_shell_command_output_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone)]
    pub enum Result {
        Success(Success),
        CommandFinished(super::ShellCommandFinished),
        LongRunningCommandSnapshot(super::LongRunningShellCommandSnapshot),
        Error(super::ShellCommandError),
    }
}

pub mod transfer_shell_command_control_to_user_result {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Success;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Error { pub message: String }
    #[derive(Debug, Clone)]
    pub enum Result {
        Success(Success),
        CommandFinished(super::ShellCommandFinished),
        LongRunningCommandSnapshot(super::LongRunningShellCommandSnapshot),
        Error(super::ShellCommandError),
    }
}

pub mod harness {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Oz;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ClaudeCode;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OpenCode;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Gemini;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Codex;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Variant { Oz(Oz), ClaudeCode(ClaudeCode), OpenCode(OpenCode), Gemini(Gemini), Codex(Codex) }
}

pub mod orchestration_config {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Local;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Remote { pub environment_id: String, pub worker_host: String }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ExecutionMode { Local(Local), Remote(Remote) }
}

pub mod orchestration_status {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Approved;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Disapproved;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Status { Approved(Approved), Disapproved(Disapproved) }
}

pub mod skill_descriptor {
    use serde::{Deserialize, Serialize};
    pub mod scope {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Type { Home(()), Project(()), Bundled(()) }
    }
    pub mod provider {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Type { Warp(()), Agents(()), Claude(()), Codex(()), Cursor(()), Gemini(()), Copilot(()), Droid(()), Github(()), OpenCode(()) }
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SkillReference { Path(String), BundledSkillId(String) }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Scope { pub r#type: Option<scope::Type> }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Provider { pub r#type: Option<provider::Type> }
}

pub mod skill_ref {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SkillReference { Path(String), BundledSkillId(String) }
}

pub mod request {
    pub mod settings {
        pub mod api_keys {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct AwsCredentials {
                pub access_key: String,
                pub secret_key: String,
                pub session_token: String,
                pub region: String,
            }
        }
        pub mod custom_model_providers {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct CustomModelProvider {
                pub models: Vec<CustomModel>,
                pub api_key: String,
                pub base_url: String,
            }
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct CustomModel { pub config_key: String, pub slug: String }
        }
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CustomModelProviders { pub providers: Vec<custom_model_providers::CustomModelProvider> }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApiKeys {
            pub anthropic: String,
            pub openai: String,
            pub google: String,
            pub open_router: String,
            pub aws_credentials: Option<api_keys::AwsCredentials>,
            pub allow_use_of_warp_credits: bool,
        }
    }
    pub mod input {
        pub mod tool_call_result {
            #[derive(Debug, Clone)]
            pub enum Result {
                RunShellCommand(super::super::super::RunShellCommandResult),
                WriteToLongRunningShellCommand(super::super::super::WriteToLongRunningShellCommandResult),
                ReadFiles(super::super::super::ReadFilesResult),
                UploadFileArtifact(super::super::super::UploadFileArtifactResult),
                SearchCodebase(super::super::super::SearchCodebaseResult),
                ApplyFileDiffs(super::super::super::ApplyFileDiffsResult),
                SuggestNewConversation(super::super::super::SuggestNewConversationResult),
                SuggestPrompt(super::super::super::SuggestPromptResult),
                Grep(super::super::super::GrepResult),
                FileGlob(super::super::super::FileGlobResult),
                FileGlobV2(super::super::super::FileGlobV2Result),
                ReadMcpResource(super::super::super::ReadMcpResourceResult),
                CallMcpTool(super::super::super::CallMcpToolResult),
                ReadSkill(super::super::super::ReadSkillResult),
                ReadDocuments(super::super::super::ReadDocumentsResult),
                EditDocuments(super::super::super::EditDocumentsResult),
                CreateDocuments(super::super::super::CreateDocumentsResult),
                ReadShellCommandOutput(super::super::super::ReadShellCommandOutputResult),
                TransferShellCommandControlToUser(super::super::super::TransferShellCommandControlToUserResult),
                UseComputer(super::super::super::UseComputerResult),
                FetchConversation(super::super::super::FetchConversationResult),
                StartAgent(super::super::super::StartAgentResult),
                StartAgentV2(super::super::super::StartAgentV2Result),
                SendMessageToAgent(super::super::super::SendMessageToAgentResult),
                RunAgentsResult(super::super::super::RunAgentsResult),
                InsertReviewComments(super::super::super::InsertReviewCommentsResult),
                AskUserQuestion(super::super::super::AskUserQuestionResult),
                RequestComputerUse(super::super::super::RequestComputerUseResult),
            }
        }
        pub mod query_with_canned_response {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Type { Install, Code, Deploy, SomethingElse, CustomOnboardingRequest }
        }
        pub mod generate_passive_suggestions {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub enum Trigger { ShellCommandCompleted, AgentResponseCompleted }
        }
        pub mod user_inputs {
            pub mod user_input {
                use serde::{Deserialize, Serialize};
                #[derive(Debug, Clone, Serialize, Deserialize)]
                pub struct Input {
                    pub r#type: Option<String>,
                }
            }
        }
    }
    pub mod mcp_context {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct McpResource;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct McpTool;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct McpServer;
    }
}

pub mod ask_user_question {
    pub mod question {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum QuestionType { MultipleChoice(()) }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Question;
    }
}

pub mod start_agent_v2 {
    pub mod execution_mode {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Harness;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ExecutionMode { Local, Remote }
    }
}

pub mod start_agent {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ExecutionMode { Local, Remote }
}

pub mod agent_event {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AgentEvent;
    #[derive(Debug, Clone)]
    pub struct LifecycleEvent;

    pub mod lifecycle_event {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct InProgress;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Succeeded;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Failed;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Started;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Idle;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Restarted;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Cancelled;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Blocked;
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Errored;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Detail {
            InProgress(InProgress),
            Succeeded(Succeeded),
            Failed(Failed),
            Started(Started),
            Idle(Idle),
            Restarted(Restarted),
            Cancelled(Cancelled),
            Blocked(Blocked),
            Errored(Errored),
        }
    }
}

pub mod attachment {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Attachment;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Value {
        DiffSet(()),
        DiffHunk(()),
        DiffLine(()),
        File(()),
        ChangedFile(()),
        Repository(()),
        PullRequest(()),
        Other(()),
    }
}

pub mod drive_object {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ObjectPayload {
        Workflow(()),
        Notebook(()),
        GenericStringObject(()),
    }
}

pub mod user_query_mode {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum UserQueryMode { Normal, Agentic }
}

pub mod input {
    pub mod tool_call_result {
        #[derive(Debug, Clone)]
        pub enum Result {
            RunShellCommand(super::super::RunShellCommandResult),
            WriteToLongRunningShellCommand(super::super::WriteToLongRunningShellCommandResult),
            ReadFiles(super::super::ReadFilesResult),
            UploadFileArtifact(super::super::UploadFileArtifactResult),
            SearchCodebase(super::super::SearchCodebaseResult),
            ApplyFileDiffs(super::super::ApplyFileDiffsResult),
            SuggestNewConversation(super::super::SuggestNewConversationResult),
            SuggestPrompt(super::super::SuggestPromptResult),
            Grep(super::super::GrepResult),
            FileGlob(super::super::FileGlobResult),
            FileGlobV2(super::super::FileGlobV2Result),
            ReadMcpResource(super::super::ReadMcpResourceResult),
            CallMcpTool(super::super::CallMcpToolResult),
            ReadSkill(super::super::ReadSkillResult),
            ReadDocuments(super::super::ReadDocumentsResult),
            EditDocuments(super::super::EditDocumentsResult),
            CreateDocuments(super::super::CreateDocumentsResult),
            ReadShellCommandOutput(super::super::ReadShellCommandOutputResult),
            TransferShellCommandControlToUser(super::super::TransferShellCommandControlToUserResult),
            UseComputer(super::super::UseComputerResult),
            FetchConversation(super::super::FetchConversationResult),
            StartAgent(super::super::StartAgentResult),
            StartAgentV2(super::super::StartAgentV2Result),
            SendMessageToAgent(super::super::SendMessageToAgentResult),
            RunAgentsResult(super::super::RunAgentsResult),
            InsertReviewComments(super::super::InsertReviewCommentsResult),
            AskUserQuestion(super::super::AskUserQuestionResult),
            RequestComputerUse(super::super::RequestComputerUseResult),
        }
    }
    pub mod query_with_canned_response {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Type { Install, Code, Deploy, SomethingElse, CustomOnboardingRequest }
    }
    pub mod generate_passive_suggestions {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Trigger { ShellCommandCompleted, AgentResponseCompleted }
    }
    pub mod user_inputs {
        pub mod user_input {
            use serde::{Deserialize, Serialize};
            #[derive(Debug, Clone, Serialize, Deserialize)]
            pub struct Input {
                pub r#type: Option<String>,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub enum Type {
        UserQuery(()),
        ToolCallResult(tool_call_result::Result),
        Other(()),
    }
}

pub mod passive_suggestion_result_type {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PassiveSuggestionResultType;
}

pub mod diff_set {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DiffSet;
}

// ============================================================================
// Root-level types
// ============================================================================

#[derive(Debug, Clone)]
pub struct AgentEvent {
    pub event_id: String,
    pub occurred_at: String,
    pub event: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AgentType { Default }

#[derive(Debug, Clone)]
pub struct ConversationData {
    pub tasks: Vec<Task>,
}

impl ConversationData {
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleEventType { Unspecified, Default }

impl From<i32> for LifecycleEventType {
    fn from(_v: i32) -> Self { LifecycleEventType::Unspecified }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDependencies { pub parent_task_id: String }
impl Default for TaskDependencies {
    fn default() -> Self { Self { parent_task_id: String::new() } }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: String, pub title: String,
    pub description: String,
    pub server_data: String,
    pub dependencies: Option<TaskDependencies>,
    pub messages: Vec<message::Message>, pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus { Pending, InProgress, Completed, Failed }

impl Default for Task {
    fn default() -> Self {
        Self { id: String::new(), title: String::new(), description: String::new(), server_data: String::new(), dependencies: None, messages: vec![], status: TaskStatus::Pending }
    }
}

impl Task {
    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn server_data(&self) -> &str {
        &self.server_data
    }
}

#[derive(Debug, Clone)]
pub enum ToolType { Default }

#[derive(Debug, Clone)]
pub enum LlmProvider { Default }

// --- New root-level types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentType {
    Unknown, WarpDriveWorkflow, WarpDriveNotebook, WarpDriveEnvVar, Rule, WarpDocumentation, WebPage,
}
impl TryFrom<i32> for DocumentType {
    type Error = ();
    fn try_from(_value: i32) -> Result<Self, Self::Error> { Ok(DocumentType::Unknown) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text { pub content: String, pub mime_type: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Binary { pub data: Vec<u8>, pub mime_type: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType { Text(Text), Binary(Binary) }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Citation { pub document_type: i32, pub document_id: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates { pub x: i32, pub y: i32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContentLineRange { pub start: u32, pub end: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent { pub file_path: String, pub content: String, pub line_range: Option<FileContentLineRange> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFileContent { pub file_path: String, pub data: Vec<u8> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentContent { pub document_id: String, pub content: String, pub line_range: Option<FileContentLineRange> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongRunningShellCommandSnapshot {
    pub command_id: String, pub output: String, pub cursor: String,
    pub is_alt_screen_active: bool, pub is_preempted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceContent { pub uri: String, pub content_type: Option<ContentType> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawImage { pub data: Vec<u8>, pub mime_type: String, pub width: i32, pub height: i32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenDimensions { pub width_px: i32, pub height_px: i32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellCommandFinished {
    pub command_id: String, pub output: String, pub exit_code: i32,
    #[serde(skip)] pub start_ts: Option<prost_types::Timestamp>,
    #[serde(skip)] pub finish_ts: Option<prost_types::Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDescriptor {
    pub skill_reference: Option<skill_descriptor::SkillReference>,
    pub name: String, pub description: String,
    pub scope: Option<skill_descriptor::Scope>,
    pub provider: Option<skill_descriptor::Provider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill { pub descriptor: Option<SkillDescriptor>, pub content: Option<FileContent> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRef { pub skill_reference: Option<skill_ref::SkillReference> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyFileContent { pub content: Option<any_file_content::Content> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDenied { pub reason: Option<permission_denied::Reason> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellCommandError { pub r#type: Option<shell_command_error::Type> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunShellCommandResult {
    pub command: String, pub output: String, pub exit_code: i32,
    pub result: Option<run_shell_command_result::Result>,
}
impl Default for RunShellCommandResult {
    fn default() -> Self { Self { command: String::new(), output: String::new(), exit_code: 0, result: None } }
}

#[derive(Debug, Clone)]
pub struct WriteToLongRunningShellCommandResult { pub result: Option<write_to_long_running_shell_command_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFilesResult { pub result: Option<read_files_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileArtifactResult { pub result: Option<upload_file_artifact_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCodebaseResult { pub result: Option<search_codebase_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyFileDiffsResult { pub result: Option<apply_file_diffs_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestNewConversationResult { pub result: Option<suggest_new_conversation_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestPromptResult { pub result: Option<suggest_prompt_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrepResult { pub result: Option<grep_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileGlobResult { pub result: Option<file_glob_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileGlobV2Result { pub result: Option<file_glob_v2_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMcpResourceResult { pub result: Option<read_mcp_resource_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSkillResult { pub result: Option<read_skill_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadDocumentsResult { pub result: Option<read_documents_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditDocumentsResult { pub result: Option<edit_documents_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentsResult { pub result: Option<create_documents_result::Result> }

#[derive(Debug, Clone)]
pub struct UploadFileArtifact {
    pub file: Option<message::tool_call::FileRef>,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ReadShellCommandOutputResult { pub command: String, pub result: Option<read_shell_command_output_result::Result> }

#[derive(Debug, Clone)]
pub struct TransferShellCommandControlToUserResult { pub result: Option<transfer_shell_command_control_to_user_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseComputerResult { pub result: Option<use_computer_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestComputerUseResult { pub result: Option<request_computer_use_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchConversationResult { pub result: Option<fetch_conversation_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAgentResult { pub result: Option<start_agent_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAgentV2Result { pub result: Option<start_agent_v2_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageToAgentResult { pub result: Option<send_message_to_agent_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAgentsResult { pub outcome: Option<run_agents_result::Outcome> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertReviewCommentsResult { pub repo_path: String, pub result: Option<insert_review_comments_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallMcpToolResult { pub result: Option<call_mcp_tool_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskUserQuestionResult { pub result: Option<ask_user_question_result::Result> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    pub execution_mode: Option<orchestration_config::ExecutionMode>,
    pub harness: Option<Harness>,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationStatus { pub status: Option<orchestration_status::Status> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Harness { pub variant: Option<harness::Variant> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestArtifact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotArtifact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileArtifact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanArtifact;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags;

// --- Additional root-level types ---

#[derive(Debug, Clone)]
pub struct ResponseEvent {
    pub r#type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CurrentRef;

#[derive(Debug, Clone)]
pub struct BaseRef;

#[derive(Debug, Clone)]
pub struct Request {
    pub input: Option<request::input::user_inputs::user_input::Input>,
}

#[derive(Debug, Clone)]
pub struct ClientAction;

#[derive(Debug, Clone)]
pub struct UserQueryMode;

#[derive(Debug, Clone)]
pub struct OpenCodeReviewResult;

#[derive(Debug, Clone)]
pub struct InitProjectResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutonomyLevel { Unsupervised, Supervised }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel { Sandbox, None }

// ============================================================================
// prost::Message trait implementations
// ============================================================================

impl prost::Message for message::Message {
    fn encoded_len(&self) -> usize { 0 }
    fn encode_raw(&self, _buf: &mut impl prost::bytes::BufMut) {}
    fn encode(&self, _buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> { Ok(()) }
    fn encode_to_vec(&self) -> Vec<u8> { vec![] }
    fn encode_length_delimited(&self, _buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> { Ok(()) }
    fn encode_length_delimited_to_vec(&self) -> Vec<u8> { vec![] }
    fn decode(_buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> { Ok(message::Message::UserQuery(())) }
    fn merge_field(&mut self, _tag: u32, _wire_type: prost::encoding::WireType, _buf: &mut impl prost::bytes::Buf, _ctx: prost::encoding::DecodeContext) -> Result<(), prost::DecodeError> { Ok(()) }
    fn clear(&mut self) { *self = message::Message::UserQuery(()); }
}

impl prost::Message for Skill {
    fn encoded_len(&self) -> usize { 0 }
    fn encode_raw(&self, _buf: &mut impl prost::bytes::BufMut) {}
    fn encode(&self, _buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> { Ok(()) }
    fn encode_to_vec(&self) -> Vec<u8> { vec![] }
    fn encode_length_delimited(&self, _buf: &mut impl prost::bytes::BufMut) -> Result<(), prost::EncodeError> { Ok(()) }
    fn encode_length_delimited_to_vec(&self) -> Vec<u8> { vec![] }
    fn decode(_buf: impl prost::bytes::Buf) -> Result<Self, prost::DecodeError> { Ok(Skill { descriptor: None, content: None }) }
    fn merge_field(&mut self, _tag: u32, _wire_type: prost::encoding::WireType, _buf: &mut impl prost::bytes::Buf, _ctx: prost::encoding::DecodeContext) -> Result<(), prost::DecodeError> { Ok(()) }
    fn clear(&mut self) { self.descriptor = None; self.content = None; }
}

impl TryFrom<i32> for request_computer_use_result::approved::Platform {
    type Error = ();
    fn try_from(_v: i32) -> Result<Self, Self::Error> { Ok(request_computer_use_result::approved::Platform::Macos) }
}

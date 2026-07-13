//! Conversions from application types to MAA API types.

use ai::agent::convert::ConvertToAPITypeError;
use anyhow::anyhow;
use chrono::{DateTime, Local, Timelike};
use cute_multi_agent_api as api;

use crate::ai::agent::{
    AIAgentActionResult, AIAgentActionResultType, AIAgentAttachment, AIAgentContext, AIAgentInput,
    DriveObjectPayload, MCPContext, PassiveSuggestionResultType, PassiveSuggestionTrigger,
    RunningCommand, StaticQueryType, Suggestions, UserQueryMode,
};
use crate::ai::block_context::BlockContext;

fn local_datetime_to_timestamp(timestamp: DateTime<Local>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: timestamp.timestamp(),
        nanos: timestamp.timestamp_subsec_nanos() as i32,
    }
}

impl TryFrom<StaticQueryType> for api::request::input::query_with_canned_response::Type {
    type Error = ConvertToAPITypeError;

    fn try_from(value: StaticQueryType) -> Result<Self, Self::Error> {
        match value {
            StaticQueryType::Install => Ok(
                api::request::input::query_with_canned_response::Type::Install(
                    api::request::input::query_with_canned_response::Install {},
                ),
            ),
            StaticQueryType::Code => {
                Ok(api::request::input::query_with_canned_response::Type::Code(
                    api::request::input::query_with_canned_response::Code {},
                ))
            }
            StaticQueryType::Deploy => Ok(
                api::request::input::query_with_canned_response::Type::Deploy(
                    api::request::input::query_with_canned_response::Deploy {},
                ),
            ),
            StaticQueryType::SomethingElse => Ok(
                api::request::input::query_with_canned_response::Type::SomethingElse(
                    api::request::input::query_with_canned_response::SomethingElse {},
                ),
            ),
            StaticQueryType::CustomOnboardingRequest => Ok(
                api::request::input::query_with_canned_response::Type::CustomOnboardingRequest(
                    api::request::input::query_with_canned_response::CustomOnboardingRequest {},
                ),
            ),
            StaticQueryType::EvaluationSuite => {
                Err(anyhow::anyhow!("EvaluationSuite StaticQueryType not yet supported").into())
            }
        }
    }
}



impl From<PassiveSuggestionTrigger> for api::request::input::generate_passive_suggestions::Trigger {
    fn from(value: PassiveSuggestionTrigger) -> Self {
        match value {
            PassiveSuggestionTrigger::FilesChanged => {
                api::request::input::generate_passive_suggestions::Trigger::FilesChanged(())
            }
            PassiveSuggestionTrigger::CommandRun => {
                api::request::input::generate_passive_suggestions::Trigger::CommandRun(())
            }
            PassiveSuggestionTrigger::ShellCommandCompleted(shell_trigger) => {
                api::request::input::generate_passive_suggestions::Trigger::ShellCommandCompleted(
                    api::request::input::generate_passive_suggestions::ShellCommandCompleted {
                        executed_shell_command: Some(
                            (*shell_trigger.executed_shell_command).into(),
                        ),
                        relevant_files: shell_trigger
                            .relevant_files
                            .into_iter()
                            .flat_map(|file| Vec::<api::AnyFileContent>::from(file).into_iter())
                            .collect(),
                    },
                )
            }
            PassiveSuggestionTrigger::AgentResponseCompleted { .. } => {
                api::request::input::generate_passive_suggestions::Trigger::AgentResponseCompleted(
                    api::request::input::generate_passive_suggestions::AgentResponseCompleted {},
                )
            }
        }
    }
}

impl From<UserQueryMode> for cute_multi_agent_api::UserQueryMode {
    fn from(value: UserQueryMode) -> Self {
        match value {
            UserQueryMode::Normal => cute_multi_agent_api::UserQueryMode { r#type: None },
            UserQueryMode::Plan => cute_multi_agent_api::UserQueryMode {
                r#type: Some(cute_multi_agent_api::user_query_mode::Type::Plan(())),
            },
            UserQueryMode::Orchestrate => cute_multi_agent_api::UserQueryMode {
                r#type: Some(cute_multi_agent_api::user_query_mode::Type::Orchestrate(())),
            },
        }
    }
}

impl From<AIAgentAttachment> for api::Attachment {
    fn from(attachment: AIAgentAttachment) -> Self {
        match attachment {
            AIAgentAttachment::PlainText(text) => api::Attachment {
                value: Some(api::attachment::Value::PlainText(text)),
            },
            AIAgentAttachment::Block(block) => api::Attachment {
                value: Some(api::attachment::Value::ExecutedShellCommand(block.into())),
            },
            AIAgentAttachment::DriveObject { uid, payload } => api::Attachment {
                value: Some(api::attachment::Value::DriveObject(api::DriveObject {
                    uid,
                    object_payload: payload.map(|p| match p {
                        DriveObjectPayload::Workflow {
                            name,
                            description,
                            command,
                        } => api::drive_object::ObjectPayload::Workflow(api::Workflow {
                            name,
                            description,
                            command,
                        }),
                        DriveObjectPayload::Notebook { title, content } => {
                            api::drive_object::ObjectPayload::Notebook(api::Notebook {
                                title,
                                content,
                            })
                        }
                        DriveObjectPayload::GenericStringObject {
                            payload,
                            object_type,
                        } => api::drive_object::ObjectPayload::GenericStringObject(
                            api::GenericStringObject {
                                payload,
                                object_type,
                            },
                        ),
                    }),
                })),
            },
            #[allow(deprecated)]
            AIAgentAttachment::DiffHunk {
                file_path,
                line_range,
                diff_content,
                lines_added,
                lines_removed,
                current,
                base,
            } => api::Attachment {
                value: Some(api::attachment::Value::DiffHunk(api::DiffHunk {
                    file_path,
                    line_range: Some(api::FileContentLineRange {
                        start: line_range.start.as_usize() as u32,
                        end: line_range.end.as_usize() as u32,
                    }),
                    diff_content,
                    lines_added,
                    lines_removed,
                    current: current.map(Into::into),
                    base: Some(base.into()),
                })),
            },
            AIAgentAttachment::DocumentContent {
                document_id,
                content,
                line_range,
                // TODO: Add attachment source to API
                ..
            } => api::Attachment {
                value: Some(api::attachment::Value::DocumentContent(
                    api::DocumentContent {
                        document_id,
                        content,
                        line_range: line_range.map(|range| api::FileContentLineRange {
                            start: range.start.as_usize() as u32,
                            end: range.end.as_usize() as u32,
                        }),
                    },
                )),
            },
            AIAgentAttachment::DiffSet {
                file_diffs,
                current,
                base,
            } => api::Attachment {
                value: Some(api::attachment::Value::DiffSet(api::DiffSet {
                    hunks: file_diffs
                        .into_iter()
                        .flat_map(|(file_path, hunks)| {
                            hunks
                                .into_iter()
                                .map(move |hunk| hunk.convert_to_api(file_path.clone()))
                        })
                        .collect(),
                    curr_ref: current.map(Into::into),
                    base_ref: Some(base.into()),
                })),
            },
            AIAgentAttachment::FilePathReference { file_path, .. } => api::Attachment {
                value: Some(api::attachment::Value::FilePathReference(
                    api::FilePathReference { file_path },
                )),
            },
        }
    }
}

impl TryFrom<AIAgentActionResult> for api::request::input::user_inputs::user_input::Input {
    type Error = ConvertToAPITypeError;

    fn try_from(action_result: AIAgentActionResult) -> Result<Self, Self::Error> {
        let result = match action_result.result {
            AIAgentActionResultType::RequestCommandOutput(request_command_result) => {
                Some(request_command_result.try_into()?)
            }
            AIAgentActionResultType::WriteToLongRunningShellCommand(result) => {
                Some(result.try_into()?)
            }
            AIAgentActionResultType::ReadFiles(read_files_result) => {
                Some(read_files_result.try_into()?)
            }
            AIAgentActionResultType::UploadArtifact(upload_artifact_result) => {
                Some(upload_artifact_result.try_into()?)
            }
            AIAgentActionResultType::SearchCodebase(search_codebase_result) => {
                Some(search_codebase_result.try_into()?)
            }
            AIAgentActionResultType::RequestFileEdits(request_file_edits_result) => {
                Some(request_file_edits_result.try_into()?)
            }
            AIAgentActionResultType::Grep(grep_result) => Some(grep_result.try_into()?),
            AIAgentActionResultType::FileGlob(file_glob_result) => {
                Some(file_glob_result.try_into()?)
            }
            AIAgentActionResultType::FileGlobV2(file_glob_result) => {
                Some(file_glob_result.try_into()?)
            }
            AIAgentActionResultType::ReadMCPResource(read_mcp_resource_result) => {
                Some(read_mcp_resource_result.try_into()?)
            }
            AIAgentActionResultType::CallMCPTool(call_mcp_tool_result) => {
                Some(call_mcp_tool_result.try_into()?)
            }
            AIAgentActionResultType::ReadSkill(read_skill_result) => {
                Some(read_skill_result.try_into()?)
            }
            AIAgentActionResultType::SuggestNewConversation(suggest_new_conversation_result) => {
                Some(suggest_new_conversation_result.try_into()?)
            }
            AIAgentActionResultType::SuggestPrompt(suggest_prompt_result) => {
                Some(suggest_prompt_result.try_into()?)
            }
            AIAgentActionResultType::OpenCodeReview => Some(
                cute_multi_agent_api::request::input::tool_call_result::Result::OpenCodeReview(
                    cute_multi_agent_api::OpenCodeReviewResult {},
                ),
            ),
            AIAgentActionResultType::InsertReviewComments(insert_review_comments_result) => {
                Some(insert_review_comments_result.try_into()?)
            }
            AIAgentActionResultType::InitProject => Some(
                cute_multi_agent_api::request::input::tool_call_result::Result::InitProject(
                    cute_multi_agent_api::InitProjectResult {},
                ),
            ),
            AIAgentActionResultType::ReadDocuments(read_documents_result) => {
                Some(read_documents_result.try_into()?)
            }
            AIAgentActionResultType::EditDocuments(edit_documents_result) => {
                Some(edit_documents_result.try_into()?)
            }
            AIAgentActionResultType::CreateDocuments(create_documents_result) => {
                Some(create_documents_result.try_into()?)
            }
            AIAgentActionResultType::ReadShellCommandOutput(read_shell_command_output_result) => {
                Some(read_shell_command_output_result.try_into()?)
            }
            AIAgentActionResultType::UseComputer(use_computer_result) => {
                Some(use_computer_result.try_into()?)
            }
            AIAgentActionResultType::RequestComputerUse(request_computer_use_result) => {
                Some(request_computer_use_result.try_into()?)
            }
            AIAgentActionResultType::FetchConversation(fetch_conversation_result) => {
                Some(fetch_conversation_result.try_into()?)
            }
            AIAgentActionResultType::StartAgent(start_agent_result) => {
                Some(start_agent_result.into())
            }
            AIAgentActionResultType::SendMessageToAgent(send_message_result) => {
                Some(send_message_result.into())
            }
            AIAgentActionResultType::TransferShellCommandControlToUser(transfer_control_result) => {
                Some(transfer_control_result.try_into()?)
            }
            AIAgentActionResultType::AskUserQuestion(ask_user_question_result) => {
                Some(ask_user_question_result.into())
            }
            AIAgentActionResultType::RunAgents(orchestrate_result) => {
                Some(orchestrate_result.try_into()?)
            }
        };
        Ok(
            api::request::input::user_inputs::user_input::Input::ToolCallResult(
                api::request::input::ToolCallResult {
                    tool_call_id: action_result.id.into(),
                    result,
                },
            ),
        )
    }
}



impl From<Suggestions> for api::Suggestions {
    fn from(value: Suggestions) -> Self {
        Self {
            rules: value
                .rules
                .into_iter()
                .map(|rule| api::SuggestedRule {
                    name: rule.name,
                    content: rule.content,
                    logging_id: rule.logging_id.to_string(),
                })
                .collect(),
            workflows: value
                .agent_mode_workflows
                .into_iter()
                .map(|workflow| api::SuggestedAgentModeWorkflow {
                    name: workflow.name,
                    prompt: workflow.prompt,
                    logging_id: workflow.logging_id.to_string(),
                })
                .collect(),
        }
    }
}

// Convert rmcp resource to proto format.
fn convert_mcp_resource(resource: rmcp::model::Resource) -> api::request::mcp_context::McpResource {
    let rmcp::model::RawResource {
        uri,
        name,
        description,
        mime_type,
        ..
    } = resource.raw;
    api::request::mcp_context::McpResource {
        uri,
        name,
        description: description.unwrap_or_default(),
        mime_type: mime_type.unwrap_or_default(),
    }
}

// Convert rmcp tool to proto format, skipping tools with invalid schemas.
fn convert_mcp_tool(tool: rmcp::model::Tool) -> Option<api::request::mcp_context::McpTool> {
    let Ok(prost_types::Value {
        kind: Some(prost_types::value::Kind::StructValue(input_schema)),
    }) = serde_json_to_prost(tool.input_schema.as_ref().clone().into())
    else {
        return None;
    };

    Some(api::request::mcp_context::McpTool {
        name: tool.name.to_string(),
        description: tool.description.map(|d| d.to_string()).unwrap_or_default(),
        input_schema: Some(input_schema),
    })
}

impl From<MCPContext> for api::request::McpContext {
    #[allow(deprecated)]
    fn from(value: MCPContext) -> Self {
        // Check if we're using the old flat structure (no servers)
        // or the new grouped structure (servers populated)
        if value.servers.is_empty() {
            // Old behavior: use deprecated flat resources and tools lists
            api::request::McpContext {
                #[allow(deprecated)]
                resources: value
                    .resources
                    .into_iter()
                    .map(convert_mcp_resource)
                    .collect(),
                #[allow(deprecated)]
                tools: value
                    .tools
                    .into_iter()
                    .filter_map(convert_mcp_tool)
                    .collect(),
                servers: vec![], // Empty for old behavior
            }
        } else {
            // New behavior: group by server
            let servers: Vec<_> = value
                .servers
                .into_iter()
                .map(|server| api::request::mcp_context::McpServer {
                    id: server.id,
                    name: server.name,
                    description: server.description,
                    resources: server
                        .resources
                        .into_iter()
                        .map(convert_mcp_resource)
                        .collect(),
                    tools: server
                        .tools
                        .into_iter()
                        .filter_map(convert_mcp_tool)
                        .collect(),
                })
                .collect();

            api::request::McpContext {
                #[allow(deprecated)]
                resources: vec![], // Empty - everything is grouped by server
                #[allow(deprecated)]
                tools: vec![], // Empty - everything is grouped by server
                servers,
            }
        }
    }
}

impl From<BlockContext> for api::ExecutedShellCommand {
    fn from(block: BlockContext) -> Self {
        api::ExecutedShellCommand {
            command: block.command,
            output: block.output,
            exit_code: block.exit_code.value(),
            command_id: block.id.into(),
            is_auto_attached: block.is_auto_attached,
            started_ts: block.started_ts.map(local_datetime_to_timestamp),
            finished_ts: block.finished_ts.map(local_datetime_to_timestamp),
        }
    }
}

/// Tries to convert a [`serde_json::Value`] to a [`prost_types::Value`].
#[cfg_attr(target_family = "wasm", allow(dead_code))]
fn serde_json_to_prost(value: serde_json::Value) -> Result<prost_types::Value, String> {
    use std::collections::BTreeMap;

    use prost_types::value::Kind::*;
    use serde_json::Value::*;

    Ok(prost_types::Value {
        kind: Some(match value {
            Null => NullValue(0),
            Bool(v) => BoolValue(v),
            Number(n) => NumberValue(
                n.as_f64()
                    .ok_or_else(|| format!("float {n} is not valid JSON number"))?,
            ),
            String(s) => StringValue(s),
            Array(a) => ListValue(prost_types::ListValue {
                values: a
                    .into_iter()
                    .map(serde_json_to_prost)
                    .collect::<Result<Vec<_>, std::string::String>>()?,
            }),
            Object(v) => StructValue(prost_types::Struct {
                fields: v
                    .into_iter()
                    .map(|(k, v)| serde_json_to_prost(v).map(|v| (k, v)))
                    .collect::<Result<BTreeMap<_, _>, std::string::String>>()?,
            }),
        }),
    })
}

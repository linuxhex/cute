use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use parking_lot::Mutex;
use cute_cli::agent::Harness;
use cute_managed_secrets::ManagedSecretValue;
use cuteui::{ModelHandle, ModelSpawner};

use super::super::terminal::{CommandHandle, TerminalDriver};
use super::super::{AgentDriver, AgentDriverError};
use super::{
    HarnessCleanupDisposition, HarnessRunner, JSONMCPServer, ResumePayload,
    SavePoint, ThirdPartyHarness,
};
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent_sdk::setup_observability::{SetupClientEventReporter, SetupStep};
use crate::ai::ambient_agent_types::task::HarnessModelConfig;
use crate::ai::ambient_agent_types::AmbientAgentTaskId;
use crate::server::server_api::harness_support::HarnessSupportClient;
use crate::server::server_api::ServerApi;
use crate::terminal::model::block::BlockId;
use crate::terminal::CLIAgent;

pub(crate) struct OpenCodeHarness;

const OPENCODE_CLI_FORMAT: &str = "qoder_cli";
const OPENCODE_CLI_NAME: &str = "qoder";
const OPENCODE_EXIT_COMMAND: &str = "/exit";

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl ThirdPartyHarness for OpenCodeHarness {
    fn harness(&self) -> Harness {
        Harness::OpenCode
    }

    fn cli_agent(&self) -> CLIAgent {
        CLIAgent::Qoder
    }

    fn install_docs_url(&self) -> Option<&'static str> {
        Some("https://qoder.ai/docs")
    }

    fn build_runner(
        &self,
        prompt: &str,
        _system_prompt: Option<&str>,
        _resumption_prompt: Option<&str>,
        context: Option<&str>,
        working_dir: &Path,
        _task_id: Option<AmbientAgentTaskId>,
        server_api: Arc<ServerApi>,
        terminal_driver: ModelHandle<TerminalDriver>,
        _resume: Option<ResumePayload>,
        _resolved_env_vars: &HashMap<OsString, OsString>,
        _resolved_secrets: &HashMap<String, ManagedSecretValue>,
        _resolved_mcp_servers: &HashMap<String, JSONMCPServer>,
        _third_party_harness_model_config: Option<&HarnessModelConfig>,
    ) -> Result<Box<dyn HarnessRunner>, AgentDriverError> {
        let effective_prompt = match context {
            Some(ctx) if !ctx.is_empty() => format!("{ctx}\n\n{prompt}"),
            _ => prompt.to_string(),
        };
        let client: Arc<dyn HarnessSupportClient> = server_api;
        Ok(Box::new(OpenCodeHarnessRunner::new(
            OPENCODE_CLI_NAME,
            &effective_prompt,
            working_dir,
            client,
            terminal_driver,
        )?))
    }
}

fn opencode_command(cli_name: &str, prompt: &str) -> String {
    // Qoder CLI uses --print -p for non-interactive mode
    format!("{cli_name} --yolo --print -p '{prompt}'")
}

enum OpenCodeRunnerState {
    Preexec,
    Running {
        conversation_id: AIConversationId,
        block_id: BlockId,
    },
}

struct OpenCodeHarnessRunner {
    command: String,
    cli_name: String,
    client: Arc<dyn HarnessSupportClient>,
    terminal_driver: ModelHandle<TerminalDriver>,
    state: Mutex<OpenCodeRunnerState>,
}

impl OpenCodeHarnessRunner {
    fn new(
        cli_command: &str,
        prompt: &str,
        _working_dir: &Path,
        client: Arc<dyn HarnessSupportClient>,
        terminal_driver: ModelHandle<TerminalDriver>,
    ) -> Result<Self, AgentDriverError> {
        Ok(Self {
            command: opencode_command(cli_command, prompt),
            cli_name: cli_command.to_string(),
            client,
            terminal_driver,
            state: Mutex::new(OpenCodeRunnerState::Preexec),
        })
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl HarnessRunner for OpenCodeHarnessRunner {
    fn harness_name(&self) -> &str {
        &self.cli_name
    }

    async fn start(
        &self,
        foreground: &ModelSpawner<AgentDriver>,
        setup_events: &SetupClientEventReporter,
    ) -> Result<CommandHandle, AgentDriverError> {
        let conversation_id = setup_events
            .record_result(SetupStep::ThirdPartyHarnessExternalConversation, async {
                self.client
                    .create_external_conversation(OPENCODE_CLI_FORMAT)
                    .await
                    .map_err(|e| {
                        log::error!("Failed to create external conversation: {e}");
                        AgentDriverError::ConfigBuildFailed(e)
                    })
            })
            .await?;
        log::info!("Created external conversation {conversation_id}");

        let command = self.command.clone();
        let terminal_driver = self.terminal_driver.clone();
        let command_handle = foreground
            .spawn(move |_, ctx| {
                terminal_driver.update(ctx, |driver, ctx| driver.execute_command(&command, ctx))
            })
            .await??
            .await?;

        *self.state.lock() = OpenCodeRunnerState::Running {
            conversation_id,
            block_id: command_handle.block_id().clone(),
        };

        Ok(command_handle)
    }

    async fn exit(&self, foreground: &ModelSpawner<AgentDriver>) -> Result<()> {
        log::info!("Sending /exit to Qoder CLI");
        let terminal_driver = self.terminal_driver.clone();
        foreground
            .spawn(move |_, ctx| {
                terminal_driver.update(ctx, |driver, ctx| {
                    driver.send_text_to_cli(OPENCODE_EXIT_COMMAND.to_string(), ctx);
                });
            })
            .await
            .map_err(|_| anyhow::anyhow!("Agent driver dropped while sending /exit"))
    }

    async fn save_conversation(
        &self,
        save_point: SavePoint,
        foreground: &ModelSpawner<AgentDriver>,
    ) -> Result<()> {
        if matches!(save_point, SavePoint::Periodic)
            && !super::has_running_cli_agent(&self.terminal_driver, foreground).await
        {
            log::debug!("Will not save conversation, Qoder not in progress");
            return Ok(());
        }

        let (conversation_id, block_id) = match &*self.state.lock() {
            OpenCodeRunnerState::Preexec => {
                log::warn!("save_conversation called before start");
                return Ok(());
            }
            OpenCodeRunnerState::Running {
                conversation_id,
                block_id,
            } => (*conversation_id, block_id.clone()),
        };

        super::upload_current_block_snapshot(
            foreground,
            &self.terminal_driver,
            self.client.as_ref(),
            conversation_id,
            block_id,
        )
        .await
    }

    async fn cleanup(
        &self,
        _cleanup_disposition: HarnessCleanupDisposition,
        _foreground: &ModelSpawner<AgentDriver>,
    ) -> Result<()> {
        Ok(())
    }
}

impl fmt::Debug for OpenCodeHarnessRunner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenCodeHarnessRunner")
            .field("command", &self.command)
            .field("cli_name", &self.cli_name)
            .finish()
    }
}

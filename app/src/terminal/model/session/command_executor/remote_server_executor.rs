//! Stub for remote server command executor after remote_server removal.
//!
//! This module provides a stub implementation that panics on construction
//! since remote_server has been removed.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use warp_completer::completer::CommandOutput;
use warp_core::SessionId;

use super::{CommandExecutor, ExecuteCommandOptions};

/// Stub remote server command executor. Construction panics since remote_server has been removed.
pub struct RemoteServerCommandExecutor {
    _phantom: (),
}

impl RemoteServerCommandExecutor {
    /// Panics: remote_server has been removed.
    pub fn new(_session_id: SessionId, _client: ()) -> Self {
        unimplemented!("RemoteServerCommandExecutor::new: remote_server has been removed")
    }
}

impl std::fmt::Debug for RemoteServerCommandExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteServerCommandExecutor").finish()
    }
}

#[async_trait]
impl CommandExecutor for RemoteServerCommandExecutor {
    async fn execute_command(
        &self,
        _command: &str,
        _shell: &crate::terminal::shell::Shell,
        _current_directory_path: Option<&str>,
        _environment_variables: Option<HashMap<String, String>>,
        _execute_command_options: ExecuteCommandOptions,
    ) -> Result<CommandOutput> {
        unimplemented!("RemoteServerCommandExecutor::execute_command: remote_server has been removed")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn supports_parallel_command_execution(&self) -> bool {
        false
    }
}

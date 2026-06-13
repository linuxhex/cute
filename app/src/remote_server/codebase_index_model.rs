//! Stub for RemoteCodebaseIndexModel after remote_server removal.

use std::path::PathBuf;
use warp_core::HostId;
use warp_util::remote_path::RemotePath;
use warpui::{Entity, ModelContext, SingletonEntity};

/// Stub RemoteCodebaseIndexModel.
pub struct RemoteCodebaseIndexModel {
    _phantom: (),
}

/// Stub event enum for RemoteCodebaseIndexModel.
#[derive(Clone, Debug)]
pub enum RemoteCodebaseIndexModelEvent {
    IndexUpdated,
    IndexRemoved,
    SettingsEntriesChanged,
}

impl Entity for RemoteCodebaseIndexModel {
    type Event = RemoteCodebaseIndexModelEvent;
}

impl SingletonEntity for RemoteCodebaseIndexModel {}

/// Stub for agent context codebase info.
#[derive(Clone, Debug)]
pub struct AgentContextCodebase {
    pub name: String,
    pub path: String,
}

impl RemoteCodebaseIndexModel {
    pub fn entries_for_settings(&self) -> Vec<RemoteCodebaseIndexSettingsEntry> {
        Vec::new()
    }

    pub fn codebases_for_agent_context(&self, _host_id: HostId) -> Vec<AgentContextCodebase> {
        Vec::new()
    }

    pub fn active_repo_path(
        &self,
        _session_context: &crate::ai::blocklist::SessionContext,
        _requested_codebase_path: Option<&str>,
    ) -> Option<PathBuf> {
        None
    }

    pub fn active_repo_availability(
        &self,
        _session_context: &crate::ai::blocklist::SessionContext,
        _requested_codebase_path: Option<&str>,
    ) -> RemoteCodebaseSearchAvailability {
        RemoteCodebaseSearchAvailability::NoConnectedHost
    }

    pub fn request_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub fn resync_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    pub fn drop_index(
        &mut self,
        _remote_path: RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }
}

/// Stub RemoteCodebaseIndex.
pub struct RemoteCodebaseIndex {
    pub repo_path: PathBuf,
    pub host_id: HostId,
    pub state: RemoteCodebaseIndexState,
}

/// Stub RemoteCodebaseIndexState.
#[derive(Clone, Debug, Default)]
pub enum RemoteCodebaseIndexState {
    #[default]
    NotEnabled,
    Unavailable,
    Disabled,
    Queued,
    Indexing,
    Ready,
    Stale,
    Failed,
    NotIndexed,
    Indexed,
    Error,
}

/// Stub RemoteCodebaseIndexStatus.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexStatus {
    pub repo_path: String,
    pub state: RemoteCodebaseIndexState,
    pub progress: f32,
    pub status: Option<String>,
    pub progress_total: Option<u32>,
    pub progress_completed: Option<u32>,
    pub failure_message: Option<String>,
    pub last_updated_epoch_millis: Option<u64>,
    pub root_hash: Option<String>,
}

/// Stub RemoteCodebaseSearchAvailability.
#[derive(Clone, Debug)]
pub enum RemoteCodebaseSearchAvailability {
    Unavailable { remote_path: RemotePath, message: String },
    Ready(Box<RemoteCodebaseSearchContext>),
    NotIndexed { remote_path: RemotePath },
    Indexing { remote_path: RemotePath },
    NoConnectedHost,
    NoActiveRepo,
}

impl Default for RemoteCodebaseSearchAvailability {
    fn default() -> Self {
        Self::NoConnectedHost
    }
}

/// Stub RemoteCodebaseSearchContext.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseSearchContext {
    pub availability: RemoteCodebaseSearchAvailability,
    pub root_hash: String,
    pub remote_path: Option<RemotePath>,
    pub is_stale: bool,
}

/// Stub RemoteCodebaseIndexSettingsEntry.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexSettingsEntry {
    pub repo_path: String,
    pub state: RemoteCodebaseIndexState,
    pub remote_path: Option<warp_util::remote_path::RemotePath>,
    pub host_label: Option<String>,
    pub path: Option<warp_util::remote_path::RemotePath>,
    pub status: RemoteCodebaseIndexStatus,
}

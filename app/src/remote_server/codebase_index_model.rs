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
}

impl Entity for RemoteCodebaseIndexModel {
    type Event = RemoteCodebaseIndexModelEvent;
}

impl SingletonEntity for RemoteCodebaseIndexModel {}

impl RemoteCodebaseIndexModel {
    pub fn entries_for_settings(&self) -> Vec<RemoteCodebaseIndexSettingsEntry> {
        Vec::new()
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
    NotIndexed,
    Indexing,
    Indexed,
    Error,
}

/// Stub RemoteCodebaseIndexStatus.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexStatus {
    pub state: RemoteCodebaseIndexState,
    pub progress: f32,
}

/// Stub RemoteCodebaseSearchAvailability.
#[derive(Clone, Debug)]
pub enum RemoteCodebaseSearchAvailability {
    Unavailable { remote_path: RemotePath, message: String },
    Ready(RemoteCodebaseSearchContext),
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
    pub root_hash: Vec<u8>,
    pub remote_path: RemotePath,
    pub is_stale: bool,
}

/// Stub RemoteCodebaseIndexSettingsEntry.
#[derive(Clone, Debug, Default)]
pub struct RemoteCodebaseIndexSettingsEntry {
    pub repo_path: String,
    pub state: RemoteCodebaseIndexState,
    pub remote_path: Option<warp_util::remote_path::RemotePath>,
}

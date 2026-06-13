//! Stub for RemoteServerManager after remote_server removal.

use std::sync::Arc;
use warp_core::SessionId;
use warp_core::HostId;
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::remote_server::client::RemoteServerClient;
use crate::remote_server::setup::RemotePlatform;
use crate::terminal::event::RemoteServerSetupState;

/// Stub RemoteServerManager. All methods panic since remote_server has been removed.
pub struct RemoteServerManager {
    _phantom: (),
}

/// Stub event enum for RemoteServerManager.
#[derive(Clone, Debug)]
pub enum RemoteServerManagerEvent {
    SessionConnected {
        session_id: SessionId,
        host_id: HostId,
    },
    SessionDisconnected {
        session_id: SessionId,
        host_id: HostId,
        exit_status: Option<i32>,
        was_reconnect_attempt: bool,
    },
    SessionConnecting {
        session_id: SessionId,
        host_id: HostId,
    },
    SessionDeregistered {
        session_id: SessionId,
    },
    SessionConnectionFailed {
        session_id: SessionId,
        host_id: HostId,
        error: String,
    },
    SessionReconnected {
        session_id: SessionId,
        client: Arc<RemoteServerClient>,
        attempt: u32,
    },
    HostConnected {
        host_id: HostId,
    },
    HostDisconnected {
        host_id: HostId,
    },
    SetupStateChanged {
        session_id: SessionId,
        state: RemoteServerSetupState,
    },
    BufferUpdated {
        session_id: SessionId,
        path: String,
        content: Vec<u8>,
    },
    BufferConflictDetected {
        session_id: SessionId,
        path: String,
    },
    NavigatedToDirectory {
        session_id: SessionId,
        path: String,
    },
    RepoMetadataSnapshot {
        session_id: SessionId,
        repo_path: String,
    },
    RepoMetadataUpdated {
        session_id: SessionId,
        repo_path: String,
    },
    RepoMetadataDirectoryLoaded {
        session_id: SessionId,
        path: String,
    },
    CodebaseIndexStatusesSnapshot {
        session_id: SessionId,
    },
    CodebaseIndexStatusUpdated {
        session_id: SessionId,
        repo_path: String,
    },
    CodebaseIndexMutationFailed {
        session_id: SessionId,
        repo_path: String,
        error: String,
    },
    BinaryCheckComplete {
        session_id: SessionId,
        needs_install: bool,
    },
    BinaryInstallComplete {
        session_id: SessionId,
        success: bool,
    },
    ClientRequestFailed {
        session_id: SessionId,
        error: String,
    },
    ServerMessageDecodingError {
        session_id: SessionId,
        error: String,
    },
    DiffStateSnapshotReceived {
        session_id: SessionId,
    },
    DiffStateMetadataUpdateReceived {
        session_id: SessionId,
    },
    DiffStateFileDeltaReceived {
        session_id: SessionId,
    },
    GetBranchesResponse {
        session_id: SessionId,
    },
}

impl Entity for RemoteServerManager {
    type Event = RemoteServerManagerEvent;
}

impl SingletonEntity for RemoteServerManager {}

impl RemoteServerManager {
    pub fn client_for_session(&self, _session_id: SessionId) -> Option<Arc<RemoteServerClient>> {
        None
    }

    /// Returns the client for a given host, if any session is connected.
    pub fn client_for_host(&self, _host_id: &HostId) -> Option<&Arc<RemoteServerClient>> {
        None
    }

    /// Returns the platform info for a given session.
    pub fn platform_for_session(&self, _session_id: SessionId) -> Option<&RemotePlatform> {
        None
    }

    /// Stub: always returns false.
    pub fn is_session_potentially_active(&self, _session_id: SessionId) -> bool {
        false
    }

    /// Stub: returns async None.
    pub fn navigate_to_directory(
        &self,
        _session_id: SessionId,
        _path: String,
        _ctx: &mut ModelContext<Self>,
    ) -> impl std::future::Future<Output = Option<warp_util::remote_path::RemoteNavigationResult>> + Send + 'static {
        std::future::ready(None)
    }

    /// Stub: returns None.
    pub fn host_label(&self, _host_id: &HostId) -> Option<String> {
        None
    }

    /// Stub: returns false.
    pub fn trigger_codebase_incremental_sync(
        &self,
        _remote_path: warp_util::remote_path::RemotePath,
        _ctx: &mut ModelContext<Self>,
    ) -> bool {
        false
    }

    /// Stub: returns empty vec.
    pub fn sessions_for_host(&self, _host_id: &HostId) -> Vec<SessionId> {
        Vec::new()
    }

    /// Stub: no-op.
    pub fn load_remote_repo_metadata_directory(
        &self,
        _session_id: SessionId,
        _repo_root: String,
        _dir_path: String,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    /// Stub: returns None.
    pub fn host_id_for_session(&self, _session_id: SessionId) -> Option<HostId> {
        None
    }

    /// Stub: no-op.
    pub fn deregister_session(&self, _session_id: SessionId, _ctx: &mut ModelContext<Self>) {
    }
}

impl RemoteServerManagerEvent {
    pub fn session_id(&self) -> Option<SessionId> {
        match self {
            RemoteServerManagerEvent::SessionConnected { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::SessionDisconnected { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::SessionConnecting { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::SessionDeregistered { session_id } => Some(*session_id),
            RemoteServerManagerEvent::SessionConnectionFailed { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::SessionReconnected { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::SetupStateChanged { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::BufferUpdated { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::BufferConflictDetected { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::NavigatedToDirectory { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::RepoMetadataSnapshot { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::RepoMetadataUpdated { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::RepoMetadataDirectoryLoaded { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::CodebaseIndexStatusesSnapshot { session_id } => Some(*session_id),
            RemoteServerManagerEvent::CodebaseIndexStatusUpdated { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::CodebaseIndexMutationFailed { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::BinaryCheckComplete { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::BinaryInstallComplete { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::ClientRequestFailed { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::ServerMessageDecodingError { session_id, .. } => Some(*session_id),
            RemoteServerManagerEvent::DiffStateSnapshotReceived { session_id } => Some(*session_id),
            RemoteServerManagerEvent::DiffStateMetadataUpdateReceived { session_id } => Some(*session_id),
            RemoteServerManagerEvent::DiffStateFileDeltaReceived { session_id } => Some(*session_id),
            RemoteServerManagerEvent::GetBranchesResponse { session_id } => Some(*session_id),
            _ => None,
        }
    }
}

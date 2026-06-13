//! Stub for RemoteServerClient after remote_server removal.

use std::path::PathBuf;

use crate::remote_server::proto::{
    UploadHandoffSnapshotResponse,
    OpenBufferResponse, TextEdit,
};

/// Stub RemoteServerClient. All methods panic since remote_server has been removed.
#[derive(Clone, Debug)]
pub struct RemoteServerClient {
    _phantom: (),
}

impl RemoteServerClient {
    pub fn new() -> Self {
        Self { _phantom: () }
    }

    pub async fn upload_handoff_snapshot(
        &self,
        _paths: Vec<PathBuf>,
    ) -> anyhow::Result<UploadHandoffSnapshotResponse> {
        anyhow::bail!("remote_server has been removed")
    }

    pub async fn open_buffer(
        &self,
        _path: &str,
        _write: bool,
    ) -> anyhow::Result<OpenBufferResponse> {
        anyhow::bail!("remote_server has been removed")
    }

    pub fn send_buffer_edit(
        &self,
        _path: String,
        _expected_server_version: u64,
        _client_version: u64,
        _edits: Vec<TextEdit>,
    ) {
    }
}

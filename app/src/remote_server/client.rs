//! Stub for RemoteServerClient after remote_server removal.

use std::path::PathBuf;

use crate::remote_server::proto::{
    ReadFileContextRequest, ReadFileContextResponse, UploadHandoffSnapshotResponse,
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

    pub async fn read_file_context(
        &self,
        _request: ReadFileContextRequest,
    ) -> anyhow::Result<ReadFileContextResponse> {
        anyhow::bail!("remote_server has been removed")
    }

    pub async fn upload_handoff_snapshot(
        &self,
        _paths: Vec<PathBuf>,
    ) -> anyhow::Result<UploadHandoffSnapshotResponse> {
        anyhow::bail!("remote_server has been removed")
    }

    pub async fn get_fragment_metadata_from_hash(
        &self,
        _repo_path: PathBuf,
        _root_hash: String,
        _hashes: Vec<String>,
    ) -> anyhow::Result<GetFragmentMetadataFromHashResponse> {
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

    pub async fn save_buffer(
        &self,
        _path: String,
    ) -> anyhow::Result<()> {
        anyhow::bail!("remote_server has been removed")
    }
}

/// Stub response for get_fragment_metadata_from_hash.
#[derive(Clone, Debug)]
pub struct GetFragmentMetadataFromHashResponse {
    pub missing_hashes: Vec<String>,
    pub fragments: Vec<crate::remote_server::proto::FragmentMetadata>,
}

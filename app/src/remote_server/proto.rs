//! Stub for remote server proto types after remote_server removal.

use serde::{Deserialize, Serialize};

/// Stub enum for run command response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RunCommandErrorCode {
    Success,
    Error,
}

/// Stub module for run_command_response.
pub mod run_command_response {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Response {
        pub exit_code: i32,
        pub output: Vec<u8>,
    }
}

/// Stub for UploadHandoffSnapshotResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadHandoffSnapshotResponse {
    pub success: bool,
    pub initial_snapshot_token: Option<String>,
    pub error: Option<String>,
}

/// Stub for ReadFileContextRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileContextRequest {
    pub files: Vec<ReadFileContextFile>,
    pub max_file_bytes: Option<u32>,
    pub max_batch_bytes: Option<u32>,
}

/// Stub for ReadFileContextResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileContextResponse {
    pub file_contexts: Vec<FileContextProto>,
    pub failed_files: Vec<FailedFile>,
}

/// Stub for FailedFile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FailedFile {
    pub path: String,
    pub error: Option<FileOperationError>,
}

/// Stub for ReadFileContextFile.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReadFileContextFile {
    pub path: String,
    #[serde(default)]
    pub line_range: Option<LineRange>,
    #[serde(default)]
    pub line_ranges: Vec<LineRange>,
}

impl ReadFileContextFile {
    pub fn new(path: String) -> Self {
        Self {
            path,
            line_range: None,
            line_ranges: Vec::new(),
        }
    }
}

/// Stub for LineRange.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineRange {
    pub start: u32,
    pub end: u32,
}

/// Stub for FragmentMetadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FragmentMetadata {
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub byte_start: u64,
    pub byte_end: u64,
    pub content_hash: Vec<u8>,
}

/// Stub for FileContextProto.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileContextProto {
    pub path: String,
    pub content: Option<file_context_proto::Content>,
    pub file_name: String,
    pub line_range_start: Option<u32>,
    pub line_range_end: Option<u32>,
    pub line_count: u32,
    pub last_modified_epoch_millis: Option<u64>,
}

/// Stub module for file_context_proto.
pub mod file_context_proto {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FileContext {
        pub path: String,
        pub content: Vec<u8>,
    }

    /// Stub for Content type - enum with text and binary variants.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Content {
        TextContent(String),
        BinaryContent(Vec<u8>),
    }
}

/// Stub for TextEdit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEdit {
    pub start_offset: u64,
    pub end_offset: u64,
    pub new_text: String,
}

/// Stub module for open_buffer_response.
pub mod open_buffer_response {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum Result {
        Success(OpenBufferSuccess),
        Error(FileOperationError),
    }
}

/// Stub for OpenBufferResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenBufferResponse {
    pub result: open_buffer_response::Result,
}

/// Stub for OpenBufferSuccess.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenBufferSuccess {
    pub content: String,
    pub version: u64,
}

/// Stub for FileOperationError.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileOperationError {
    pub message: String,
}

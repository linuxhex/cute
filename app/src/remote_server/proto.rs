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
}

/// Stub for ReadFileContextRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileContextRequest {
    pub files: Vec<ReadFileContextFile>,
}

/// Stub for ReadFileContextResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileContextResponse {
    pub file_contexts: Vec<FileContextProto>,
}

/// Stub for ReadFileContextFile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileContextFile {
    pub path: String,
    pub line_range: Option<LineRange>,
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
    pub location: String,
    pub content_hash: Vec<u8>,
}

/// Stub for FileContextProto.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileContextProto {
    pub path: String,
    pub content: Vec<u8>,
}

/// Stub module for file_context_proto.
pub mod file_context_proto {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FileContext {
        pub path: String,
        pub content: Vec<u8>,
    }

    /// Stub for Content type.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Content {
        pub data: Vec<u8>,
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

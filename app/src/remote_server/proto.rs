//! Stub for remote server proto types after remote_server removal.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Stub for UploadHandoffSnapshotResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadHandoffSnapshotResponse {
    pub success: bool,
    pub initial_snapshot_token: Option<String>,
    pub error: Option<String>,
}

/// Stub for TextEdit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextEdit {
    pub start_offset: u64,
    pub end_offset: u64,
    pub new_text: String,
    pub text: String,
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

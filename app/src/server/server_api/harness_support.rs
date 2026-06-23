// We don't directly run agent harnesses on WASM, so this code is unused.
#![cfg_attr(target_family = "wasm", expect(dead_code))]

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg(feature = "local_fs")]
pub use super::presigned_upload::FileUploadBody;
pub use super::presigned_upload::UploadBody;
use super::ServerApi;
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::artifacts::Artifact;

/// A presigned upload target returned by the server.
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UploadTarget {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    pub fields: Vec<UploadField>,
}

/// A single multipart form field on a POST upload target.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UploadField {
    pub name: String,
    pub value: UploadFieldValue,
}

/// Descriptor for a field value when uploading to an [`UploadTarget`].
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UploadFieldValue {
    Static { value: String },
    ContentCrc32C,
    ContentData,
}

/// Request body for upload-snapshot upload targets.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotUploadRequest {
    pub files: Vec<SnapshotFileInfo>,
}

/// Describes a single file in a snapshot upload request.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SnapshotFileInfo {
    pub filename: String,
    pub mime_type: String,
}

/// Response from the upload-snapshot endpoint.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SnapshotUploadResponse {
    pub uploads: Vec<UploadTarget>,
}

#[derive(serde::Serialize)]
struct CreateExternalConversationRequest {
    format: String,
}

#[derive(serde::Deserialize)]
struct CreateExternalConversationResponse {
    conversation_id: String,
}

#[derive(serde::Serialize)]
struct GetUploadTargetRequest {
    conversation_id: String,
}

/// Skill attached to a resolve-prompt request.
#[derive(serde::Serialize)]
pub struct ResolvePromptAttachedSkill {
    pub name: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ResolvePromptRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<ResolvePromptAttachedSkill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_dir: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ResolvedHarnessPrompt {
    pub prompt: String,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub resumption_prompt: Option<String>,
    #[serde(default)]
    pub context: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ReportArtifactResponse {
    pub artifact_uid: String,
}

#[derive(serde::Serialize)]
struct NotifyUserRequest {
    message: String,
}

#[derive(serde::Serialize)]
struct FinishTaskRequest {
    success: bool,
    summary: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct ShutdownError {
    category: String,
    message: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ReportShutdownRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ShutdownError>,
}

impl ReportShutdownRequest {
    pub fn clean() -> Self {
        Self { error: None }
    }

    pub fn abnormal(category: String, message: String) -> Self {
        Self {
            error: Some(ShutdownError { category, message }),
        }
    }
}

/// Trait for API endpoints used to support third-party agent harnesses.
#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait HarnessSupportClient: 'static + Send + Sync {
    async fn create_external_conversation(&self, format: &str) -> Result<AIConversationId>;

    async fn get_transcript_upload_target(
        &self,
        conversation_id: &AIConversationId,
    ) -> Result<UploadTarget>;

    async fn get_block_snapshot_upload_target(
        &self,
        conversation_id: &AIConversationId,
    ) -> Result<UploadTarget>;

    async fn resolve_prompt(&self, request: ResolvePromptRequest) -> Result<ResolvedHarnessPrompt>;

    async fn report_artifact(&self, artifact: &Artifact) -> Result<ReportArtifactResponse>;

    async fn notify_user(&self, message: &str) -> Result<()>;

    async fn finish_task(&self, success: bool, summary: &str) -> Result<()>;

    async fn report_clean_shutdown(&self) -> Result<()>;

    async fn report_error_shutdown(
        &self,
        error_category: String,
        error_message: String,
    ) -> Result<()>;

    async fn get_snapshot_upload_targets(
        &self,
        request: &SnapshotUploadRequest,
    ) -> Result<Vec<UploadTarget>>;

    async fn fetch_transcript(&self) -> Result<bytes::Bytes>;

    fn http_client(&self) -> &http_client::Client;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl HarnessSupportClient for ServerApi {
    async fn create_external_conversation(&self, _format: &str) -> Result<AIConversationId> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn get_transcript_upload_target(
        &self,
        _conversation_id: &AIConversationId,
    ) -> Result<UploadTarget> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn get_block_snapshot_upload_target(
        &self,
        _conversation_id: &AIConversationId,
    ) -> Result<UploadTarget> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn resolve_prompt(&self, _request: ResolvePromptRequest) -> Result<ResolvedHarnessPrompt> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn report_artifact(&self, _artifact: &Artifact) -> Result<ReportArtifactResponse> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn notify_user(&self, _message: &str) -> Result<()> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn finish_task(&self, _success: bool, _summary: &str) -> Result<()> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn report_clean_shutdown(&self) -> Result<()> {
        // No-op for local version
        Ok(())
    }

    async fn report_error_shutdown(
        &self,
        _error_category: String,
        _error_message: String,
    ) -> Result<()> {
        // No-op for local version
        Ok(())
    }

    async fn get_snapshot_upload_targets(
        &self,
        _request: &SnapshotUploadRequest,
    ) -> Result<Vec<UploadTarget>> {
        Err(anyhow!("Harness support not available in local version"))
    }

    async fn fetch_transcript(&self) -> Result<bytes::Bytes> {
        Err(anyhow!("Harness support not available in local version"))
    }

    fn http_client(&self) -> &http_client::Client {
        &self.client
    }
}

/// Upload a blob to a presigned upload target.
pub async fn upload_to_target(
    http_client: &http_client::Client,
    target: &UploadTarget,
    body: impl UploadBody,
) -> Result<()> {
    super::presigned_upload::upload_to_target(http_client, target, body).await
}

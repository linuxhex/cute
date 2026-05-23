// We don't directly run agent harnesses on WASM, so this code is unused.
#![cfg_attr(target_family = "wasm", expect(dead_code))]

use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[cfg(feature = "local_fs")]
pub use super::presigned_upload::FileUploadBody;
pub use super::presigned_upload::UploadBody;
use super::ServerApi;
use crate::server::server_api::auth::AuthClient;

/// A presigned upload target returned by the server.
#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UploadTarget {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    /// Ordered multipart form fields for POST uploads.
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
/// This is currently only used for `POST` requests, but may be supported
/// for HTTP headers in the future.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UploadFieldValue {
    /// Literal string value known at URL-generation time.
    Static { value: String },
    /// Client should compute CRC32C of the upload, base64-encode the 4-byte
    /// big-endian result, and send it as this field's value.
    ContentCrc32C,
    /// Client should use the raw upload bytes as this field's value.
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
///
/// The `uploads` list is aligned by index with the [`SnapshotUploadRequest::files`]
/// list in the request, so callers match each upload target back to the filename
/// they requested by position. The server does not include filenames on the
/// response entries — see the `UploadSnapshotResponse` schema in
/// `warp-server`'s `public_api/openapi.yaml`.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct SnapshotUploadResponse {
    pub uploads: Vec<UploadTarget>,
}

/// Skill attached to a resolve-prompt request,
/// used when invoking a third-party harness with a skill
/// via the CLI.
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
    /// Optional user-turn preamble for resumed third-party harness sessions. The harness
    /// decides how to surface this — Claude Code prepends it to the user-turn prompt fed
    /// into the CLI so the agent treats it as immediate intent rather than background
    /// system context. Empty when no resumption is in effect.
    #[serde(default)]
    pub resumption_prompt: Option<String>,
    /// Optional server-retrieved context relevant to the task prompt. Each harness
    /// decides how to inject this — typically by prepending it to the user-turn prompt
    /// after any resumption preamble.
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
    /// A clean shutdown with no error payload.
    pub fn clean() -> Self {
        Self { error: None }
    }

    /// An abnormal shutdown carrying an error category and message.
    pub fn abnormal(category: String, message: String) -> Self {
        Self {
            error: Some(ShutdownError { category, message }),
        }
    }
}

/// Trait for API endpoints used to support third-party agent harnesses in Oz.
/// Note: AI-related methods have been removed.
#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait HarnessSupportClient: 'static + Send + Sync {
    /// Resolve the prompt for a third-party harness run for a task stored on the server.
    async fn resolve_prompt(&self, request: ResolvePromptRequest) -> Result<ResolvedHarnessPrompt>;

    /// Send a progress notification to the task's originating platform.
    async fn notify_user(&self, message: &str) -> Result<()>;

    /// Report task completion or failure. The server derives PR links/branches from
    /// artifacts already reported via `report_artifact`.
    async fn finish_task(&self, success: bool, summary: &str) -> Result<()>;

    /// Report a clean shutdown of the agent process.
    async fn report_clean_shutdown(&self) -> Result<()>;

    /// Report an error shutdown of the agent process.
    async fn report_error_shutdown(
        &self,
        error_category: String,
        error_message: String,
    ) -> Result<()>;

    /// Get presigned upload targets for a workspace state snapshot.
    ///
    /// The returned list is aligned by index with `request.files`. See
    /// [`SnapshotUploadResponse`] for details on the server contract.
    async fn get_snapshot_upload_targets(
        &self,
        request: &SnapshotUploadRequest,
    ) -> Result<Vec<UploadTarget>>;

    /// Get an HTTP client to use with [`UploadTarget`]s for saving blobs.
    fn http_client(&self) -> &http_client::Client;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl HarnessSupportClient for ServerApi {
    async fn resolve_prompt(&self, request: ResolvePromptRequest) -> Result<ResolvedHarnessPrompt> {
        self.post_public_api("harness-support/resolve-prompt", &request)
            .await
    }

    async fn notify_user(&self, message: &str) -> Result<()> {
        self.post_public_api_unit(
            "harness-support/notify-user",
            &NotifyUserRequest {
                message: message.to_string(),
            },
        )
        .await
    }

    async fn finish_task(&self, success: bool, summary: &str) -> Result<()> {
        self.post_public_api_unit(
            "harness-support/finish-task",
            &FinishTaskRequest {
                success,
                summary: summary.to_string(),
            },
        )
        .await
    }

    async fn report_clean_shutdown(&self) -> Result<()> {
        self.post_public_api_unit(
            "harness-support/report-shutdown",
            &ReportShutdownRequest::clean(),
        )
        .await
    }

    async fn report_error_shutdown(
        &self,
        error_category: String,
        error_message: String,
    ) -> Result<()> {
        self.post_public_api_unit(
            "harness-support/report-shutdown",
            &ReportShutdownRequest::abnormal(error_category, error_message),
        )
        .await
    }

    async fn get_snapshot_upload_targets(
        &self,
        request: &SnapshotUploadRequest,
    ) -> Result<Vec<UploadTarget>> {
        let response: SnapshotUploadResponse = self
            .post_public_api("harness-support/upload-snapshot", request)
            .await?;
        Ok(response.uploads)
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

#[cfg(test)]
#[path = "harness_support_tests.rs"]
mod tests;

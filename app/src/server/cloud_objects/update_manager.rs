// Re-export InitiatedBy from sync_queue for backwards compatibility
pub use crate::server::sync_queue::InitiatedBy;

use std::future::Future;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use cuteui::{AppContext, Entity, ModelContext, SingletonEntity};
use crate::persistence::ModelEvent;

use crate::ai::ambient_agent_types::AmbientAgentTaskId;
use crate::ai::cloud_environments::{
    AmbientAgentEnvironment, CloudAmbientAgentEnvironmentModel,
};
use crate::ai::execution_profiles::AIExecutionProfile;
use crate::ai::facts::AIFact;
use crate::ai::mcp::gallery::GalleryMCPServer;
use crate::ai::mcp::templatable::TemplatableMCPServer;
use crate::cloud_stub_types::{CloudObjectEventEntrypoint, Owner};
use crate::cloud_stub_types::models::notebook::CloudNotebookModel;
use crate::cloud_stub_types::models::env_vars::{CloudEnvVarCollectionModel, EnvVarCollection};
use cute_server_client::cloud_object::Revision;
use crate::cloud_stub_types::CloudObjectTypeAndId;
use crate::server::ids::{ClientId, ServerId, SyncId};

/// Options for fetching a single object
#[derive(Debug, Clone)]
pub enum FetchSingleObjectOption {
    None,
}

/// Operation type for cloud objects
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectOperation {
    Create { initiated_by: InitiatedBy },
    Update,
    Delete { initiated_by: InitiatedBy },
    Trash,
    Untrash,
    MoveToFolder,
    MoveToDrive,
    Leave,
    TakeEditAccess,
    UpdatePermissions,
    EmptyTrash,
}

/// Success type for operations
#[derive(Debug, Clone, PartialEq)]
pub enum OperationSuccessType {
    Success,
    Failure,
    Rejection,
    FeatureNotAvailable,
    Denied(String),
}

/// Result of an object operation
#[derive(Debug, Clone)]
pub struct ObjectOperationResult {
    pub operation: ObjectOperation,
    pub success_type: OperationSuccessType,
    pub client_id: Option<ClientId>,
    pub server_id: Option<ServerId>,
    pub num_objects: Option<usize>,
}

/// Event emitted by UpdateManager
#[derive(Debug, Clone)]
pub enum UpdateManagerEvent {
    ObjectOperationComplete { result: ObjectOperationResult },
    AmbientTaskUpdated { task_id: AmbientAgentTaskId, timestamp: DateTime<Utc> },
    MCPGalleryUpdated { templates: Vec<GalleryMCPServer> },
    CloudPreferencesUpdated,
}

/// UpdateManager for cloud objects (stub for local version)
#[derive(Debug)]
pub struct UpdateManager;

impl Entity for UpdateManager {
    type Event = UpdateManagerEvent;
}

impl SingletonEntity for UpdateManager {}

impl UpdateManager {
    pub fn new(
        _model_event_sender: Option<SyncSender<ModelEvent>>,
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _ctx: &mut AppContext,
    ) -> Self {
        Self
    }

    pub fn initial_load_complete(&self) -> bool {
        // 本地模式无云端对象同步，立即视为加载完成，避免 UI 等待。
        cfg!(feature = "skip_login")
    }

    /// Creates a new templatable MCP server.
    pub fn create_templatable_mcp_server(
        &mut self,
        _templatable_mcp_server: TemplatableMCPServer,
        _client_id: ClientId,
        _owner: String,
        _initiated_by: InitiatedBy,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates an existing templatable MCP server.
    pub fn update_templatable_mcp_server(
        &mut self,
        _templatable_mcp_server: TemplatableMCPServer,
        _id: SyncId,
        _revision: String,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Deletes a cloud object by user.
    pub fn delete_object_by_user(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Deletes a cloud object with initiated_by.
    pub fn delete_object_with_initiated_by(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _initiated_by: InitiatedBy,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Moves a cloud object to a location.
    pub fn move_object_to_location(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _destination_folder_id: Option<ServerId>,
        _space: crate::cloud_stub_types::Space,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new AI execution profile.
    pub fn create_ai_execution_profile(
        &mut self,
        _profile: AIExecutionProfile,
        _client_id: ClientId,
        _owner: Owner,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new AI fact.
    pub fn create_ai_fact(
        &mut self,
        _ai_fact: AIFact,
        _client_id: ClientId,
        _owner: Owner,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new folder.
    pub fn create_folder(
        &mut self,
        _name: String,
        _owner: Owner,
        _client_id: ClientId,
        _parent_folder_id: Option<ServerId>,
        _bool_param: bool,
        _initiated_by: InitiatedBy,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new notebook.
    pub fn create_notebook(
        &mut self,
        _title: String,
        _parent_folder_id: Option<ServerId>,
        _client_id: ClientId,
        _owner: String,
        _initiated_by: InitiatedBy,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new notebook with full model (for AI documents and integration testing).
    pub fn create_notebook_with_model(
        &mut self,
        _client_id: ClientId,
        _owner: Owner,
        _parent_folder_id: Option<SyncId>,
        _notebook_model: CloudNotebookModel,
        _entrypoint: CloudObjectEventEntrypoint,
        _bool_param: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Deletes an AI execution profile.
    pub fn delete_ai_execution_profile(
        &mut self,
        _profile_id: SyncId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates an existing AI execution profile.
    pub fn update_ai_execution_profile(
        &mut self,
        _profile: AIExecutionProfile,
        _id: SyncId,
        _revision: Option<Revision>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates an existing AI fact.
    pub fn update_ai_fact(
        &mut self,
        _ai_fact: AIFact,
        _id: SyncId,
        _revision: Option<Revision>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates notebook data.
    pub fn update_notebook_data(
        &mut self,
        _notebook_data: Vec<u8>,
        _id: SyncId,
        _revision: Option<Revision>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Records an object action (e.g., execution of a workflow).
    pub fn record_object_action(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _action_type: crate::server::server_api::object::ObjectActionType,
        _data: Option<String>,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new ambient agent environment.
    pub fn create_ambient_agent_environment(
        &mut self,
        _environment: AmbientAgentEnvironment,
        _client_id: ClientId,
        _owner: Owner,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Creates a new ambient agent environment online.
    /// Returns a Future that resolves to the ServerId of the created environment.
    pub fn create_ambient_agent_environment_online(
        &mut self,
        _environment: CloudAmbientAgentEnvironmentModel,
        _client_id: ClientId,
        _owner: Owner,
        _ctx: &mut ModelContext<Self>,
    ) -> impl Future<Output = anyhow::Result<ServerId>> {
        // Stub implementation - returns error for local version
        std::future::ready(Err(anyhow::anyhow!("Not implemented in local version")))
    }

    /// Updates an existing cloud object.
    pub fn update_object<K, M>(
        &mut self,
        _model: M,
        _id: SyncId,
        _revision: String,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
        // K and M type parameters are kept for API compatibility but not used in stub
    }

    /// Creates a new environment variable collection.
    pub fn create_env_var_collection(
        &mut self,
        _client_id: ClientId,
        _owner: Owner,
        _parent_folder_id: Option<SyncId>,
        _env_var_collection_model: CloudEnvVarCollectionModel,
        _entrypoint: CloudObjectEventEntrypoint,
        _bool_param: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates an existing environment variable collection.
    pub fn update_env_var_collection(
        &mut self,
        _env_var_collection: EnvVarCollection,
        _id: SyncId,
        _revision: String,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Gives up notebook edit access.
    pub fn give_up_notebook_edit_access(
        &mut self,
        _id: SyncId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Grabs notebook edit access.
    pub fn grab_notebook_edit_access(
        &mut self,
        _id: SyncId,
        _optimistically_grant_access: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Duplicates a cloud object.
    pub fn duplicate_object(
        &mut self,
        _cloud_object_type_and_id: &CloudObjectTypeAndId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Trashes a cloud object.
    pub fn trash_object(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Untrashes a cloud object.
    pub fn untrash_object(
        &mut self,
        _cloud_object_type_and_id: CloudObjectTypeAndId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Updates notebook title.
    pub fn update_notebook_title(
        &mut self,
        _title: std::sync::Arc<String>,
        _id: SyncId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Replaces object with conflict.
    pub fn replace_object_with_conflict(
        &mut self,
        _uid: &cute_server_client::ids::ObjectUid,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Fetches a single cloud object.
    pub fn fetch_single_cloud_object(
        &mut self,
        _id: &ServerId,
        _option: FetchSingleObjectOption,
        _ctx: &mut ModelContext<Self>,
    ) -> impl Future<Output = bool> {
        // Stub implementation - returns false for local version
        std::future::ready(false)
    }

    /// Removes objects owned by a team.
    pub fn remove_team_objects(
        &mut self,
        _team_uid: ServerId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    /// Refreshes updated objects.
    pub fn refresh_updated_objects(
        &mut self,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    pub fn create_workflow(
        &mut self,
        _workflow: crate::workflows::workflow::Workflow,
        _owner: Owner,
        _parent_folder_id: Option<SyncId>,
        _client_id: ClientId,
        _entrypoint: CloudObjectEventEntrypoint,
        _bool_param: bool,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Stub implementation - no-op for local version
    }

    pub fn spawned_futures(&self) -> Vec<usize> {
        vec![]
    }
}

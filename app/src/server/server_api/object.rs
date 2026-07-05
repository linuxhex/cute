use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_channel::Sender;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
#[cfg(test)]
use mockall::automock;

use cute_server_client::cloud_object::{SerializedModel, ServerMetadata};
use cute_server_client::ids::{GenericStringObjectId, HashedSqliteId, ObjectUid, ServerId, SyncId};

use super::ServerApi;
use crate::cloud_object::{
    BulkCreateCloudObjectResult, BulkCreateGenericStringObjectsRequest, CreateCloudObjectResult,
    CreateObjectRequest, ObjectsToUpdate, Owner, Revision,
};
use crate::drive::folders::FolderId;
use crate::drive::sharing::SharingAccessLevel;
use crate::workflows::WorkflowId;

// ---------------------------------------------------------------------------
// Stub types (previously provided by the cloud_object_client crate)
// ---------------------------------------------------------------------------

/// Identifies a guest to remove from an object.
#[derive(Clone, Debug)]
pub enum GuestIdentifier {
    /// Remove a user guest by their email address.
    Email(String),
    /// Remove a team guest by their team UID.
    TeamUid(ServerId),
}

/// The type of action that occurred on an object.
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectActionType {
    Execute,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for ObjectActionType {
    fn to_string(&self) -> String {
        match self {
            ObjectActionType::Execute => String::from("EXECUTE"),
        }
    }
}

impl ObjectActionType {
    pub fn singular(&self) -> String {
        match self {
            ObjectActionType::Execute => "run".to_string(),
        }
    }

    pub fn plural(&self) -> String {
        match self {
            ObjectActionType::Execute => "runs".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectActionSubtype {
    SingleAction {
        timestamp: DateTime<Utc>,
        processed_at_timestamp: Option<DateTime<Utc>>,
        data: Option<String>,
        pending: bool,
    },
    BundledActions {
        count: i32,
        oldest_timestamp: DateTime<Utc>,
        latest_timestamp: DateTime<Utc>,
        latest_processed_at_timestamp: DateTime<Utc>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAction {
    pub action_type: ObjectActionType,
    pub uid: ObjectUid,
    pub hashed_sqlite_id: HashedSqliteId,
    pub action_subtype: ObjectActionSubtype,
}

impl ObjectAction {
    pub fn is_pending(&self) -> bool {
        match self.action_subtype {
            ObjectActionSubtype::SingleAction { pending, .. } => pending,
            ObjectActionSubtype::BundledActions { .. } => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectActionHistory {
    pub uid: ObjectUid,
    pub hashed_sqlite_id: HashedSqliteId,
    pub latest_processed_at_timestamp: DateTime<Utc>,
    pub actions: Vec<ObjectAction>,
}

/// Stub enum — all usage sites are no-ops in the local version.
pub enum ObjectUpdateMessage {}

impl ObjectUpdateMessage {
    pub fn as_str(&self) -> &'static str {
        match *self {}
    }
}

#[derive(Default)]
pub struct InitialLoadResponse {}

pub struct GetCloudObjectResponse {}

#[derive(Clone, Debug)]
pub enum ObjectDeleteResult {
    Success { deleted_ids: Vec<SyncId> },
    Failure,
}

#[derive(Clone, Debug)]
pub enum ObjectMetadataUpdateResult {
    Success { metadata: Box<ServerMetadata> },
    Failure,
}

#[derive(Clone, Debug)]
pub enum ObjectPermissionUpdateResult {
    Success,
    Failure,
}

#[derive(Clone, Debug)]
pub struct ObjectPermissionsUpdateData {}

#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait ObjectClient: 'static + Send + Sync {
    /// This method saves a workflow for a given owner and returns it on success.
    async fn create_workflow(
        &self,
        request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult>;

    /// Updates a workflow with the new data. The update may be rejected if a revision
    /// is specified _and_ that revision is not the current revision of the object in storage.
    async fn update_workflow(
        &self,
        workflow_id: WorkflowId,
        data: SerializedModel,
        revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerWorkflow>>;

    /// Creates n generic string objects in a single graphql request.
    async fn bulk_create_generic_string_objects(
        &self,
        owner: Owner,
        objects: &[BulkCreateGenericStringObjectsRequest],
    ) -> Result<BulkCreateCloudObjectResult>;

    async fn create_generic_string_object(
        &self,
        format: crate::cloud_object::GenericStringObjectFormat,
        uniqueness_key: Option<crate::cloud_object::GenericStringObjectUniqueKey>,
        request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult>;

    /// Creates a notebook on the server, returning the ID and revision of the object after
    /// creation.
    async fn create_notebook(
        &self,
        request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult>;

    /// Updates a notebook with the new title and data.
    async fn update_notebook(
        &self,
        notebook_id: crate::cloud_object::models::NotebookId,
        title: Option<String>,
        data: Option<SerializedModel>,
        revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerNotebook>>;

    async fn create_folder(&self, request: CreateObjectRequest) -> Result<CreateCloudObjectResult>;

    async fn update_folder(
        &self,
        folder_id: FolderId,
        name: SerializedModel,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerFolder>>;

    async fn update_generic_string_object(
        &self,
        object_id: GenericStringObjectId,
        model: SerializedModel,
        revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<Box<dyn crate::cloud_object::ServerObject>>>;

    /// Sets the current editor of the notebook to be the logged in user
    async fn grab_notebook_edit_access(
        &self,
        notebook_id: crate::cloud_object::models::NotebookId,
    ) -> Result<crate::cloud_object::ServerMetadata>;

    /// Sets the current editor of the notebook to be null
    async fn give_up_notebook_edit_access(
        &self,
        notebook_id: crate::cloud_object::models::NotebookId,
    ) -> Result<crate::cloud_object::ServerMetadata>;

    /// Gets updates for all Warp Drive actions.
    async fn get_warp_drive_updates(
        &self,
        message_sender: Sender<ObjectUpdateMessage>,
        stream_ready_sender: Sender<()>,
    ) -> Result<()>;

    async fn fetch_changed_objects(
        &self,
        objects_to_update: ObjectsToUpdate,
        force_refresh: bool,
    ) -> Result<InitialLoadResponse>;

    async fn fetch_single_cloud_object(&self, id: ServerId) -> Result<GetCloudObjectResponse>;

    // Transfers a notebook to the given owner
    async fn transfer_notebook_owner(
        &self,
        notebook_id: crate::cloud_object::models::NotebookId,
        owner: Owner,
    ) -> Result<bool>;

    async fn transfer_workflow_owner(&self, workflow_id: WorkflowId, owner: Owner) -> Result<bool>;

    async fn transfer_generic_string_object_owner(
        &self,
        workflow_id: GenericStringObjectId,
        owner: Owner,
    ) -> Result<bool>;

    async fn trash_object(&self, id: ServerId) -> Result<bool>;

    async fn untrash_object(&self, id: ServerId) -> Result<ObjectMetadataUpdateResult>;

    async fn delete_object(&self, id: ServerId) -> Result<ObjectDeleteResult>;

    async fn empty_trash(&self, owner: Owner) -> Result<ObjectDeleteResult>;

    async fn move_object(
        &self,
        id: ServerId,
        folder_id: Option<FolderId>,
        owner: Owner,
        object_type: crate::cloud_object::ObjectType,
    ) -> Result<bool>;

    async fn record_object_action(
        &self,
        id: ServerId,
        action_type: ObjectActionType,
        timestamp: DateTime<Utc>,
        data: Option<String>,
    ) -> Result<ObjectActionHistory>;

    async fn leave_object(&self, id: ServerId) -> Result<ObjectDeleteResult>;

    async fn set_object_link_permissions(
        &self,
        object_id: ServerId,
        access_level: SharingAccessLevel,
    ) -> Result<ObjectPermissionUpdateResult>;

    async fn remove_object_link_permissions(
        &self,
        object_id: ServerId,
    ) -> Result<ObjectPermissionUpdateResult>;

    async fn add_object_guests(
        &self,
        object_id: ServerId,
        guest_emails: Vec<String>,
        access_level: cute_graphql::object_permissions::AccessLevel,
    ) -> Result<ObjectPermissionsUpdateData>;

    async fn update_object_guests(
        &self,
        object_id: ServerId,
        guest_emails: Vec<String>,
        access_level: cute_graphql::object_permissions::AccessLevel,
    ) -> Result<crate::cloud_object::ServerPermissions>;

    async fn remove_object_guest(
        &self,
        object_id: ServerId,
        guest: GuestIdentifier,
    ) -> Result<crate::cloud_object::ServerPermissions>;

    /// Fetches the last-used timestamps for all cloud environments.
    async fn fetch_environment_last_task_run_timestamps(
        &self,
    ) -> Result<HashMap<String, DateTime<Utc>>>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl ObjectClient for ServerApi {
    async fn create_workflow(
        &self,
        _request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_workflow(
        &self,
        _workflow_id: WorkflowId,
        _data: SerializedModel,
        _revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerWorkflow>> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn bulk_create_generic_string_objects(
        &self,
        _owner: Owner,
        _objects: &[BulkCreateGenericStringObjectsRequest],
    ) -> Result<BulkCreateCloudObjectResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn create_generic_string_object(
        &self,
        _format: crate::cloud_object::GenericStringObjectFormat,
        _uniqueness_key: Option<crate::cloud_object::GenericStringObjectUniqueKey>,
        _request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn create_notebook(
        &self,
        _request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_notebook(
        &self,
        _notebook_id: crate::cloud_object::models::NotebookId,
        _title: Option<String>,
        _data: Option<SerializedModel>,
        _revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerNotebook>> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn create_folder(&self, _request: CreateObjectRequest) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_folder(
        &self,
        _folder_id: FolderId,
        _name: SerializedModel,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<crate::cloud_object::ServerFolder>> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_generic_string_object(
        &self,
        _object_id: GenericStringObjectId,
        _model: SerializedModel,
        _revision: Option<Revision>,
    ) -> Result<crate::cloud_object::UpdateCloudObjectResult<Box<dyn crate::cloud_object::ServerObject>>> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn grab_notebook_edit_access(
        &self,
        _notebook_id: crate::cloud_object::models::NotebookId,
    ) -> Result<crate::cloud_object::ServerMetadata> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn give_up_notebook_edit_access(
        &self,
        _notebook_id: crate::cloud_object::models::NotebookId,
    ) -> Result<crate::cloud_object::ServerMetadata> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn get_warp_drive_updates(
        &self,
        _message_sender: Sender<ObjectUpdateMessage>,
        _stream_ready_sender: Sender<()>,
    ) -> Result<()> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn fetch_changed_objects(
        &self,
        _objects_to_update: ObjectsToUpdate,
        _force_refresh: bool,
    ) -> Result<InitialLoadResponse> {
        Ok(InitialLoadResponse::default())
    }

    async fn fetch_single_cloud_object(&self, _id: ServerId) -> Result<GetCloudObjectResponse> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn transfer_notebook_owner(
        &self,
        _notebook_id: crate::cloud_object::models::NotebookId,
        _owner: Owner,
    ) -> Result<bool> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn transfer_workflow_owner(&self, _workflow_id: WorkflowId, _owner: Owner) -> Result<bool> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn transfer_generic_string_object_owner(
        &self,
        _object_id: GenericStringObjectId,
        _owner: Owner,
    ) -> Result<bool> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn trash_object(&self, _id: ServerId) -> Result<bool> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn untrash_object(&self, _id: ServerId) -> Result<ObjectMetadataUpdateResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn delete_object(&self, _id: ServerId) -> Result<ObjectDeleteResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn empty_trash(&self, _owner: Owner) -> Result<ObjectDeleteResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn move_object(
        &self,
        _id: ServerId,
        _folder_id: Option<FolderId>,
        _owner: Owner,
        _object_type: crate::cloud_object::ObjectType,
    ) -> Result<bool> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn record_object_action(
        &self,
        _id: ServerId,
        _action_type: ObjectActionType,
        _timestamp: DateTime<Utc>,
        _data: Option<String>,
    ) -> Result<ObjectActionHistory> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn leave_object(&self, _id: ServerId) -> Result<ObjectDeleteResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn set_object_link_permissions(
        &self,
        _object_id: ServerId,
        _access_level: SharingAccessLevel,
    ) -> Result<ObjectPermissionUpdateResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn remove_object_link_permissions(
        &self,
        _object_id: ServerId,
    ) -> Result<ObjectPermissionUpdateResult> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn add_object_guests(
        &self,
        _object_id: ServerId,
        _guest_emails: Vec<String>,
        _access_level: cute_graphql::object_permissions::AccessLevel,
    ) -> Result<ObjectPermissionsUpdateData> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_object_guests(
        &self,
        _object_id: ServerId,
        _guest_emails: Vec<String>,
        _access_level: cute_graphql::object_permissions::AccessLevel,
    ) -> Result<crate::cloud_object::ServerPermissions> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn remove_object_guest(
        &self,
        _object_id: ServerId,
        _guest: GuestIdentifier,
    ) -> Result<crate::cloud_object::ServerPermissions> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn fetch_environment_last_task_run_timestamps(
        &self,
    ) -> Result<HashMap<String, DateTime<Utc>>> {
        Ok(HashMap::new())
    }
}

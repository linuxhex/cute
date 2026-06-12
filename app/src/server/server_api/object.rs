use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_channel::Sender;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use cloud_object_client::{
    GetCloudObjectResponse, InitialLoadResponse, ObjectActionHistory, ObjectActionType,
    ObjectDeleteResult, ObjectMetadataUpdateResult, ObjectPermissionUpdateResult,
    ObjectPermissionsUpdateData, ObjectUpdateMessage, SerializedModel,
};
pub use cloud_object_client::{GuestIdentifier, ObjectClient};
use cloud_objects::ids::{GenericStringObjectId, ServerId};

use super::ServerApi;
use crate::cloud_object::{
    BulkCreateCloudObjectResult, BulkCreateGenericStringObjectsRequest, CreateCloudObjectResult,
    CreateObjectRequest, ObjectsToUpdate, Owner, Revision,
};
use crate::drive::folders::FolderId;
use crate::drive::sharing::SharingAccessLevel;
use crate::server::ids::ToServerId;
use crate::workflows::WorkflowId;

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
        _notebook_id: cloud_object_models::NotebookId,
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
        _notebook_id: cloud_object_models::NotebookId,
    ) -> Result<crate::cloud_object::ServerMetadata> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn give_up_notebook_edit_access(
        &self,
        _notebook_id: cloud_object_models::NotebookId,
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
        _notebook_id: cloud_object_models::NotebookId,
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
        _access_level: warp_graphql::object_permissions::AccessLevel,
    ) -> Result<ObjectPermissionsUpdateData> {
        Err(anyhow!("Cloud objects not supported in local version"))
    }

    async fn update_object_guests(
        &self,
        _object_id: ServerId,
        _guest_emails: Vec<String>,
        _access_level: warp_graphql::object_permissions::AccessLevel,
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

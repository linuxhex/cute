use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;

use chrono::{Duration, Utc};
use derivative::Derivative;
use lazy_static::lazy_static;
use regex::Regex;
use url::Url;
use cute_core::channel::Channel;
use cuteui::{AppContext, SingletonEntity};

use self::breadcrumbs::ContainingObject;
use self::model::actions::ObjectActions;

pub use self::model::generic_string_model::{GenericStringObjectId, GenericStringModel, Serializer, StringModel};
pub use self::model::persistence::CloudModel;
pub use crate::server::ids::HashedSqliteId;
use crate::appearance::Appearance;
use crate::channel::ChannelState;
use crate::drive::items::WarpDriveItem;
use crate::drive::{CloudObjectTypeAndId, OpenWarpDriveObjectArgs};
use crate::persistence::ModelEvent;
use crate::server::ids::{HashableId, ObjectUid, ServerId, SyncId, ToServerId};
use crate::util::time_format::format_approx_duration_from_now_utc;
use crate::workflows::{CloudWorkflow, WorkflowSource};
use crate::workspaces::user_profiles::UserProfiles;
use crate::workspaces::user_workspaces::UserWorkspaces;

pub mod breadcrumbs;
pub mod model;
pub mod models;
pub mod toast_message;

pub use cute_server_client::cloud_object::*;

pub trait CloudObject: Debug {
    fn model_type_name(&self) -> &'static str;

    fn uid(&self) -> ObjectUid;

    fn sync_id(&self) -> SyncId;

    fn hashed_sqlite_id(&self) -> HashedSqliteId;

    fn metadata(&self) -> &CloudObjectMetadata;

    fn metadata_mut(&mut self) -> &mut CloudObjectMetadata;

    fn permissions(&self) -> &CloudObjectPermissions;

    fn permissions_mut(&mut self) -> &mut CloudObjectPermissions;

    fn object_type(&self) -> ObjectType;

    fn cloud_object_type_and_id(&self) -> CloudObjectTypeAndId;

    fn set_server_id(&mut self, _server_id: ServerId) {}

    fn can_move_to_space(&self, _space: Space, _app: &AppContext) -> bool {
        true
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        false
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        true
    }

    fn upsert_event(&self) -> ModelEvent;

    fn display_name(&self) -> String;

    fn versions(&self, _app: &AppContext) -> Option<cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput> {
        None
    }

    fn renders_in_warp_drive(&self) -> bool;

    fn should_show_activity_toasts(&self) -> bool {
        true
    }

    fn to_warp_drive_item(&self, appearance: &Appearance) -> Option<Box<dyn WarpDriveItem>>;

    fn object_link(&self) -> Option<String> {
        None
    }

    fn space(&self, app: &AppContext) -> Space {
        UserWorkspaces::as_ref(app).owner_to_space(self.permissions().owner, app)
    }

    fn can_leave(&self, _app: &AppContext) -> bool {
        false
    }

    fn containing_object_name(&self, app: &AppContext) -> String {
        self.containing_objects_path(app)
            .into_iter()
            .next_back()
            .expect("Object should have at least one ancestor")
            .name
    }

    fn containing_objects_path(&self, app: &AppContext) -> Vec<ContainingObject> {
        let space = self.space(app);

        match self.metadata().folder_id {
            Some(folder_id) => {
                let cloud_model = CloudModel::as_ref(app);
                if let Some(folder) = cloud_model.get_folder_by_uid(&folder_id.uid()) {
                    let mut path = vec![];
                    let ancestors = folder.containing_objects_path(app);
                    path.extend(ancestors);
                    path.push(folder.into());
                    path
                } else {
                    vec![space.into_containing_object(app)]
                }
            }
            None => vec![space.into_containing_object(app)],
        }
    }

    fn breadcrumbs(&self, app: &AppContext) -> String {
        self.containing_objects_path(app)
            .into_iter()
            .map(|object| object.name)
            .collect::<Vec<String>>()
            .join(" / ")
    }

    fn is_in_space(&self, space: Space, app: &AppContext) -> bool {
        self.space(app) == space
    }

    fn is_welcome_object(&self) -> bool {
        self.metadata().is_welcome_object
    }

    fn location(&self, cloud_model: &CloudModel, app: &AppContext) -> CloudObjectLocation {
        if let Some(folder_id) = self.metadata().folder_id {
            if cloud_model.get_folder(&folder_id).is_some() {
                return CloudObjectLocation::Folder(folder_id);
            }
        }

        CloudObjectLocation::Space(self.space(app))
    }

    fn is_trashed(&self, cloud_model: &CloudModel) -> bool {
        self.is_trashed_internal(cloud_model, &mut HashSet::new())
    }

    fn is_trashed_internal(
        &self,
        cloud_model: &CloudModel,
        ancestors: &mut HashSet<String>,
    ) -> bool {
        if self.metadata().trashed_ts.is_some() {
            return true;
        }

        match self.metadata().folder_id.map(|parent_id| parent_id.uid()) {
            Some(hashed_parent_id) => {
                if ancestors.contains(&hashed_parent_id) {
                    return true;
                }
                ancestors.insert(hashed_parent_id.clone());

                match cloud_model.get_by_uid(&hashed_parent_id) {
                    Some(parent) => parent.is_trashed_internal(cloud_model, ancestors),
                    None => false,
                }
            }
            None => false,
        }
    }

    fn has_conflicting_changes(&self) -> bool {
        false
    }

    fn conflicting_object_revision(&self) -> Option<Revision> {
        None
    }

    fn clear_conflict_status(&mut self) {}

    fn replace_object_with_conflict(&mut self) {}

    fn increment_in_flight_request_count(&mut self) {}

    fn decrement_in_flight_request_count(&mut self, _status_if_no_reqs: CloudObjectSyncStatus) -> bool {
        true
    }

    fn set_pending_content_changes_status(&mut self, _pending_content_changes_status: CloudObjectSyncStatus) {}

    fn can_export(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_model_type<K, M>(cloud_object: &dyn CloudObject) -> Option<&GenericCloudObject<K, M>>
    where
        Self: Sized,
        K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        cloud_object
            .as_any()
            .downcast_ref::<GenericCloudObject<K, M>>()
    }

    fn as_model_type_mut<K, M>(
        cloud_object: &mut dyn CloudObject,
    ) -> Option<&mut GenericCloudObject<K, M>>
    where
        Self: Sized,
        K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        cloud_object
            .as_any_mut()
            .downcast_mut::<GenericCloudObject<K, M>>()
    }

    fn clone_box(&self) -> Box<dyn CloudObject>;

    fn create_object_queue_item(
        &self,
        _entrypoint: CloudObjectEventEntrypoint,
        _initiated_by: crate::server::cloud_objects::update_manager::InitiatedBy,
    ) -> Option<crate::server::sync_queue::QueueItem> {
        None
    }

    fn update_object_queue_item(&self, _revision: Option<Revision>) -> crate::server::sync_queue::QueueItem {
        panic!("update_object_queue_item: cloud sync has been removed")
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
pub trait CloudModelType: Debug + Clone + Send + Sync {
    type CloudObjectType: CloudObject + 'static;
    type IdType: HashableId + ToServerId + Debug + Into<String> + Clone + 'static;

    fn model_type_name(&self) -> &'static str;

    fn cloud_object_type_and_id(&self, id: SyncId) -> CloudObjectTypeAndId;

    fn object_type(&self) -> ObjectType;

    fn renders_in_warp_drive(&self) -> bool;

    fn should_show_activity_toasts(&self) -> bool {
        true
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        true
    }

    fn to_warp_drive_item(
        &self,
        id: SyncId,
        appearance: &Appearance,
        object: &Self::CloudObjectType,
    ) -> Option<Box<dyn WarpDriveItem>>;

    fn display_name(&self) -> String;

    fn set_display_name(&mut self, _name: &str) {}

    fn upsert_event(params: CloudObjectUpsertParams<Self>) -> ModelEvent
    where
        Self: Sized;

    fn bulk_upsert_event(objects: Vec<CloudObjectUpsertParams<Self>>) -> ModelEvent
    where
        Self: Sized;

    async fn send_create_request(
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _request: CreateObjectRequest,
    ) -> anyhow::Result<CreateCloudObjectResult> {
        Err(anyhow::anyhow!("cloud sync has been removed"))
    }

    async fn send_update_request(
        &self,
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _server_id: ServerId,
        _revision: Option<Revision>,
    ) -> anyhow::Result<UpdateCloudObjectResult<GenericServerObject<Self::IdType, Self>>> {
        Err(anyhow::anyhow!("cloud sync has been removed"))
    }

    fn can_move_to_space(&self, _current_space: Space, _new_space: Space) -> bool {
        true
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        false
    }

    fn supports_linking(&self) -> bool {
        false
    }

    fn should_update_after_server_conflict(&self) -> bool {
        false
    }

    fn can_export(&self) -> bool {
        false
    }
}

pub trait CloudObjectLookup: Sized + Clone {
    fn get_all(app: &AppContext) -> Vec<Self>;

    fn get_by_id<'a>(sync_id: &'a SyncId, app: &'a AppContext) -> Option<&'a Self>;
}

impl<K, M> CloudObjectLookup for GenericCloudObject<K, M>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn get_all(app: &AppContext) -> Vec<Self> {
        CloudModel::as_ref(app)
            .get_all_objects_of_type::<K, M>()
            .cloned()
            .collect()
    }

    fn get_by_id<'a>(sync_id: &'a SyncId, app: &'a AppContext) -> Option<&'a Self> {
        CloudModel::as_ref(app).get_object_of_type::<K, M>(sync_id)
    }
}

pub trait CloudObjectUuid {
    fn uuid(&self) -> uuid::Uuid;
}

pub trait CloudObjectUuidLookup: Sized {
    fn get_by_uuid<'a>(uuid: &'a uuid::Uuid, app: &'a AppContext) -> Option<&'a Self>;
}

impl<T, S> CloudObjectUuidLookup
    for GenericCloudObject<GenericStringObjectId, GenericStringModel<T, S>>
where
    T: StringModel<
            CloudObjectType = GenericCloudObject<GenericStringObjectId, GenericStringModel<T, S>>,
        > + CloudObjectUuid,
    S: Serializer<T>,
{
    fn get_by_uuid<'a>(uuid: &'a uuid::Uuid, app: &'a AppContext) -> Option<&'a Self> {
        CloudModel::as_ref(app)
            .get_all_objects_of_type::<GenericStringObjectId, GenericStringModel<T, S>>()
            .find(|object| object.model().string_model.uuid() == *uuid)
    }
}

lazy_static! {
    static ref SPACE_DETECT_RE: Regex = Regex::new(r"\s+").expect("Expect regex to be valid");
    static ref SAFE_URL_CHAR_RE: Regex =
        Regex::new(r"[^a-zA-Z0-9\s-]").expect("Expect regex to be valid");
}

impl<K, M> CloudObject for GenericCloudObject<K, M>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn model_type_name(&self) -> &'static str {
        self.model().model_type_name()
    }

    fn uid(&self) -> ObjectUid {
        self.id.uid()
    }

    fn hashed_sqlite_id(&self) -> HashedSqliteId {
        self.id.sqlite_uid_hash(self.object_type().into())
    }

    fn sync_id(&self) -> SyncId {
        self.id
    }

    fn should_show_activity_toasts(&self) -> bool {
        self.model().should_show_activity_toasts()
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        self.model().warn_if_unsaved_at_quit()
    }

    fn metadata(&self) -> &CloudObjectMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut CloudObjectMetadata {
        &mut self.metadata
    }

    fn permissions(&self) -> &CloudObjectPermissions {
        &self.permissions
    }

    fn permissions_mut(&mut self) -> &mut CloudObjectPermissions {
        &mut self.permissions
    }

    fn object_type(&self) -> ObjectType {
        self.model().object_type()
    }

    fn cloud_object_type_and_id(&self) -> CloudObjectTypeAndId {
        self.model().cloud_object_type_and_id(self.id)
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        self.model().should_clear_on_unique_key_conflict()
    }

    fn can_move_to_space(&self, space: Space, app: &AppContext) -> bool {
        self.model().can_move_to_space(self.space(app), space)
    }

    fn has_conflicting_changes(&self) -> bool {
        self.conflict_status.has_conflicts()
    }

    fn conflicting_object_revision(&self) -> Option<Revision> {
        match &self.conflict_status {
            ConflictStatus::ConflictingChanges { object } => Some(object.metadata.revision.clone()),
            ConflictStatus::NoConflicts => None,
        }
    }

    fn clear_conflict_status(&mut self) {
        self.conflict_status = ConflictStatus::NoConflicts;
    }

    fn replace_object_with_conflict(&mut self) {
        let mut new_conflict = ConflictStatus::NoConflicts;
        std::mem::swap(&mut new_conflict, &mut self.conflict_status);

        self.set_pending_content_changes_status(CloudObjectSyncStatus::NoLocalChanges);

        if let ConflictStatus::ConflictingChanges { object } = new_conflict {
            if self.model().should_update_after_server_conflict() {
                self.metadata.update_revision_from_server(&object.metadata);
                self.set_model(object.model.clone());
                if self.metadata.has_pending_content_changes() {
                    self.conflict_status = ConflictStatus::ConflictingChanges { object };
                } else {
                    self.conflict_status = ConflictStatus::NoConflicts;
                }
            }
        }
    }

    fn set_server_id(&mut self, server_id: ServerId) {
        self.id = SyncId::ServerId(server_id);
    }

    fn object_link(&self) -> Option<String> {
        if !self.model().supports_linking() {
            return None;
        }

        let display_name = self.model().display_name();
        let name_without_unsafe_chars = SAFE_URL_CHAR_RE.replace_all(display_name.trim(), "");
        let link_safe_name = SPACE_DETECT_RE.replace_all(&name_without_unsafe_chars, "-");
        match &self.id {
            SyncId::ClientId(_) => None,
            SyncId::ServerId(id) => {
                let object_type = self.object_type();
                let object_type_for_link = if self
                    .as_any()
                    .downcast_ref::<CloudWorkflow>()
                    .is_some_and(|w| w.model().data.is_agent_mode_workflow())
                {
                    "prompt".to_string()
                } else {
                    object_type.to_string()
                };

                let mut link = format!(
                    "{}/drive/{}/{}-{}",
                    ChannelState::server_root_url(),
                    object_type_for_link,
                    link_safe_name,
                    id.uid()
                );

                if matches!(ChannelState::channel(), Channel::Preview) {
                    link.push_str("?preview=true");
                }

                Some(link)
            }
        }
    }

    fn upsert_event(&self) -> ModelEvent {
        M::upsert_event(self.upsert_params(self.object_type()))
    }

    fn display_name(&self) -> String {
        self.model().display_name()
    }

    fn versions(&self, app: &AppContext) -> Option<cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput> {
        match (self.id, self.metadata.revision.as_ref()) {
            (SyncId::ServerId(id), Some(revision)) => {
                let actions_ts = ObjectActions::as_ref(app)
                    .get_latest_processed_at_ts(&self.id.uid())
                    .map(|t| t.into());
                Some(cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput {
                    uid: id.into(),
                    revision_ts: revision.timestamp(),
                    metadata_ts: self.metadata.metadata_last_updated_ts,
                    permissions_ts: self.permissions.permissions_last_updated_ts,
                    actions_ts,
                })
            }
            _ => None,
        }
    }

    fn renders_in_warp_drive(&self) -> bool {
        self.model().renders_in_warp_drive()
    }

    fn to_warp_drive_item(&self, appearance: &Appearance) -> Option<Box<dyn WarpDriveItem>> {
        self.model().to_warp_drive_item(self.id, appearance, self)
    }

    fn can_export(&self) -> bool {
        self.model().can_export()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn CloudObject> {
        Box::new(self.clone())
    }
}

pub fn extract_server_id_and_object_type_from_warp_drive_link(
    _url: &Url,
) -> Option<OpenWarpDriveObjectArgs> {
    None
}

impl<'a, K, M> From<&'a dyn CloudObject> for Option<&'a GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a dyn CloudObject) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type(value)
    }
}

impl<'a, K, M> From<&'a Box<dyn CloudObject>> for Option<&'a GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a Box<dyn CloudObject>) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type(value.as_ref())
    }
}

impl<'a, K, M> From<&'a mut Box<dyn CloudObject>> for Option<&'a mut GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a mut Box<dyn CloudObject>) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type_mut(value.as_mut())
    }
}

impl Clone for Box<dyn CloudObject> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl From<&dyn CloudObject> for ObjectType {
    fn from(value: &dyn CloudObject) -> Self {
        value.object_type()
    }
}

impl From<&Box<dyn CloudObject>> for ObjectType {
    fn from(value: &Box<dyn CloudObject>) -> Self {
        <ObjectType as From<&dyn CloudObject>>::from(value.as_ref())
    }
}

pub trait CloudObjectMetadataExt {
    fn semantic_editing_history(&self, app: &AppContext) -> Option<String>;

    #[cfg_attr(target_family = "wasm", expect(dead_code))]
    fn semantic_creator(&self, app: &AppContext) -> Option<String>;

    fn semantic_permadeletion_countdown(&self, app: &AppContext) -> Option<String>;
}

impl CloudObjectMetadataExt for CloudObjectMetadata {
    fn semantic_editing_history(&self, app: &AppContext) -> Option<String> {
        let user_profiles = UserProfiles::as_ref(app);

        let editor_string = self
            .last_editor_uid
            .as_ref()
            .and_then(|uid| user_profiles.displayable_identifier_for_uid(crate::auth::UserUid::new(uid)));

        let time_ago_string = self
            .revision
            .clone()
            .map(|r| format_approx_duration_from_now_utc(r.utc()));

        let full_string = match (editor_string, time_ago_string) {
            (Some(name), Some(time_ago)) if name.is_empty() => format!("Edited {time_ago}"),
            (Some(name), Some(time_ago)) => format!("{name} edited {time_ago}"),
            (None, Some(time_ago)) => format!("Edited {time_ago}"),
            (Some(name), None) => format!("Last edited by {name}"),
            _ => return None,
        };

        Some(full_string)
    }

    fn semantic_creator(&self, app: &AppContext) -> Option<String> {
        let user_profiles = UserProfiles::as_ref(app);
        self.creator_uid
            .as_ref()
            .and_then(|uid| user_profiles.displayable_identifier_for_uid(crate::auth::UserUid::new(uid)))
    }

    fn semantic_permadeletion_countdown(&self, app: &AppContext) -> Option<String> {
        if let Some(trashed_ts) = self
            .trashed_ts
            .or_else(|| get_top_folder_trashed_ts(self.folder_id, app))
        {
            let deletion_time = trashed_ts.utc() + Duration::days(31);
            let current_time = Utc::now();
            let days_left = deletion_time.signed_duration_since(current_time).num_days();

            let full_string = match days_left {
                0 | 1 => "1 day until permanent deletion".to_string(),
                _ => format!("{days_left} days until permanent deletion"),
            };
            Some(full_string)
        } else {
            None
        }
    }
}

fn get_top_folder_trashed_ts(
    folder_id: Option<SyncId>,
    app: &AppContext,
) -> Option<cute_graphql::scalars::time::ServerTimestamp> {
    let mut folder_id = folder_id;
    let cloud_model = CloudModel::as_ref(app);
    while let Some(current_folder_id) = folder_id {
        let folder = cloud_model.get_folder_by_uid(&current_folder_id.uid())?;

        if let Some(_parent_folder_id) = folder.metadata.folder_id {
            folder_id = folder.metadata.folder_id
        } else {
            return folder.metadata.trashed_ts;
        }
    }
    None
}

pub use models::{
    ServerCloudObject, ServerFolder, ServerNotebook, ServerWorkflow,
};

#[derive(Default, Clone, Copy, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum Space {
    #[default]
    Personal,
    Team { team_uid: ServerId },
    Shared,
}

impl Space {
    pub fn name(&self, app: &AppContext) -> String {
        match self {
            Space::Personal => "Personal".to_string(),
            Space::Team { team_uid, .. } => {
                let user_workspaces = UserWorkspaces::as_ref(app);
                if let Some(team) = user_workspaces.team_from_uid(*team_uid) {
                    team.name.clone()
                } else {
                    "Team".to_string()
                }
            }
            Space::Shared => "Shared with me".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum CloudObjectLocation {
    Space(Space),
    Folder(SyncId),
    Trash,
}

impl From<Space> for WorkflowSource {
    fn from(space: Space) -> Self {
        match space {
            Space::Personal => WorkflowSource::PersonalCloud,
            Space::Team { team_uid } => WorkflowSource::Team { team_uid },
            Space::Shared => WorkflowSource::PersonalCloud,
        }
    }
}

impl From<Owner> for WorkflowSource {
    fn from(owner: Owner) -> Self {
        match owner {
            Owner::User { .. } => Self::PersonalCloud,
            Owner::Team { team_uid } => Self::Team { team_uid },
        }
    }
}

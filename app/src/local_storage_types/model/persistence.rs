use std::collections::{HashMap, HashSet};
use std::sync::mpsc::SyncSender;

use chrono::{DateTime, Utc};
use itertools::Itertools;
use cuteui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::server::ids::HashableId;
use crate::server::ids::ToServerId;

use super::generic_string_model::GenericStringObjectId;
use crate::ai::execution_profiles::CloudAIExecutionProfile;
use crate::local_storage_types::{
    CloudModelType, CloudObject, CloudObjectLocation, GenericCloudObject,
    ObjectIdType, ObjectType, Owner,
    Space,
};
//  // Removed: WarpDrive functionality
// Import cloud stub types for removed WarpDrive functionality
use crate::{CloudObjectTypeAndId, CloudFolder, CloudFolderModel, CloudNotebook, DriveIndexVariant};
use crate::env_vars::{CloudEnvVarCollection, CloudEnvVarCollectionModel, EnvVarCollection};
// use crate::local_storage_types::CloudNotebook; // Removed: cloud notebook functionality
use crate::persistence::ModelEvent;
use crate::server::ids::{ObjectUid, SyncId};
use crate::settings::cloud_preferences::{CloudPreference, CloudPreferenceModel};
use crate::workflows::workflow::Workflow;
use crate::workflows::workflow_enum::{CloudWorkflowEnum, CloudWorkflowEnumModel, WorkflowEnum};
use crate::workflows::{CloudWorkflow, CloudWorkflowModel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateSource {
    Server,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudModelEvent {
    InitialLoadCompleted,
    ObjectMoved {
        type_and_id: CloudObjectTypeAndId,
        source: UpdateSource,
        from_folder: Option<SyncId>,
        to_folder: Option<SyncId>,
    },
    ObjectUpdated {
        type_and_id: CloudObjectTypeAndId,
        source: UpdateSource,
    },
    ObjectTrashed {
        type_and_id: CloudObjectTypeAndId,
        source: UpdateSource,
    },
    ObjectUntrashed {
        type_and_id: CloudObjectTypeAndId,
        source: UpdateSource,
    },
    ObjectCreated {
        type_and_id: CloudObjectTypeAndId,
    },
    ObjectDeleted {
        type_and_id: CloudObjectTypeAndId,
        folder_id: Option<SyncId>,
    },
    ObjectPermissionsUpdated {
        type_and_id: CloudObjectTypeAndId,
        source: UpdateSource,
    },
    ObjectForceExpanded {
        id: String,
    },
}

enum FolderOpenState {
    Open,
    Closed,
    Reversed,
}

pub struct CloudModel {
    objects_by_id: HashMap<ObjectUid, Box<dyn CloudObject>>,
    model_event_sender: Option<SyncSender<ModelEvent>>,
}

impl CloudModel {
    pub fn new(
        model_event_sender: Option<SyncSender<ModelEvent>>,
        cached_objects: Vec<Box<dyn CloudObject>>,
        _time_of_next_force_refresh: Option<DateTime<Utc>>,
    ) -> Self {
        let objects_by_id = cached_objects
            .into_iter()
            .map(|object| (object.uid().to_owned(), object))
            .collect::<HashMap<ObjectUid, Box<dyn CloudObject>>>();

        Self {
            objects_by_id,
            model_event_sender,
        }
    }

    pub fn can_move_object_to_location(
        &self,
        hashed_id: &str,
        new_location: CloudObjectLocation,
        app: &AppContext,
    ) -> bool {
        if let Some(object) = self.objects_by_id.get(hashed_id) {
            let object_space = object.space(app);
            if let CloudObjectLocation::Space(space) = new_location {
                if matches!(object_space, Space::Team { .. }) && space == Space::Personal {
                    return false;
                }

                if !object.can_move_to_space(space, app) {
                    return false;
                }
            }

            if let CloudObjectLocation::Folder(target_folder_id) = new_location {
                let folder_to_move: Option<&CloudFolder> = object.into();
                if let Some(folder_to_move) = folder_to_move {
                    if folder_to_move.id == target_folder_id {
                        return false;
                    }

                    let mut target_folder_parent_folder_id = self
                        .get_folder(&target_folder_id)
                        .and_then(|folder| folder.metadata().folder_id);
                    while let Some(parent_id) = target_folder_parent_folder_id {
                        if parent_id == folder_to_move.id {
                            return false;
                        }
                        target_folder_parent_folder_id = self
                            .get_folder(&parent_id)
                            .and_then(|folder| folder.metadata().folder_id);
                    }
                }
                if let Some(target_folder) = self.get_folder(&target_folder_id) {
                    if target_folder.permissions.owner != object.permissions().owner {
                        return false;
                    }
                }
            }

            return true;
        }
        false
    }

    pub fn object_location(
        &self,
        hashed_id: &str,
        app: &AppContext,
    ) -> Option<CloudObjectLocation> {
        self.objects_by_id
            .get(hashed_id)
            .map(|object| object.location(self, app))
    }

    pub fn object_link(&self, uid: &ObjectUid) -> Option<String> {
        self.objects_by_id.get(uid).and_then(|object| object.object_link())
    }

    pub fn get_by_uid(&self, uid: &ObjectUid) -> Option<&dyn CloudObject> {
        self.objects_by_id.get(uid).map(|o| o.as_ref())
    }

    pub fn get_mut_by_uid(&mut self, uid: &ObjectUid) -> Option<&mut Box<dyn CloudObject>> {
        self.objects_by_id.get_mut(uid)
    }

    pub fn cloud_objects(&self) -> impl Iterator<Item = &Box<dyn CloudObject>> {
        self.objects_by_id.values()
    }

    pub fn cloud_objects_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn CloudObject>> {
        self.objects_by_id.values_mut()
    }

    pub fn create_object(
        &mut self,
        id: SyncId,
        object: impl CloudObject + 'static,
        ctx: &mut ModelContext<CloudModel>,
    ) {
        ctx.emit(CloudModelEvent::ObjectCreated {
            type_and_id: object.cloud_object_type_and_id(),
        });
        self.create_object_internal(id, object);
        ctx.notify();
    }

    fn create_object_internal(&mut self, id: SyncId, object: impl CloudObject + 'static) {
        self.objects_by_id.insert(id.uid(), Box::new(object));
    }

    pub fn delete_objects_by_id(
        &mut self,
        uids: Vec<ObjectUid>,
        ctx: &mut ModelContext<Self>,
    ) -> (Vec<(SyncId, ObjectIdType)>, i32) {
        let mut count = 0;
        let mut sync_ids_and_types: Vec<(SyncId, ObjectIdType)> = Vec::new();
        for uid in uids {
            if let Some(object) = self.objects_by_id.remove(&uid) {
                let cloud_object_type_and_id = object.cloud_object_type_and_id();
                sync_ids_and_types.push((
                    cloud_object_type_and_id.sync_id(),
                    cloud_object_type_and_id.object_type().into(),
                ));

                ctx.emit(CloudModelEvent::ObjectDeleted {
                    type_and_id: object.cloud_object_type_and_id(),
                    folder_id: object.metadata().folder_id,
                });
                count += 1;
            }
        }
        ctx.notify();
        (sync_ids_and_types, count)
    }

    pub fn delete_object_and_descendants(
        &mut self,
        uid: ObjectUid,
        ctx: &mut ModelContext<Self>,
    ) -> Vec<(SyncId, ObjectIdType)> {
        let mut accumulator = Vec::new();
        self.delete_object_and_descendants_internal(uid, &mut accumulator, ctx);
        accumulator
    }

    fn delete_object_and_descendants_internal(
        &mut self,
        uid: ObjectUid,
        accumulator: &mut Vec<(SyncId, ObjectIdType)>,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(object) = self.objects_by_id.remove(&uid) {
            accumulator.push((
                object.sync_id(),
                object.object_type().into(),
            ));
            ctx.emit(CloudModelEvent::ObjectDeleted {
                type_and_id: object.cloud_object_type_and_id(),
                folder_id: object.metadata().folder_id,
            });
            if object.object_type() == ObjectType::Folder {
                let contents = self
                    .objects_by_id
                    .iter()
                    .filter_map(|(child_uid, child)| {
                        if child
                            .metadata()
                            .folder_id
                            .is_some_and(|parent| parent.uid() == uid)
                        {
                            Some(child_uid.clone())
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                for child in contents {
                    self.delete_object_and_descendants_internal(child, accumulator, ctx);
                }
            }
        }
    }

    pub fn check_if_object_is_in_cloudmodel(&mut self, uid: ObjectUid) -> bool {
        self.objects_by_id.contains_key(&uid)
    }

    pub fn update_object_location(
        &mut self,
        uid: &ObjectUid,
        new_owner: Option<Owner>,
        new_folder: Option<SyncId>,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(object) = self.get_mut_by_uid(uid) {
            let old_folder = object.metadata().folder_id;
            let mut changed = false;

            if let Some(new_owner) = new_owner {
                if new_owner != object.permissions().owner {
                    object.permissions_mut().owner = new_owner;
                    changed = true;
                }
            }

            if new_folder != old_folder {
                object.metadata_mut().folder_id = new_folder;
                changed = true;
            }

            if changed {
                ctx.emit(CloudModelEvent::ObjectMoved {
                    type_and_id: object.cloud_object_type_and_id(),
                    source: UpdateSource::Local,
                    from_folder: old_folder,
                    to_folder: new_folder,
                });
                ctx.notify();
            }
        }
    }

    pub fn update_object_from_edit<K, M>(
        &mut self,
        model: M,
        object_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) where
        K: HashableId
            + ToServerId
            + std::fmt::Debug
            + Into<String>
            + Clone
            + Copy
            + Send
            + Sync
            + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        if let Some(cloud_object) = self.get_object_of_type_mut(&object_id) {
            cloud_object.set_model(model);
            ctx.emit(CloudModelEvent::ObjectUpdated {
                type_and_id: cloud_object.cloud_object_type_and_id(),
                source: UpdateSource::Local,
            });
            ctx.notify();
        }
    }

    pub fn overwrite_workflow(
        &mut self,
        workflow: Workflow,
        workflow_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(cloud_workflow) = self.get_workflow_mut(&workflow_id) {
            cloud_workflow.set_model(CloudWorkflowModel::new(workflow));
            ctx.emit(CloudModelEvent::ObjectUpdated {
                type_and_id: cloud_workflow.cloud_object_type_and_id(),
                source: UpdateSource::Local,
            });
            ctx.notify();
        }
    }

    pub fn overwrite_env_var_collection(
        &mut self,
        env_var_collection: EnvVarCollection,
        env_var_collection_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(cloud_env_var_collection) = self
            .get_object_of_type_mut::<GenericStringObjectId, CloudEnvVarCollectionModel>(
                &env_var_collection_id,
            )
        {
            cloud_env_var_collection.set_model(CloudEnvVarCollectionModel::new(env_var_collection));
            ctx.emit(CloudModelEvent::ObjectUpdated {
                type_and_id: cloud_env_var_collection.cloud_object_type_and_id(),
                source: UpdateSource::Local,
            });
            ctx.notify();
        }
    }

    pub fn overwrite_workflow_enum(
        &mut self,
        workflow_enum: WorkflowEnum,
        workflow_enum_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(cloud_workflow_enum) = self
            .get_object_of_type_mut::<GenericStringObjectId, CloudWorkflowEnumModel>(
                &workflow_enum_id,
            )
        {
            cloud_workflow_enum.set_model(CloudWorkflowEnumModel::new(workflow_enum));
            ctx.emit(CloudModelEvent::ObjectUpdated {
                type_and_id: cloud_workflow_enum.cloud_object_type_and_id(),
                source: UpdateSource::Local,
            });
            ctx.notify();
        }
    }

    fn set_folder_open_state(
        &mut self,
        folder_id: SyncId,
        open_state: FolderOpenState,
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(folder) = self.get_folder(&folder_id) {
            let is_open = match open_state {
                FolderOpenState::Open => true,
                FolderOpenState::Closed => false,
                FolderOpenState::Reversed => !folder.model().is_open,
            };

            let new_folder = CloudFolder::new(
                folder.id,
                CloudFolderModel {
                    is_open,
                    is_cute_pack: folder.model().is_cute_pack,
                    name: folder.model().name.clone(),
                },
                folder.metadata.clone(),
                folder.permissions.clone(),
            );

            let upsert_event = new_folder.upsert_event();
            self.objects_by_id.insert(folder_id.uid(), Box::new(new_folder));

            if let Some(model_event_sender) = &self.model_event_sender {
                if let Err(e) = model_event_sender.send(upsert_event) {
                    log::error!("Error persisting folder: {e:?}");
                }
            }

            ctx.notify();
        }
    }

    pub fn open_folder(&mut self, folder_id: SyncId, ctx: &mut ModelContext<Self>) {
        self.set_folder_open_state(folder_id, FolderOpenState::Open, ctx)
    }

    pub fn close_folder(&mut self, folder_id: SyncId, ctx: &mut ModelContext<Self>) {
        self.set_folder_open_state(folder_id, FolderOpenState::Closed, ctx)
    }

    pub fn toggle_folder_open(&mut self, folder_id: SyncId, ctx: &mut ModelContext<Self>) {
        self.set_folder_open_state(folder_id, FolderOpenState::Reversed, ctx)
    }

    pub fn collapse_all_in_location(
        &mut self,
        location: CloudObjectLocation,
        index_variant: DriveIndexVariant,
        ctx: &mut ModelContext<Self>,
    ) {
        let mut folder_ids: Vec<SyncId> = Vec::new();
        self.collapse_all_in_location_helper(location, index_variant, &mut folder_ids, ctx);

        folder_ids.iter().for_each(|folder_id| {
            self.set_folder_open_state(*folder_id, FolderOpenState::Closed, ctx)
        });

        ctx.notify();
    }

    fn collapse_all_in_location_helper(
        &self,
        location: CloudObjectLocation,
        index_variant: DriveIndexVariant,
        folder_ids: &mut Vec<SyncId>,
        app: &AppContext,
    ) {
        if let CloudObjectLocation::Folder(folder_id) = location {
            folder_ids.push(folder_id);
        }

        match index_variant {
            DriveIndexVariant::Personal | DriveIndexVariant::Team | DriveIndexVariant::Shared => {
                self
                    .active_cloud_objects_in_location_without_descendents(location, app)
                    .for_each(|object| {
                        let folder: Option<&CloudFolder> = object.into();
                        if let Some(folder) = folder {
                            self.collapse_all_in_location_helper(
                                CloudObjectLocation::Folder(folder.id),
                                index_variant.clone(),
                                folder_ids,
                                app,
                            );
                        }
                    })
            }
        }
    }

    pub fn force_expand_object_and_ancestors(&mut self, id: SyncId, ctx: &mut ModelContext<Self>) {
        let hashed_id = &id.uid();
        if !self.objects_by_id.contains_key(hashed_id) {
            return;
        }

        self.force_expand_object_and_ancestors_internal(id, ctx);
        ctx.emit(CloudModelEvent::ObjectForceExpanded {
            id: hashed_id.clone(),
        });
    }

    fn force_expand_object_and_ancestors_internal(
        &mut self,
        id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(object) = self.objects_by_id.get(&id.uid()) else {
            return;
        };

        let parent_folder_id = object.metadata().folder_id;
        let folder: Option<&CloudFolder> = object.into();

        if let Some(folder) = folder {
            self.set_folder_open_state(folder.id, FolderOpenState::Open, ctx);
        }

        if let Some(parent_folder_id) = parent_folder_id {
            self.force_expand_object_and_ancestors_internal(parent_folder_id, ctx);
        }
    }

    pub fn force_expand_object_and_ancestors_cloud_id(
        &mut self,
        id: CloudObjectTypeAndId,
        ctx: &mut ModelContext<Self>,
    ) {
        match id {
            CloudObjectTypeAndId::Notebook(sync_id) => {
                self.force_expand_object_and_ancestors(sync_id, ctx)
            }
            CloudObjectTypeAndId::Workflow(sync_id) => {
                self.force_expand_object_and_ancestors(sync_id, ctx)
            }
            CloudObjectTypeAndId::Folder(sync_id) => {
                self.force_expand_object_and_ancestors(sync_id, ctx)
            }
            CloudObjectTypeAndId::GenericStringObject { object_type, id } => {
                if matches!(object_type, cute_server_client::cloud_object::GenericStringObjectFormat::Json(cute_server_client::cloud_object::JsonObjectType::EnvVarCollection)) {
                    self.force_expand_object_and_ancestors(id, ctx)
                } else {
                    log::error!("Attempted to force expand an unsupported GenericStringObject type")
                }
            }
            CloudObjectTypeAndId::EnvVarCollection(sync_id) => {
                self.force_expand_object_and_ancestors(sync_id, ctx)
            }
        }
    }

    pub fn delete_object(&mut self, id: SyncId, ctx: &mut ModelContext<Self>) {
        if let Some(object) = self.objects_by_id.remove(&id.uid()) {
            ctx.emit(CloudModelEvent::ObjectDeleted {
                type_and_id: object.cloud_object_type_and_id(),
                folder_id: object.metadata().folder_id,
            });
        }
        ctx.notify();
    }

    pub fn num_unsaved_objects(&self) -> usize {
        self.objects_by_id
            .values()
            .filter(|object| object.metadata().has_pending_content_changes())
            .count()
    }

    pub fn num_unsaved_objects_to_warn_about_before_quitting(&self) -> usize {
        self.objects_by_id
            .values()
            .filter(|object| {
                object.warn_if_unsaved_at_quit() && object.metadata().has_pending_content_changes()
            })
            .count()
    }

    pub fn num_visible_errored_objects(&self) -> usize {
        self.objects_by_id
            .values()
            .filter(|object| object.renders_in_cute_drive() && object.metadata().is_errored())
            .count()
    }

    pub fn has_objects(&self) -> bool {
        !self.objects_by_id.is_empty()
    }

    pub fn has_non_welcome_objects(&self) -> bool {
        self.objects_by_id
            .iter()
            .any(|(_, object)| !object.metadata().is_welcome_object)
    }

    pub fn get_folder_by_uid(&self, uid: &str) -> Option<&CloudFolder> {
        self.objects_by_id.get(uid).and_then(|object| object.into())
    }

    pub fn get_folder(&self, folder_id: &SyncId) -> Option<&CloudFolder> {
        self.objects_by_id
            .get(&folder_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_folder_mut(&mut self, folder_id: &SyncId) -> Option<&mut CloudFolder> {
        self.objects_by_id
            .get_mut(&folder_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_all_exportable_object_ids(&self) -> Vec<CloudObjectTypeAndId> {
        self.objects_by_id
            .values()
            .filter(|object| object.can_export())
            .map(|object| object.cloud_object_type_and_id())
            .collect()
    }

    #[allow(unused)]
    pub fn get_all_active_folders(&self) -> impl Iterator<Item = &CloudFolder> {
        self.objects_by_id
            .values()
            .filter(|object| !object.is_trashed(self))
            .filter_map(|object| object.into())
    }

    pub fn get_all_active_and_inactive_folders(&self) -> impl Iterator<Item = &CloudFolder> {
        self.objects_by_id
            .values()
            .filter_map(|object| object.into())
    }

    pub fn get_workflow(&self, workflow_id: &SyncId) -> Option<&CloudWorkflow> {
        self.objects_by_id
            .get(&workflow_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_workflow_by_uid(&self, uid: &str) -> Option<&CloudWorkflow> {
        self.objects_by_id.get(uid).and_then(|object| object.into())
    }

    pub fn get_workflow_enum(&self, enum_id: &SyncId) -> Option<&CloudWorkflowEnum> {
        self.objects_by_id
            .get(&enum_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_ai_execution_profile(
        &self,
        profile_id: &SyncId,
    ) -> Option<&CloudAIExecutionProfile> {
        self.objects_by_id
            .get(&profile_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_workflow_enum_mut(&mut self, enum_id: &SyncId) -> Option<&mut CloudWorkflowEnum> {
        self.objects_by_id
            .get_mut(&enum_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_workflow_mut(&mut self, workflow_id: &SyncId) -> Option<&mut CloudWorkflow> {
        self.objects_by_id
            .get_mut(&workflow_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_all_active_workflows(&self) -> impl Iterator<Item = &CloudWorkflow> {
        self.objects_by_id
            .values()
            .filter(|object| !object.is_trashed(self))
            .filter_map(|object| object.into())
    }

    pub fn get_all_active_notebooks(&self) -> impl Iterator<Item = &CloudNotebook> {
        self.objects_by_id
            .values()
            .filter(|object| !object.is_trashed(self))
            .filter_map(|object| object.into())
    }

    pub fn get_all_active_and_inactive_workflows(&self) -> impl Iterator<Item = &CloudWorkflow> {
        self.objects_by_id
            .values()
            .filter_map(|object| object.into())
    }

    pub fn get_all_active_and_inactive_workflows_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut CloudWorkflow> {
        self.objects_by_id
            .values_mut()
            .filter_map(|object| object.into())
    }

    pub fn active_workflows_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudWorkflow> + 'a {
        self.active_cloud_objects_in_space(space, app)
            .filter_map(|object| object.into())
    }

    pub fn active_non_welcome_workflows_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudWorkflow> + 'a {
        self.active_non_welcome_cloud_objects_in_space(space, app)
            .filter_map(|object| object.into())
    }

    pub fn active_notebooks_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudNotebook> + 'a {
        self.active_cloud_objects_in_space(space, app)
            .filter_map(|object| object.into())
    }

    pub fn active_non_welcome_notebooks_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudNotebook> + 'a {
        self.active_non_welcome_cloud_objects_in_space(space, app)
            .filter_map(|object| object.into())
    }

    pub fn active_non_welcome_env_var_collections_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudEnvVarCollection> + 'a {
        self.active_non_welcome_cloud_objects_in_space(space, app)
            .filter_map(|object| object.into())
    }

    pub fn workflow_enums_with_owner<'a>(
        &'a self,
        owner: Owner,
        _: &'a AppContext,
    ) -> impl Iterator<Item = &'a CloudWorkflowEnum> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| !object.is_trashed(self) && object.permissions().owner == owner)
            .filter_map(|object| object.into())
    }

    pub fn get_all_cloud_preferences_by_storage_key(&self) -> HashMap<String, &CloudPreference> {
        let mut keys: HashSet<String> = HashSet::new();
        self.get_all_objects_of_type::<GenericStringObjectId, CloudPreferenceModel>()
            .map(|cloud_prefs| {
                if keys.contains(&cloud_prefs.model().string_model.storage_key) {
                    log::warn!(
                        "Duplicate cloud preference storage key: {}",
                        cloud_prefs.model().string_model.storage_key
                    );
                }
                keys.insert(cloud_prefs.model().string_model.storage_key.clone());
                (
                    cloud_prefs.model().string_model.storage_key.clone(),
                    cloud_prefs,
                )
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn get_object_of_type<K, M>(&self, object_id: &SyncId) -> Option<&GenericCloudObject<K, M>>
    where
        K: HashableId + ToServerId + std::fmt::Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        self.objects_by_id
            .get(&object_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_object_of_type_mut<K, M>(
        &mut self,
        object_id: &SyncId,
    ) -> Option<&mut GenericCloudObject<K, M>>
    where
        K: HashableId + ToServerId + std::fmt::Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        self.objects_by_id
            .get_mut(&object_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_all_objects_of_type<K, M>(&self) -> impl Iterator<Item = &GenericCloudObject<K, M>>
    where
        K: HashableId + ToServerId + std::fmt::Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        self.objects_by_id
            .values()
            .filter_map(|object| object.into())
    }

    pub fn get_notebook(&self, notebook_id: &SyncId) -> Option<&CloudNotebook> {
        self.objects_by_id
            .get(&notebook_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_notebook_by_uid(&self, uid: &str) -> Option<&CloudNotebook> {
        self.objects_by_id.get(uid).and_then(|object| object.into())
    }

    pub fn get_notebook_mut(&mut self, notebook_id: &SyncId) -> Option<&mut CloudNotebook> {
        self.objects_by_id
            .get_mut(&notebook_id.uid())
            .and_then(|notebook| notebook.into())
    }

    pub fn get_env_var_collection(
        &self,
        env_var_collection_id: &SyncId,
    ) -> Option<&CloudEnvVarCollection> {
        self.objects_by_id
            .get(&env_var_collection_id.uid())
            .and_then(|object| object.into())
    }

    pub fn get_env_var_collection_by_uid(&self, uid: &str) -> Option<&CloudEnvVarCollection> {
        self.objects_by_id.get(uid).and_then(|object| object.into())
    }

    pub fn get_all_active_env_var_collections(
        &self,
    ) -> impl Iterator<Item = &CloudEnvVarCollection> {
        self.objects_by_id
            .values()
            .filter(|object| !object.is_trashed(self))
            .filter_map(|object| object.into())
    }

    #[cfg(test)]
    pub fn as_cloud_objects(&self) -> impl Iterator<Item = &'_ Box<dyn CloudObject>> {
        self.objects_by_id.values()
    }

    #[cfg(test)]
    pub fn add_object(&mut self, id: SyncId, object: impl CloudObject + 'static) {
        self.objects_by_id.insert(id.uid(), Box::new(object));
    }

    pub fn active_object_uids(&self) -> HashSet<ObjectUid> {
        let mut cache = HashMap::new();
        let mut visiting = HashSet::new();
        let mut active = HashSet::new();
        for uid in self.objects_by_id.keys() {
            if !self.is_trashed_memoized(uid, &mut cache, &mut visiting) {
                active.insert(uid.clone());
            }
        }
        active
    }

    fn is_trashed_memoized(
        &self,
        uid: &str,
        cache: &mut HashMap<String, bool>,
        visiting: &mut HashSet<String>,
    ) -> bool {
        if let Some(&cached) = cache.get(uid) {
            return cached;
        }

        if visiting.contains(uid) {
            return true;
        }

        let result = match self.objects_by_id.get(uid) {
            Some(object) => {
                if object.metadata().trashed_ts.is_some() {
                    true
                } else {
                    match object.metadata().folder_id.map(|parent_id| parent_id.uid()) {
                        Some(parent_uid) => {
                            visiting.insert(uid.to_owned());
                            let r = self.is_trashed_memoized(&parent_uid, cache, visiting);
                            visiting.remove(uid);
                            r
                        }
                        None => false,
                    }
                }
            }
            None => false,
        };

        cache.insert(uid.to_owned(), result);
        result
    }

    pub fn active_cloud_objects_in_location_without_descendents<'a>(
        &'a self,
        location: CloudObjectLocation,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| {
                !object.is_trashed(self) && object.location(self, app) == location
            })
            .map(|object| object.as_ref())
    }

    pub fn trashed_cloud_objects_in_location_without_descendents<'a>(
        &'a self,
        location: CloudObjectLocation,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| object.is_trashed(self) && object.location(self, app) == location)
            .map(|object| object.as_ref())
    }

    pub fn trashed_cloud_object_types_in_location_with_descendants(
        &self,
        location: CloudObjectLocation,
        app: &AppContext,
    ) -> Vec<ObjectType> {
        let mut trashed_objects: Vec<ObjectType> = Vec::new();
        self.trashed_cloud_object_types_in_location_with_descendants_helper(
            location,
            &mut trashed_objects,
            app,
        );
        trashed_objects
    }

    fn trashed_cloud_object_types_in_location_with_descendants_helper(
        &self,
        location: CloudObjectLocation,
        trashed_objects: &mut Vec<ObjectType>,
        app: &AppContext,
    ) {
        self.trashed_cloud_objects_in_location_without_descendents(location, app)
            .for_each(|object| {
                trashed_objects.push(object.object_type());
                let folder: Option<&CloudFolder> = object.into();
                if let Some(folder) = folder {
                    self.trashed_cloud_object_types_in_location_with_descendants_helper(
                        CloudObjectLocation::Folder(folder.id),
                        trashed_objects,
                        app,
                    );
                }
            });
    }

    pub fn indirectly_trashed_cloud_objects_in_location_without_descendents<'a>(
        &'a self,
        location: CloudObjectLocation,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> {
        self.objects_by_id
            .values()
            .filter(move |object| {
                object.is_trashed(self)
                    && object.location(self, app) == location
                    && object.metadata().trashed_ts.is_none()
            })
            .map(|object| object.as_ref())
    }

    pub fn active_cloud_objects_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| object.is_in_space(space, app) && !object.is_trashed(self))
            .map(|object| object.as_ref())
    }

    pub fn active_non_welcome_cloud_objects_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| {
                object.is_in_space(space, app)
                    && !object.is_trashed(self)
                    && !object.is_welcome_object()
            })
            .map(|object| object.as_ref())
    }

    pub fn all_cloud_objects_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| object.is_in_space(space, app))
            .map(|object| object.as_ref())
    }

    pub fn trashed_cloud_objects_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> + 'a {
        self.objects_by_id
            .values()
            .filter(move |object| object.is_in_space(space, app) && object.is_trashed(self))
            .map(|object| object.as_ref())
    }

    pub fn directly_trashed_cloud_objects_in_space<'a>(
        &'a self,
        space: Space,
        app: &'a AppContext,
    ) -> impl Iterator<Item = &'a dyn CloudObject> {
        self.objects_by_id
            .values()
            .filter(move |object| {
                object.is_in_space(space, app) && object.metadata().trashed_ts.is_some()
            })
            .map(|object| object.as_ref())
    }

    pub fn num_active_cloud_objects_per_space<'a, I>(
        &self,
        spaces: I,
        app: &AppContext,
    ) -> HashMap<Space, usize>
    where
        I: Iterator<Item = &'a Space>,
    {
        spaces
            .map(|space| {
                (
                    *space,
                    self.active_cloud_objects_in_space(*space, app).count(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    pub fn num_trashed_cloud_objects_per_space<'a, I>(
        &self,
        spaces: I,
        app: &AppContext,
    ) -> HashMap<Space, usize>
    where
        I: Iterator<Item = &'a Space>,
    {
        spaces
            .map(|space| {
                (
                    *space,
                    self.trashed_cloud_objects_in_space(*space, app).count(),
                )
            })
            .collect::<HashMap<_, _>>()
    }

    #[cfg(test)]
    pub fn mock(_ctx: &mut ModelContext<Self>) -> Self {
        Self::new(None, Vec::new(), None)
    }

    pub fn reset(&mut self) {
        self.objects_by_id = HashMap::new();
    }
}

impl Entity for CloudModel {
    type Event = CloudModelEvent;
}

impl SingletonEntity for CloudModel {}



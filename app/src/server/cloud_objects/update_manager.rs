use std::collections::HashSet;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use chrono::{DateTime, Utc};
#[cfg(test)]
pub use cloud_object_client::GetCloudObjectResponse;
pub use cloud_object_client::InitialLoadResponse;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use warp_core::report_error;
use warp_graphql::scalars::time::ServerTimestamp;
use warp_util::sync::Condition;
use warpui::r#async::FutureId;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::ambient_agents::scheduled::CloudScheduledAmbientAgentModel;
use crate::ai::ambient_agents::AmbientAgentTaskId;
use crate::ai::blocklist::BlocklistAIHistoryModel;
use crate::ai::cloud_environments::CloudAmbientAgentEnvironmentModel;
use crate::ai::execution_profiles::profiles::AIExecutionProfilesModel;
use crate::ai::execution_profiles::CloudAIExecutionProfileModel;
use crate::ai::facts::{AIFact, CloudAIFactModel};
#[cfg(not(target_family = "wasm"))]
use crate::ai::mcp::templatable::{CloudTemplatableMCPServerModel, TemplatableMCPServer};
use crate::auth::auth_manager::AuthManager;
use crate::auth::AuthStateProvider;
use crate::cloud_object::model::actions::{
    ObjectAction, ObjectActionHistory, ObjectActionType, ObjectActions,
};
use crate::cloud_object::model::generic_string_model::{
    GenericStringModel, GenericStringObjectId, Serializer, StringModel,
};
use crate::cloud_object::model::persistence::{CloudModel, CloudModelEvent, UpdateSource};
use crate::cloud_object::model::view::{CloudViewModel, Editor, EditorState};
use crate::cloud_object::{
    CloudModelType, CloudObject, CloudObjectEventEntrypoint, CloudObjectSyncStatus,
    CreateCloudObjectResult, CreateObjectRequest, GenericCloudObject, GenericServerObject,
    GenericStringObjectFormat, JsonObjectType, NumInFlightRequests, ObjectIdType, ObjectType, Owner,
    Revision, RevisionAndLastEditor, ServerAIExecutionProfile, ServerAIFact, ServerAmbientAgentEnvironment,
    ServerCloudAgentConfig, ServerCloudObject, ServerEnvVarCollection, ServerMCPServer,
    ServerMetadata, ServerPermissions, ServerPreference, ServerScheduledAmbientAgent,
    ServerTemplatableMCPServer, ServerWorkflowEnum, Space, UpdateCloudObjectResult,
};
use crate::drive::folders::{CloudFolderModel, FolderId};
use crate::env_vars::{CloudEnvVarCollectionModel, EnvVarCollection};
use crate::notebooks::{CloudNotebookModel, NotebookId};
use crate::persistence::ModelEvent;
use crate::server::ids::{ClientId, HashableId, HashedSqliteId, ObjectUid, ServerId, SyncId, ToServerId};
use crate::server::server_api::object::ObjectClient;
use crate::server::sync_queue::{
    CreationFailureReason, GenericStringObjectToCreate, QueueItem, SyncQueue, SyncQueueEvent,
};
use crate::workflows::workflow::Workflow;
use crate::workflows::workflow_enum::{CloudWorkflowEnum, CloudWorkflowEnumModel, WorkflowEnum};
use crate::workflows::{CloudWorkflowModel, WorkflowId};
use crate::workspaces::user_profiles::UserProfileWithUID;
use crate::workspaces::user_profiles::UserProfiles;
use crate::workspaces::user_workspaces::UserWorkspaces;

lazy_static! {
    static ref DUPLICATE_OBJECT_NAME_REGEX: Regex = Regex::new(r" \((\d+)\)$").expect("regex should not fail to compile");
}

#[derive(Debug, PartialEq)]
pub enum OperationSuccessType {
    Success,
    Failure,
    Rejection,
    Denied(String),
    FeatureNotAvailable,
}

#[derive(Debug, PartialEq)]
pub enum ObjectOperation {
    Create { initiated_by: InitiatedBy },
    Update,
    Trash,
    Untrash,
    Delete { initiated_by: InitiatedBy },
    MoveToFolder,
    MoveToDrive,
    Leave,
    TakeEditAccess,
    UpdatePermissions,
    EmptyTrash,
}

#[derive(Debug)]
pub struct ObjectOperationResult {
    pub success_type: OperationSuccessType,
    pub operation: ObjectOperation,
    pub client_id: Option<ClientId>,
    pub server_id: Option<ServerId>,
    pub num_objects: Option<i32>,
}

#[derive(Debug)]
pub enum UpdateManagerEvent {
    ObjectOperationComplete {
        result: ObjectOperationResult,
    },
    AmbientTaskUpdated,
    MCPGalleryUpdated,
}

/// An enum for choosing the behavior of the fetch_single_cloud_object function.
pub enum FetchSingleObjectOption {
    /// Perform the normal upsert behavior.
    None,
    /// Perform the normal upsert behavior, but additionally force overwrite the
    /// in-memory object to whatever the server object is.
    ForceOverwrite,
    /// Only perform the normal upsert behavior if the object doesn't already
    /// exist in-memory.
    IgnoreIfExists,
}

/// An enum that defines whether the action was initiated by the user or the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitiatedBy {
    User,
    System,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GenericStringObjectInput<T, S>
where
    T: StringModel<
            CloudObjectType = GenericCloudObject<GenericStringObjectId, GenericStringModel<T, S>>,
        > + 'static,
    S: Serializer<T> + 'static,
{
    pub id: ClientId,
    pub model: GenericStringModel<T, S>,
    pub initial_folder_id: Option<SyncId>,
    pub entrypoint: CloudObjectEventEntrypoint,
}

/// Simplified UpdateManager for local-only operations.
/// Removed: polling, server message handling, sharing, object moving across spaces.
pub struct UpdateManager {
    model_event_sender: Option<SyncSender<ModelEvent>>,
    object_client: Arc<dyn ObjectClient>,
    spawned_futures: Vec<FutureId>,
    has_initial_load: Condition,
}

impl UpdateManager {
    pub fn new(
        model_event_sender: Option<SyncSender<ModelEvent>>,
        object_client: Arc<dyn ObjectClient>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let sync_queue = SyncQueue::handle(ctx);
        ctx.subscribe_to_model(&sync_queue, |me, event, ctx| {
            me.handle_model_event(event, ctx);
        });

        Self {
            model_event_sender,
            object_client,
            spawned_futures: Default::default(),
            has_initial_load: Condition::new(),
        }
    }

    #[cfg(test)]
    pub fn mock(ctx: &mut ModelContext<Self>) -> Self {
        use crate::server::server_api::ServerApiProvider;

        Self::new(
            None,
            ServerApiProvider::new_for_test().get_cloud_objects_client(),
            ctx,
        )
    }

    #[cfg(any(test, feature = "integration_tests"))]
    pub fn spawned_futures(&self) -> &[FutureId] {
        &self.spawned_futures
    }

    fn save_to_db(&self, events: impl IntoIterator<Item = ModelEvent>) {
        let model_event_sender = self.model_event_sender.clone();
        if let Some(model_event_sender) = &model_event_sender {
            for event in events {
                if let Err(e) = model_event_sender.send(event) {
                    log::error!("Error saving to database: {e:?}");
                }
            }
        }
    }

    fn handle_model_event(&mut self, event: &SyncQueueEvent, ctx: &mut ModelContext<Self>) {
        match event {
            SyncQueueEvent::ObjectCreationSuccessful {
                server_creation_info,
                client_id,
                revision_and_editor,
                metadata_ts,
                initiated_by,
            } => {
                let server_id = &server_creation_info.server_id_and_type.id;

                // Update server ID in sqlite.
                self.save_to_db([ModelEvent::UpdateObjectAfterServerCreation {
                    client_id: client_id.sqlite_hash(),
                    server_creation_info: server_creation_info.clone(),
                }]);

                // Update in-memory model.
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    cloud_model.update_object_after_server_creation(
                        *client_id,
                        server_creation_info.clone(),
                        ctx,
                    );
                    if let Some(object) = cloud_model.get_mut_by_uid(&server_id.uid()) {
                        let is_no_longer_in_flight = {
                            let status_if_no_reqs = CloudObjectSyncStatus::NoLocalChanges;
                            object.decrement_in_flight_request_count(status_if_no_reqs)
                        };

                        if is_no_longer_in_flight {
                            self.save_to_db([ModelEvent::MarkObjectAsSynced {
                                hashed_sqlite_id: server_creation_info
                                    .server_id_and_type
                                    .sqlite_type_and_uid_hash(),
                                revision_and_editor: revision_and_editor.clone(),
                                metadata_ts: Some(*metadata_ts),
                            }]);
                        }

                        ctx.notify();
                    }

                    cloud_model.set_latest_revision_and_editor(
                        &server_id.uid(),
                        revision_and_editor.clone(),
                        ctx,
                    );

                    cloud_model.update_object_metadata_last_updated_ts(
                        &server_id.uid(),
                        *metadata_ts,
                        ctx,
                    );
                });

                // Delete the actions on the client ID.
                ObjectActions::handle(ctx).update(ctx, |object_actions, ctx| {
                    object_actions.delete_actions_for_object(&client_id.to_string(), ctx);
                });
                self.sync_actions_for_objects_to_sqlite(vec![&client_id.to_string()], ctx);

                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Success,
                        operation: ObjectOperation::Create {
                            initiated_by: *initiated_by,
                        },
                        client_id: Some(*client_id),
                        server_id: Some(*server_id),
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectUpdateSuccessful {
                server_id,
                revision_and_editor,
            } => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    cloud_model.set_latest_revision_and_editor(
                        &server_id.uid(),
                        revision_and_editor.clone(),
                        ctx,
                    );
                    cloud_model.check_and_maybe_clear_current_conflict(&server_id.uid(), ctx);

                    if let Some(object) = cloud_model.get_mut_by_uid(&server_id.uid()) {
                        let is_no_longer_in_flight = {
                            object.decrement_in_flight_request_count(
                                CloudObjectSyncStatus::NoLocalChanges,
                            )
                        };

                        if is_no_longer_in_flight {
                            self.save_to_db([ModelEvent::MarkObjectAsSynced {
                                hashed_sqlite_id: server_id
                                    .sqlite_type_and_uid_hash(object.object_type().into()),
                                revision_and_editor: revision_and_editor.clone(),
                                metadata_ts: None,
                            }]);
                        }

                        ctx.notify();
                    }
                });

                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Success,
                        operation: ObjectOperation::Update,
                        client_id: None,
                        server_id: Some(*server_id),
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectCreationFailure {
                reason: CreationFailureReason::UniqueKeyConflict { id, initiated_by },
            } => {
                self.handle_failure_response(id, true, ctx);
                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Failure,
                        operation: ObjectOperation::Create {
                            initiated_by: *initiated_by,
                        },
                        client_id: ClientId::from_hash(id),
                        server_id: None,
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectCreationFailure {
                reason: CreationFailureReason::Other { id, initiated_by },
            } => {
                self.handle_failure_response(id, false, ctx);
                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Failure,
                        operation: ObjectOperation::Create {
                            initiated_by: *initiated_by,
                        },
                        client_id: ClientId::from_hash(id),
                        server_id: None,
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectCreationFailure {
                reason:
                    CreationFailureReason::Denied {
                        message,
                        client_id,
                        initiated_by,
                    },
            } => {
                self.handle_creation_denied_response(client_id, ctx);
                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Denied(message.to_string()),
                        operation: ObjectOperation::Create {
                            initiated_by: *initiated_by,
                        },
                        client_id: Some(*client_id),
                        server_id: None,
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectUpdateFailure { id } => {
                self.handle_failure_response(&id.uid(), false, ctx);
                match id {
                    SyncId::ClientId(id) => ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Update,
                            client_id: Some(*id),
                            server_id: None,
                            num_objects: None,
                        },
                    }),
                    SyncId::ServerId(id) => ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Update,
                            client_id: None,
                            server_id: Some(*id),
                            num_objects: None,
                        },
                    }),
                }
            }
            SyncQueueEvent::ObjectUpdateRejected {
                id,
                object: conflicting_object,
            } => {
                self.handle_conflicting_object(conflicting_object, id, ctx);
                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::Rejection,
                        operation: ObjectOperation::Update,
                        client_id: None,
                        server_id: Some(ServerId::from_string_lossy(id)),
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ObjectUpdateFeatureNotAvailable { id } => {
                ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                    result: ObjectOperationResult {
                        success_type: OperationSuccessType::FeatureNotAvailable,
                        operation: ObjectOperation::Update,
                        client_id: None,
                        server_id: Some(ServerId::from_string_lossy(id)),
                        num_objects: None,
                    },
                });
            }
            SyncQueueEvent::ReportObjectActionFailed {
                uid,
                action_timestamp,
            } => {
                self.remove_pending_object_action(uid, action_timestamp, ctx);
                self.sync_actions_for_objects_to_sqlite(vec![uid], ctx);
            }
            SyncQueueEvent::ReportObjectActionSucceeded {
                uid,
                action_timestamp,
                action_history,
            } => {
                self.remove_pending_object_action(uid, action_timestamp, ctx);
                self.maybe_overwrite_object_action_history(action_history, ctx);
                self.sync_actions_for_objects_to_sqlite(vec![uid], ctx);
            }
        }
    }

    fn handle_creation_denied_response(&self, client_id: &ClientId, ctx: &mut ModelContext<Self>) {
        let uid = client_id.to_string();

        let in_personal_drive = CloudModel::handle(ctx).read(ctx, |cloud_model, ctx| {
            cloud_model
                .get_by_uid(&uid)
                .is_none_or(|object| object.space(ctx) == Space::Personal)
        });

        if !in_personal_drive {
            CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                let personal_drive = UserWorkspaces::as_ref(ctx).personal_drive(ctx);
                cloud_model.update_object_location(&uid, personal_drive, None, ctx);
            });

            let cloud_model = CloudModel::as_ref(ctx);
            if let Some(cloud_object) = cloud_model.get_by_uid(&uid) {
                self.save_to_db([cloud_object.upsert_event()]);
            }

            CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                if let Some(object) = cloud_model.get_mut_by_uid(&uid) {
                    let queue_item = object
                        .create_object_queue_item(
                            CloudObjectEventEntrypoint::default(),
                            InitiatedBy::User,
                        )
                        .unwrap_or(object.update_object_queue_item(None));
                    SyncQueue::handle(ctx).update(ctx, |sync_queue, ctx| {
                        sync_queue.enqueue(queue_item, ctx);
                    });
                }
            });
        } else {
            self.handle_failure_response(&uid, false, ctx);
        }
    }

    fn handle_failure_response(
        &self,
        uid: &ObjectUid,
        unique_key_creation_conflict: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let mut hashed_sqlite_id = None;
        if let Some((sync_id, object_type)) =
            CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                if let Some(object) = cloud_model.get_mut_by_uid(uid) {
                    if unique_key_creation_conflict && object.should_clear_on_unique_key_conflict()
                    {
                        return Some((object.sync_id(), object.object_type()));
                    } else {
                        object.decrement_in_flight_request_count(CloudObjectSyncStatus::Errored);
                        hashed_sqlite_id = Some(object.hashed_sqlite_id());
                    }
                }
                ctx.notify();
                None
            })
        {
            CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                log::info!("Removing object {sync_id:?} after unique key conflict");
                cloud_model.delete_object(sync_id, ctx);
                ctx.notify();
            });

            ObjectActions::handle(ctx).update(ctx, |object_actions, ctx| {
                object_actions.delete_actions_for_object(uid, ctx);
            });

            self.save_to_db([ModelEvent::DeleteObjects {
                ids: vec![(sync_id, object_type.into())],
            }]);
            ctx.notify();
        }

        if let Some(hashed_sqlite_id) = hashed_sqlite_id {
            self.save_to_db([ModelEvent::IncrementRetryCount(hashed_sqlite_id.to_owned())]);
        }
    }

    fn handle_conflicting_object(
        &self,
        conflicting_object: &Arc<ServerCloudObject>,
        uid: &ObjectUid,
        ctx: &mut ModelContext<Self>,
    ) {
        match conflicting_object.as_ref() {
            ServerCloudObject::Notebook(server_notebook) => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    if let Some(notebook) = cloud_model.get_notebook_mut(&server_notebook.id) {
                        notebook.set_conflicting_object(Arc::new(server_notebook.clone()));
                        notebook
                            .set_pending_content_changes_status(CloudObjectSyncStatus::InConflict);
                        ctx.notify();
                    }
                });
            }
            ServerCloudObject::Workflow(workflow) => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    cloud_model.overwrite_workflow(workflow.clone().model.data, workflow.id, ctx);
                    let workflow_metadata = workflow.clone().metadata;
                    cloud_model.set_latest_revision_and_editor(
                        uid,
                        RevisionAndLastEditor {
                            revision: workflow_metadata.revision,
                            last_editor_uid: workflow_metadata.last_editor_uid,
                        },
                        ctx,
                    );
                    if let Some(object) = cloud_model.get_mut_by_uid(uid) {
                        object.decrement_in_flight_request_count(
                            CloudObjectSyncStatus::NoLocalChanges,
                        );
                        ctx.notify();
                    }
                });

                let cloud_model = CloudModel::as_ref(ctx);
                if let Some(workflow) = cloud_model.get_workflow(&workflow.id) {
                    self.save_to_db([ModelEvent::UpsertWorkflow {
                        workflow: workflow.clone(),
                    }]);
                }
            }
            ServerCloudObject::EnvVarCollection(env_var_collection) => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    cloud_model.overwrite_env_var_collection(
                        env_var_collection.clone().model.string_model,
                        env_var_collection.id,
                        ctx,
                    );
                    let env_var_collection_metadata = env_var_collection.clone().metadata;
                    cloud_model.set_latest_revision_and_editor(
                        uid,
                        RevisionAndLastEditor {
                            revision: env_var_collection_metadata.revision,
                            last_editor_uid: env_var_collection_metadata.last_editor_uid,
                        },
                        ctx,
                    );
                    if let Some(object) = cloud_model.get_mut_by_uid(uid) {
                        object.decrement_in_flight_request_count(
                            CloudObjectSyncStatus::NoLocalChanges,
                        );
                        ctx.notify();
                    }
                });

                let cloud_model = CloudModel::as_ref(ctx);
                if let Some(env_var_collection) = cloud_model
                    .get_object_of_type::<GenericStringObjectId, CloudEnvVarCollectionModel>(
                        &env_var_collection.id,
                    )
                {
                    self.save_to_db([ModelEvent::UpsertGenericStringObject {
                        object: Box::new(env_var_collection.clone()),
                    }]);
                }
            }
            ServerCloudObject::WorkflowEnum(workflow_enum) => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    cloud_model.overwrite_workflow_enum(
                        workflow_enum.clone().model.string_model,
                        workflow_enum.id,
                        ctx,
                    );
                    let workflow_enum_metadata = workflow_enum.clone().metadata;
                    cloud_model.set_latest_revision_and_editor(
                        uid,
                        RevisionAndLastEditor {
                            revision: workflow_enum_metadata.revision,
                            last_editor_uid: workflow_enum_metadata.last_editor_uid,
                        },
                        ctx,
                    );
                    if let Some(object) = cloud_model.get_mut_by_uid(uid) {
                        object.decrement_in_flight_request_count(
                            CloudObjectSyncStatus::NoLocalChanges,
                        );
                        ctx.notify();
                    }
                });

                let cloud_model = CloudModel::as_ref(ctx);
                if let Some(workflow_enum) = cloud_model
                    .get_object_of_type::<GenericStringObjectId, CloudWorkflowEnumModel>(
                        &workflow_enum.id,
                    )
                {
                    self.save_to_db([ModelEvent::UpsertGenericStringObject {
                        object: Box::new(workflow_enum.clone()),
                    }]);
                }
            }
            ServerCloudObject::AIExecutionProfile(server_profile) => {
                CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                    if let Some(profile) = cloud_model.get_object_of_type_mut(&server_profile.id) {
                        profile.set_conflicting_object(Arc::new(server_profile.clone()));
                        profile
                            .set_pending_content_changes_status(CloudObjectSyncStatus::InConflict);
                        ctx.notify();
                    }
                });
            }
            ServerCloudObject::Folder(_)
            | ServerCloudObject::Preference(_)
            | ServerCloudObject::AIFact(_)
            | ServerCloudObject::MCPServer(_)
            | ServerCloudObject::TemplatableMCPServer(_)
            | ServerCloudObject::AmbientAgentEnvironment(_)
            | ServerCloudObject::ScheduledAmbientAgent(_)
            | ServerCloudObject::CloudAgentConfig(_) => {}
        }
    }

    /// Replace an object's data with the conflicting version from the server.
    pub fn replace_object_with_conflict(&mut self, uid: &ObjectUid, ctx: &mut ModelContext<Self>) {
        let cloud_model_handle = CloudModel::handle(ctx);

        let had_conflicts = cloud_model_handle.update(ctx, |cloud_model, ctx| {
            match cloud_model.get_mut_by_uid(uid) {
                Some(object) if object.has_conflicting_changes() => {
                    object.replace_object_with_conflict();
                    ctx.emit(CloudModelEvent::ObjectUpdated {
                        type_and_id: object.cloud_object_type_and_id(),
                        source: UpdateSource::Server,
                    });
                    true
                }
                _ => false,
            }
        });

        if had_conflicts {
            self.save_in_memory_object_to_sqlite(cloud_model_handle.as_ref(ctx), uid);
        }
    }

    pub fn update_ai_fact(
        &mut self,
        ai_fact: AIFact,
        ai_fact_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(CloudAIFactModel::new(ai_fact), ai_fact_id, revision_ts, ctx);
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn update_templatable_mcp_server(
        &mut self,
        templatable_mcp_server: TemplatableMCPServer,
        templatable_mcp_server_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudTemplatableMCPServerModel::new(templatable_mcp_server),
            templatable_mcp_server_id,
            revision_ts,
            ctx,
        );
    }

    pub fn update_workflow(
        &mut self,
        workflow: Workflow,
        workflow_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudWorkflowModel::new(workflow),
            workflow_id,
            revision_ts,
            ctx,
        );
    }

    pub fn update_workflow_enum(
        &mut self,
        workflow_enum: WorkflowEnum,
        workflow_enum_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudWorkflowEnumModel::new(workflow_enum),
            workflow_enum_id,
            revision_ts,
            ctx,
        );
    }

    pub fn update_env_var_collection(
        &mut self,
        env_var_collection: EnvVarCollection,
        env_var_collection_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudEnvVarCollectionModel::new(env_var_collection),
            env_var_collection_id,
            revision_ts,
            ctx,
        );
    }

    pub fn update_ambient_agent_environment(
        &mut self,
        environment: crate::ai::cloud_environments::AmbientAgentEnvironment,
        environment_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudAmbientAgentEnvironmentModel::new(environment),
            environment_id,
            revision_ts,
            ctx,
        );
    }

    pub fn update_notebook_data(
        &mut self,
        data: Arc<String>,
        notebook_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        let cloud_model = CloudModel::as_ref(ctx);
        let revision = cloud_model.current_revision(&notebook_id).cloned();
        if let Some(notebook) = cloud_model.get_notebook(&notebook_id) {
            let new_notebook = CloudNotebookModel {
                title: notebook.model().title.to_owned(),
                data: data.to_string(),
                ai_document_id: notebook.model().ai_document_id,
                conversation_id: notebook.model().conversation_id.clone(),
            };
            self.update_object(new_notebook, notebook_id, revision, ctx);
        } else {
            log::warn!("Expected notebook to be in model with id {notebook_id:?}");
        }
    }

    pub fn update_notebook_title(
        &mut self,
        title: Arc<String>,
        notebook_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        let cloud_model = CloudModel::as_ref(ctx);
        let revision = cloud_model.current_revision(&notebook_id).cloned();
        if let Some(notebook) = cloud_model.get_notebook(&notebook_id) {
            let new_notebook = CloudNotebookModel {
                title: title.to_string(),
                data: notebook.model().data.to_owned(),
                ai_document_id: notebook.model().ai_document_id,
                conversation_id: notebook.model().conversation_id.clone(),
            };
            self.update_object(new_notebook, notebook_id, revision, ctx);
        } else {
            log::warn!("Expected notebook to be in model with id {notebook_id:?}");
        }
    }

    pub fn duplicate_object(
        &mut self,
        cloud_object_type_and_id: &crate::drive::CloudObjectTypeAndId,
        ctx: &mut ModelContext<Self>,
    ) {
        use crate::drive::CloudObjectTypeAndId;
        match cloud_object_type_and_id {
            CloudObjectTypeAndId::Notebook(notebook_id) => {
                self.duplicate_object_internal::<NotebookId, CloudNotebookModel>(notebook_id, ctx);
            }
            CloudObjectTypeAndId::Workflow(workflow_id) => {
                self.duplicate_object_internal::<WorkflowId, CloudWorkflowModel>(workflow_id, ctx);
            }
            CloudObjectTypeAndId::GenericStringObject { object_type, id } => {
                if let GenericStringObjectFormat::Json(JsonObjectType::EnvVarCollection) =
                    object_type
                {
                    self.duplicate_object_internal::<GenericStringObjectId, CloudEnvVarCollectionModel>(
                        id, ctx,
                    );
                } else {
                    log::error!("Tried to duplicate an unsupported type: json object");
                }
            }
            CloudObjectTypeAndId::Folder(_) => {
                log::error!("Tried to duplicate an unsupported type: folder");
            }
        }
    }

    fn duplicate_object_internal<K, M>(&mut self, id: &SyncId, ctx: &mut ModelContext<Self>)
    where
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
        let (duplicate_model, client_id, owner, initial_folder_id, entrypoint) = {
            let cloud_model = CloudModel::as_ref(ctx);
            let object: GenericCloudObject<K, M> = cloud_model
                .get_object_of_type(id)
                .expect("object should exist in order to be duplicated")
                .clone();
            let client_id = ClientId::new();
            let owner = object.permissions.owner;
            let initial_folder_id = object.metadata.folder_id;
            let entrypoint = CloudObjectEventEntrypoint::Unknown;
            let mut duplicate_model = object.model().clone();
            let duplicate_name =
                self.get_next_duplicate_object_name(&object as &dyn CloudObject, cloud_model, ctx);
            duplicate_model.set_display_name(&duplicate_name);
            (
                duplicate_model,
                client_id,
                owner,
                initial_folder_id,
                entrypoint,
            )
        };
        self.create_object(
            duplicate_model,
            owner,
            client_id,
            entrypoint,
            true,
            initial_folder_id,
            InitiatedBy::User,
            ctx,
        );
    }

    pub fn create_ai_fact(
        &mut self,
        ai_fact: AIFact,
        client_id: ClientId,
        owner: Owner,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudAIFactModel::new(ai_fact),
            owner,
            client_id,
            Default::default(),
            false,
            None,
            InitiatedBy::User,
            ctx,
        );
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn create_templatable_mcp_server(
        &mut self,
        templatable_mcp_server: TemplatableMCPServer,
        client_id: ClientId,
        owner: Owner,
        initiated_by: InitiatedBy,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudTemplatableMCPServerModel::new(templatable_mcp_server),
            owner,
            client_id,
            Default::default(),
            false,
            None,
            initiated_by,
            ctx,
        );
    }

    #[cfg_attr(target_family = "wasm", allow(dead_code))]
    pub fn create_ambient_agent_environment(
        &mut self,
        ambient_agent_environment: crate::ai::cloud_environments::AmbientAgentEnvironment,
        client_id: ClientId,
        owner: Owner,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudAmbientAgentEnvironmentModel::new(ambient_agent_environment),
            owner,
            client_id,
            Default::default(),
            false,
            None,
            InitiatedBy::User,
            ctx,
        )
    }

    #[allow(dead_code)]
    pub fn create_ai_execution_profile(
        &mut self,
        ai_execution_profile: crate::ai::execution_profiles::AIExecutionProfile,
        client_id: ClientId,
        owner: Owner,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudAIExecutionProfileModel::new(ai_execution_profile),
            owner,
            client_id,
            Default::default(),
            false,
            None,
            InitiatedBy::User,
            ctx,
        );
    }

    #[allow(dead_code)]
    pub fn update_ai_execution_profile(
        &mut self,
        ai_execution_profile: crate::ai::execution_profiles::AIExecutionProfile,
        ai_execution_profile_id: SyncId,
        revision_ts: Option<Revision>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.update_object(
            CloudAIExecutionProfileModel::new(ai_execution_profile),
            ai_execution_profile_id,
            revision_ts,
            ctx,
        );
    }

    pub fn delete_ai_execution_profile(
        &mut self,
        ai_execution_profile_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        self.delete_object_by_user(
            crate::drive::CloudObjectTypeAndId::GenericStringObject {
                object_type: GenericStringObjectFormat::Json(JsonObjectType::AIExecutionProfile),
                id: ai_execution_profile_id,
            },
            ctx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_notebook(
        &mut self,
        client_id: ClientId,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
        model: CloudNotebookModel,
        entrypoint: CloudObjectEventEntrypoint,
        force_expand: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let count = CloudModel::handle(ctx).read(ctx, |model, ctx| {
            model
                .active_non_welcome_notebooks_in_space(Space::Personal, ctx)
                .count()
        });
        if AuthStateProvider::handle(ctx).read(ctx, |auth_state_provider, _ctx| {
            auth_state_provider
                .get()
                .is_anonymous_user_past_object_limit(ObjectType::Notebook, count + 1)
                .unwrap_or_default()
        }) {
            AuthManager::handle(ctx).update(ctx, |auth_manager: &mut AuthManager, ctx| {
                auth_manager.anonymous_user_hit_drive_object_limit(ctx);
            });
            return;
        };

        self.create_object(
            model,
            owner,
            client_id,
            entrypoint,
            force_expand,
            initial_folder_id,
            InitiatedBy::User,
            ctx,
        );
    }

    fn get_next_duplicate_object_name(
        &self,
        original_cloud_object: &dyn CloudObject,
        cloud_model: &CloudModel,
        app: &AppContext,
    ) -> String {
        let original_name = original_cloud_object.display_name();

        let same_type_and_folder_names = cloud_model
            .active_cloud_objects_in_location_without_descendents(
                original_cloud_object.location(cloud_model, app),
                app,
            )
            .filter(|&object| object.object_type() == original_cloud_object.object_type())
            .map(|object| object.display_name())
            .collect::<HashSet<String>>();

        let mut duplicate_name = get_duplicate_object_name(&original_name);
        while same_type_and_folder_names.contains(&duplicate_name) {
            duplicate_name = get_duplicate_object_name(&duplicate_name);
        }
        duplicate_name
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_workflow(
        &mut self,
        workflow: Workflow,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
        client_id: ClientId,
        entrypoint: CloudObjectEventEntrypoint,
        force_expand: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let count = CloudModel::handle(ctx).read(ctx, |model, ctx| {
            model
                .active_non_welcome_workflows_in_space(Space::Personal, ctx)
                .count()
        });
        if AuthStateProvider::handle(ctx).read(ctx, |auth_state_provider, _ctx| {
            auth_state_provider
                .get()
                .is_anonymous_user_past_object_limit(ObjectType::Workflow, count + 1)
                .unwrap_or_default()
        }) {
            AuthManager::handle(ctx).update(ctx, |auth_manager: &mut AuthManager, ctx| {
                auth_manager.anonymous_user_hit_drive_object_limit(ctx);
            });
            return;
        };

        self.create_object(
            CloudWorkflowModel::new(workflow),
            owner,
            client_id,
            entrypoint,
            force_expand,
            initial_folder_id,
            InitiatedBy::User,
            ctx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_workflow_enum(
        &mut self,
        workflow_enum: WorkflowEnum,
        owner: Owner,
        client_id: ClientId,
        entrypoint: CloudObjectEventEntrypoint,
        force_expand: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudWorkflowEnumModel::new(workflow_enum),
            owner,
            client_id,
            entrypoint,
            force_expand,
            None,
            InitiatedBy::User,
            ctx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_env_var_collection(
        &mut self,
        client_id: ClientId,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
        model: CloudEnvVarCollectionModel,
        entrypoint: CloudObjectEventEntrypoint,
        force_expand: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let count = CloudModel::handle(ctx).read(ctx, |model, ctx| {
            model
                .active_non_welcome_env_var_collections_in_space(Space::Personal, ctx)
                .count()
        });
        let env_var_collection_type = ObjectType::GenericStringObject(
            GenericStringObjectFormat::Json(JsonObjectType::EnvVarCollection),
        );
        if AuthStateProvider::handle(ctx).read(ctx, |auth_state_provider, _ctx| {
            auth_state_provider
                .get()
                .is_anonymous_user_past_object_limit(env_var_collection_type, count + 1)
                .unwrap_or_default()
        }) {
            AuthManager::handle(ctx).update(ctx, |auth_manager: &mut AuthManager, ctx| {
                auth_manager.anonymous_user_hit_drive_object_limit(ctx);
            });
            return;
        };

        self.create_object(
            model,
            owner,
            client_id,
            entrypoint,
            force_expand,
            initial_folder_id,
            InitiatedBy::User,
            ctx,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_folder(
        &mut self,
        name: String,
        owner: Owner,
        client_id: ClientId,
        initial_folder_id: Option<SyncId>,
        force_expand: bool,
        initiated_by: InitiatedBy,
        ctx: &mut ModelContext<Self>,
    ) {
        self.create_object(
            CloudFolderModel::new(&name, false),
            owner,
            client_id,
            Default::default(),
            force_expand,
            initial_folder_id,
            initiated_by,
            ctx,
        );
    }

    /// Generic function for creating a new cloud object with a given model.
    #[allow(clippy::too_many_arguments)]
    pub fn create_object<K, M>(
        &mut self,
        model: M,
        owner: Owner,
        client_id: ClientId,
        entrypoint: CloudObjectEventEntrypoint,
        force_expand: bool,
        initial_folder_id: Option<SyncId>,
        initiated_by: InitiatedBy,
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
        let object_id = SyncId::ClientId(client_id);
        let auth_state = AuthStateProvider::as_ref(ctx).get();
        let initial_editor = auth_state.user_id();

        CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
            let mut object = GenericCloudObject::<K, M>::new_local(
                model.clone(),
                owner,
                initial_folder_id,
                client_id,
            );
            object.metadata.current_editor_uid = initial_editor.map(|uid| uid.as_string());
            cloud_model.create_object(object_id, object, ctx);

            if force_expand {
                cloud_model.force_expand_object_and_ancestors(object_id, ctx);
            }
        });

        let cloud_model = CloudModel::as_ref(ctx);
        if let Some(object) = cloud_model.get_object_of_type::<K, M>(&object_id) {
            self.save_to_db([object.upsert_event()]);
        }

        SyncQueue::handle(ctx).update(ctx, |sync_queue, ctx| {
            let cloud_model = CloudModel::as_ref(ctx);
            if let Some(object) = cloud_model.get_object_of_type::<K, M>(&object_id) {
                if let Some(queue_item) = object.create_object_queue_item(entrypoint, initiated_by)
                {
                    sync_queue.enqueue(queue_item, ctx);
                }
            };
        });
    }

    /// Generic function for updating a cloud object with a new model.
    pub fn update_object<K, M>(
        &mut self,
        model: M,
        object_id: SyncId,
        revision_ts: Option<Revision>,
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
        CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
            cloud_model.update_object_from_edit(model.clone(), object_id, ctx);
            if let Some(object) = cloud_model.get_mut_by_uid(&object_id.uid()) {
                object.increment_in_flight_request_count();
                ctx.notify();
            }
        });

        let cloud_model = CloudModel::as_ref(ctx);
        if let Some(object) = cloud_model.get_object_of_type::<K, M>(&object_id) {
            self.save_to_db([object.upsert_event()]);
        };

        SyncQueue::handle(ctx).update(ctx, |sync_queue, ctx| {
            let cloud_model = CloudModel::as_ref(ctx);
            if let Some(object) = cloud_model.get_object_of_type::<K, M>(&object_id) {
                sync_queue.enqueue(object.update_object_queue_item(revision_ts), ctx);
            };
        });
    }

    pub fn record_object_action(
        &mut self,
        id_and_type: crate::drive::CloudObjectTypeAndId,
        action_type: ObjectActionType,
        data: Option<String>,
        ctx: &mut ModelContext<Self>,
    ) {
        let action_timestamp = Utc::now();

        let object_action = ObjectActions::handle(ctx).update(ctx, |object_actions_model, ctx| {
            object_actions_model.insert_action(
                id_and_type.uid(),
                id_and_type.sqlite_uid_hash(),
                action_type.clone(),
                data.clone(),
                action_timestamp,
                ctx,
            )
        });

        self.save_to_db([ModelEvent::InsertObjectAction { object_action }]);

        SyncQueue::handle(ctx).update(ctx, |sync_queue, ctx| {
            sync_queue.enqueue(
                QueueItem::RecordObjectAction {
                    id_and_type,
                    action_type,
                    data,
                    action_timestamp,
                },
                ctx,
            );
        });
    }

    fn remove_pending_object_action(
        &mut self,
        uid: &ObjectUid,
        action_timestamp: &DateTime<Utc>,
        ctx: &mut ModelContext<Self>,
    ) {
        ObjectActions::handle(ctx).update(ctx, |object_actions_model, ctx| {
            object_actions_model.remove_pending_action(uid, action_timestamp, ctx);
        });
    }

    fn maybe_overwrite_object_action_history(
        &mut self,
        history: &ObjectActionHistory,
        ctx: &mut ModelContext<Self>,
    ) {
        ObjectActions::handle(ctx).update(ctx, |object_actions_model, ctx| {
            let latest_processed_at_ts =
                object_actions_model.get_latest_processed_at_ts(&history.uid);
            if latest_processed_at_ts
                .is_none_or(|client_ts| client_ts <= history.latest_processed_at_timestamp)
            {
                object_actions_model.overwrite_action_history_for_object(
                    &history.uid,
                    history.actions.clone(),
                    ctx,
                );
            }
        });
    }

    fn sync_actions_for_objects_to_sqlite(
        &mut self,
        object_uids: Vec<&ObjectUid>,
        ctx: &mut ModelContext<Self>,
    ) {
        let actions = ObjectActions::handle(ctx).read(ctx, |object_actions_model, _ctx| {
            object_actions_model.get_actions_for_objects(object_uids)
        });

        let actions_to_sync: Vec<ObjectAction> = actions.values().flatten().cloned().collect();
        self.save_to_db([ModelEvent::SyncObjectActions { actions_to_sync }]);
    }

    fn set_notebook_current_editor(
        &self,
        notebook_id: &SyncId,
        editor_uid: Option<String>,
        ctx: &mut ModelContext<Self>,
    ) {
        CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
            if let Some(notebook) = cloud_model.get_notebook_mut(notebook_id) {
                notebook.metadata.set_current_editor(editor_uid);
                ctx.notify();
            }
        });
    }

    pub fn grab_notebook_edit_access(
        &mut self,
        notebook_id: SyncId,
        optimistically_grant_access: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let SyncId::ServerId(server_id) = notebook_id else {
            return;
        };

        let auth_state = AuthStateProvider::as_ref(ctx).get();
        let user_uid = auth_state.user_id().unwrap_or_default();
        if optimistically_grant_access {
            self.set_notebook_current_editor(&notebook_id, Some(user_uid.as_string()), ctx);
        }
        let cloud_object_client = self.object_client.clone();
        let future = ctx.spawn(
            async move { cloud_object_client.grab_notebook_edit_access(server_id.into()).await },
            move |me, res, ctx| match res {
                Ok(metadata) => {
                    me.store_metadata_update(server_id, metadata, ctx, |_| {});
                    if !optimistically_grant_access {
                        me.set_notebook_current_editor(
                            &notebook_id,
                            Some(user_uid.as_string()),
                            ctx,
                        );
                        ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                            result: ObjectOperationResult {
                                success_type: OperationSuccessType::Success,
                                operation: ObjectOperation::Update,
                                client_id: None,
                                server_id: Some(server_id),
                                num_objects: None,
                            },
                        });
                    }
                }
                Err(e) => {
                    if !optimistically_grant_access {
                        log::warn!("Failed to grab edit access on server: {e}. Not retrying.");
                        ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                            result: ObjectOperationResult {
                                success_type: OperationSuccessType::Failure,
                                operation: ObjectOperation::Update,
                                client_id: None,
                                server_id: Some(server_id),
                                num_objects: None,
                            },
                        });
                    } else {
                        log::warn!("Failed to grab edit access on server: {e}. Not retrying. Edit access still granted on client.");
                    }
                    ctx.notify();
                }
            },
        );
        self.spawned_futures.push(future.future_id());
    }

    pub fn give_up_notebook_edit_access(
        &mut self,
        notebook_id: SyncId,
        ctx: &mut ModelContext<Self>,
    ) {
        let SyncId::ServerId(server_id) = notebook_id else {
            return;
        };

        let current_editor = CloudViewModel::as_ref(ctx)
            .object_current_editor(&notebook_id.uid(), ctx)
            .unwrap_or(Editor::no_editor());

        if matches!(current_editor.state, EditorState::CurrentUser) {
            self.set_notebook_current_editor(&notebook_id, None, ctx);
            let object_client = self.object_client.clone();
            let future = ctx.spawn(
                async move {
                    object_client
                        .give_up_notebook_edit_access(server_id.into())
                        .await
                },
                move |me, res, ctx| match res {
                    Ok(new_metadata) => {
                        me.store_metadata_update(server_id, new_metadata, ctx, |_| {});
                    }
                    Err(e) => {
                        log::warn!("Failed to give up edit access: {e}. Not retrying");
                    }
                },
            );
            self.spawned_futures.push(future.future_id());
        }
    }

    pub fn trash_object(&mut self, id: crate::drive::CloudObjectTypeAndId, ctx: &mut ModelContext<Self>) {
        let Some(server_id) = id.server_id() else {
            return;
        };

        let hashed_id = id.uid();
        let Some(has_pending_online_only_operation) =
            CloudModel::handle(ctx).read(ctx, |model, _| {
                model
                    .get_by_uid(&hashed_id)
                    .map(|object| object.metadata().has_pending_online_only_change())
            })
        else {
            return;
        };

        if has_pending_online_only_operation {
            return;
        }

        let (metadata_ts, _trashed_ts) =
            self.mark_object_trashed_and_return_timestamps(&hashed_id, ctx);

        let object_client = self.object_client.clone();

        let future = ctx.spawn(
            async move { object_client.trash_object(server_id).await },
            move |me, res, ctx| match res {
                Ok(_) => {
                    CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
                        if let Some(object) = cloud_model.get_mut_by_uid(&hashed_id) {
                            object
                                .metadata_mut()
                                .pending_changes_statuses
                                .has_pending_metadata_change = false;
                        }

                        let hashed_sqlite_id =
                            server_id.sqlite_type_and_uid_hash(id.object_id_type());
                        me.save_in_memory_object_metadata_to_sqlite(
                            cloud_model,
                            &hashed_id,
                            &hashed_sqlite_id,
                        );
                    });

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Success,
                            operation: ObjectOperation::Trash,
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&hashed_id)),
                            num_objects: None,
                        },
                    });
                    ctx.notify();
                }
                Err(e) => {
                    log::warn!("Failed to trash object: {e}. Not retrying");
                    CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                        if let Some(obj) = cloud_model.get_mut_by_uid(&hashed_id) {
                            let metadata_ts_unchanged =
                                obj.metadata().metadata_last_updated_ts == metadata_ts;
                            if metadata_ts_unchanged {
                                obj.metadata_mut().trashed_ts = None;
                            }

                            obj.metadata_mut()
                                .pending_changes_statuses
                                .has_pending_metadata_change = false;

                            ctx.emit(CloudModelEvent::ObjectUntrashed {
                                type_and_id: obj.cloud_object_type_and_id(),
                                source: UpdateSource::Local,
                            });
                            ctx.notify();
                        }
                    });

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Trash,
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&hashed_id)),
                            num_objects: None,
                        },
                    });
                    ctx.notify();
                }
            },
        );

        self.spawned_futures.push(future.future_id());
    }

    pub fn untrash_object(&mut self, id: crate::drive::CloudObjectTypeAndId, ctx: &mut ModelContext<Self>) {
        let Some(server_id) = id.server_id() else {
            return;
        };

        let hashed_id = id.uid();
        let Some(has_pending_online_only_operation) =
            CloudModel::handle(ctx).read(ctx, |model, _| {
                model
                    .get_by_uid(&hashed_id)
                    .map(|object| object.metadata().has_pending_online_only_change())
            })
        else {
            return;
        };

        if has_pending_online_only_operation {
            return;
        }

        CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
            if let Some(object) = cloud_model.get_mut_by_uid(&hashed_id) {
                object
                    .metadata_mut()
                    .pending_changes_statuses
                    .pending_untrash = true;
            }
        });

        let object_client = self.object_client.clone();

        let future = ctx.spawn(
            async move { object_client.untrash_object(server_id).await },
            move |me, res, ctx| match res {
                Ok(crate::cloud_object::ObjectMetadataUpdateResult::Failure) => {
                    CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
                        if let Some(object) = cloud_model.get_mut_by_uid(&hashed_id) {
                            object
                                .metadata_mut()
                                .pending_changes_statuses
                                .pending_untrash = false;
                        }
                    });

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Untrash,
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&hashed_id)),
                            num_objects: None,
                        },
                    });
                }
                Ok(crate::cloud_object::ObjectMetadataUpdateResult::Success { metadata }) => {
                    me.store_metadata_update(server_id, *metadata, ctx, |object| {
                        object
                            .metadata_mut()
                            .pending_changes_statuses
                            .pending_untrash = false;
                    });

                    CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
                        if let Some(object) = cloud_model.get_by_uid(&hashed_id) {
                            ctx.emit(CloudModelEvent::ObjectUntrashed {
                                type_and_id: object.cloud_object_type_and_id(),
                                source: UpdateSource::Local,
                            });
                            ctx.notify();
                        }
                    });

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Success,
                            operation: ObjectOperation::Untrash,
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&hashed_id)),
                            num_objects: None,
                        },
                    });
                }
                Err(e) => {
                    log::warn!("Failed to restore object: {e}. Not retrying");

                    CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
                        if let Some(object) = cloud_model.get_mut_by_uid(&hashed_id) {
                            object
                                .metadata_mut()
                                .pending_changes_statuses
                                .pending_untrash = false;
                        }
                    });

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Untrash,
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&hashed_id)),
                            num_objects: None,
                        },
                    });

                    ctx.notify();
                }
            },
        );

        self.spawned_futures.push(future.future_id());
    }

    pub fn delete_object_by_user(
        &mut self,
        id: crate::drive::CloudObjectTypeAndId,
        ctx: &mut ModelContext<Self>,
    ) {
        self.delete_object_with_initiated_by(id, InitiatedBy::User, ctx);
    }

    pub fn delete_object_with_initiated_by(
        &mut self,
        id: crate::drive::CloudObjectTypeAndId,
        initiated_by: InitiatedBy,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(server_id) = id.server_id() else {
            return;
        };

        let uid = id.uid();
        let Some((has_pending_online_only_operation, has_pending_delete)) = CloudModel::handle(ctx)
            .read(ctx, |model, _| {
                model.get_by_uid(&uid).map(|object| {
                    (
                        object.metadata().has_pending_online_only_change(),
                        object.metadata().pending_changes_statuses.pending_delete,
                    )
                })
            })
        else {
            return;
        };

        if has_pending_online_only_operation || has_pending_delete {
            return;
        }

        let object_client = self.object_client.clone();

        CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
            if let Some(object) = cloud_model.get_mut_by_uid(&uid) {
                object
                    .metadata_mut()
                    .pending_changes_statuses
                    .pending_delete = true;
            }
        });

        let future = ctx.spawn(
            async move { object_client.delete_object(server_id).await },
            move |me, res, ctx| match res {
                Ok(crate::cloud_object::ObjectDeleteResult::Success { deleted_ids }) => {
                    let num_deleted_objects = me.on_object_delete_success(deleted_ids, ctx);
                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Success,
                            operation: ObjectOperation::Delete { initiated_by },
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&uid)),
                            num_objects: Some(num_deleted_objects),
                        },
                    });
                }
                Ok(crate::cloud_object::ObjectDeleteResult::Failure) => {
                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Delete { initiated_by },
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&uid)),
                            num_objects: None,
                        },
                    });
                }
                Err(e) => {
                    log::warn!("Failed to delete object: {e}. Not retrying");

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Delete { initiated_by },
                            client_id: None,
                            server_id: Some(ServerId::from_string_lossy(&uid)),
                            num_objects: None,
                        },
                    });

                    CloudModel::handle(ctx).update(ctx, |cloud_model, _| {
                        if let Some(object) = cloud_model.get_mut_by_uid(&uid) {
                            object
                                .metadata_mut()
                                .pending_changes_statuses
                                .pending_delete = false;
                        }
                    });

                    ctx.notify();
                }
            },
        );

        self.spawned_futures.push(future.future_id());
    }

    pub fn empty_trash(&mut self, space: Space, ctx: &mut ModelContext<Self>) {
        let object_client = self.object_client.clone();

        let owner = match UserWorkspaces::as_ref(ctx).space_to_owner(space, ctx) {
            Some(owner) => owner,
            None => {
                log::warn!("Tried to empty trash in unsupported space {space:?}");
                return;
            }
        };

        let future = ctx.spawn(
            async move { object_client.empty_trash(owner).await },
            move |me, res, ctx| match res {
                Ok(crate::cloud_object::ObjectDeleteResult::Success { deleted_ids }) => {
                    let num_deleted_objects = me.on_object_delete_success(deleted_ids, ctx);

                    if num_deleted_objects == 0 {
                        ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                            result: ObjectOperationResult {
                                success_type: OperationSuccessType::Rejection,
                                operation: ObjectOperation::Delete { initiated_by: InitiatedBy::User },
                                client_id: None,
                                server_id: None,
                                num_objects: Some(num_deleted_objects),
                            },
                        });
                    } else {
                        ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                            result: ObjectOperationResult {
                                success_type: OperationSuccessType::Success,
                                operation: ObjectOperation::Delete { initiated_by: InitiatedBy::User },
                                client_id: None,
                                server_id: None,
                                num_objects: Some(num_deleted_objects),
                            },
                        });
                    }
                }
                Ok(crate::cloud_object::ObjectDeleteResult::Failure) => {
                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Delete { initiated_by: InitiatedBy::User },
                            client_id: None,
                            server_id: None,
                            num_objects: Some(0),
                        },
                    });
                }
                Err(e) => {
                    log::warn!("Failed to empty trash: {e}. Not retrying");

                    ctx.emit(UpdateManagerEvent::ObjectOperationComplete {
                        result: ObjectOperationResult {
                            success_type: OperationSuccessType::Failure,
                            operation: ObjectOperation::Delete { initiated_by: InitiatedBy::User },
                            client_id: None,
                            server_id: None,
                            num_objects: Some(0),
                        },
                    });
                    ctx.notify();
                }
            },
        );

        self.spawned_futures.push(future.future_id());
    }

    pub fn on_object_delete_success(
        &mut self,
        deleted_ids: Vec<SyncId>,
        ctx: &mut ModelContext<'_, UpdateManager>,
    ) -> i32 {
        let cloud_model_handle = CloudModel::handle(ctx);
        let all_object_uids: Vec<ObjectUid> = deleted_ids.iter().map(|&id| id.uid()).collect();

        let mut num_deleted_objects = 0;
        let mut sync_ids_and_types: Vec<(SyncId, ObjectIdType)> = Vec::new();
        cloud_model_handle.update(ctx, |cloud_model, ctx| {
            (sync_ids_and_types, num_deleted_objects) =
                cloud_model.delete_objects_by_id(all_object_uids.clone(), ctx);
        });

        ObjectActions::handle(ctx).update(ctx, |object_actions, ctx| {
            for uid in all_object_uids.clone() {
                object_actions.delete_actions_for_object(&uid, ctx);
            }
        });

        if num_deleted_objects == 0 {
            return num_deleted_objects;
        }

        self.save_to_db([ModelEvent::DeleteObjects {
            ids: sync_ids_and_types,
        }]);

        num_deleted_objects
    }

    pub fn rename_folder(
        &mut self,
        folder_id: SyncId,
        new_name: String,
        ctx: &mut ModelContext<Self>,
    ) {
        let cloud_model = CloudModel::as_ref(ctx);
        let revision = cloud_model.current_revision(&folder_id).cloned();
        if let Some(folder) = cloud_model.get_folder(&folder_id) {
            let new_folder = CloudFolderModel {
                name: new_name,
                is_open: folder.model().is_open,
                is_warp_pack: folder.model().is_warp_pack,
            };
            self.update_object(new_folder, folder_id, revision, ctx);
        } else {
            log::warn!("Attempted to rename folder that doesn't exist with id: {folder_id:?}");
        }
    }

    fn store_metadata_update(
        &mut self,
        server_id: ServerId,
        new_metadata: ServerMetadata,
        ctx: &mut ModelContext<Self>,
        update: impl FnOnce(&mut dyn CloudObject),
    ) {
        let cloud_model_handle = CloudModel::handle(ctx);

        let mut hashed_sqlite_id = None;
        cloud_model_handle.update(ctx, |cloud_model, _| {
            if let Some(object) = cloud_model.get_mut_by_uid(&server_id.uid()) {
                object
                    .metadata_mut()
                    .update_from_new_metadata_ts(new_metadata);
                update(object.as_mut());

                hashed_sqlite_id =
                    Some(server_id.sqlite_type_and_uid_hash(object.object_type().into()));
            }
        });

        if let Some(hashed_sqlite_id) = hashed_sqlite_id {
            self.save_in_memory_object_metadata_to_sqlite(
                cloud_model_handle.as_ref(ctx),
                &server_id.uid(),
                &hashed_sqlite_id,
            );
        }
    }

    fn mark_object_trashed_and_return_timestamps(
        &self,
        uid: &ObjectUid,
        ctx: &mut ModelContext<Self>,
    ) -> (Option<ServerTimestamp>, Option<ServerTimestamp>) {
        let timestamp = ServerTimestamp::new(Utc::now());
        CloudModel::handle(ctx).update(ctx, |cloud_model, ctx| {
            if let Some(object) = cloud_model.get_mut_by_uid(uid) {
                object.metadata_mut().trashed_ts = Some(timestamp);
                object
                    .metadata_mut()
                    .pending_changes_statuses
                    .has_pending_metadata_change = true;
                ctx.emit(CloudModelEvent::ObjectTrashed {
                    type_and_id: object.cloud_object_type_and_id(),
                    source: UpdateSource::Local,
                });
                ctx.notify();
                (
                    object.metadata().metadata_last_updated_ts,
                    object.metadata().trashed_ts,
                )
            } else {
                (None, None)
            }
        })
    }

    fn save_in_memory_object_to_sqlite(
        &self,
        cloud_model: &CloudModel,
        uid: &ObjectUid,
    ) {
        if let Some(cloud_object) = cloud_model.get_by_uid(uid) {
            self.save_to_db([cloud_object.upsert_event()]);
        }
    }

    fn save_in_memory_object_metadata_to_sqlite(
        &self,
        cloud_model: &CloudModel,
        uid: &ObjectUid,
        hashed_sqlite_id: &HashedSqliteId,
    ) {
        if let Some(cloud_object) = cloud_model.get_by_uid(uid) {
            let metadata = cloud_object.metadata().clone();
            self.save_to_db([ModelEvent::UpdateObjectMetadata {
                id: hashed_sqlite_id.clone(),
                metadata,
            }]);
        }
    }

    /// Create a scheduled ambient agent (stub for local version).
    pub fn create_scheduled_ambient_agent_online(
        &mut self,
        _task_id: AmbientAgentTaskId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Not supported in local version
    }

    /// Update a scheduled ambient agent (stub for local version).
    pub fn update_scheduled_ambient_agent_online(
        &mut self,
        _task_id: AmbientAgentTaskId,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Not supported in local version
    }

    /// Move object to location (stub for local version).
    pub fn move_object_to_location(
        &mut self,
        _object_to_move: crate::drive::CloudObjectTypeAndId,
        _destination_folder_id: Option<ServerId>,
        _space: Space,
        _ctx: &mut ModelContext<Self>,
    ) {
        // Not supported in local version
    }

    /// Stop polling for updated objects (stub for local version).
    pub fn stop_polling_for_updated_objects(&mut self) {
        // Not supported in local version
    }

    /// Wait for an initial load to complete.
    pub fn initial_load_complete(&self) -> impl std::future::Future<Output = ()> {
        self.has_initial_load.wait()
    }

    /// Reset the initial-load condition.
    pub fn reset_initial_load(&self) {
        log::info!("Resetting initial_load_complete condition for fresh cloud object fetch");
        self.has_initial_load.reset();
    }
}

/// Return the newly duplicated object's name based on the original object's name.
pub fn get_duplicate_object_name(original_name: &str) -> String {
    match DUPLICATE_OBJECT_NAME_REGEX
        .captures(original_name)
        .and_then(|caps| caps.get(1))
        .and_then(|num| num.as_str().parse::<usize>().ok())
    {
        Some(num) => {
            let new_num = num.saturating_add(1);

            if new_num == usize::MAX {
                format!("{original_name} (1)")
            } else {
                DUPLICATE_OBJECT_NAME_REGEX
                    .replace(original_name, format!(" ({new_num})"))
                    .to_string()
            }
        }
        None => format!("{original_name} (1)"),
    }
}

impl Entity for UpdateManager {
    type Event = UpdateManagerEvent;
}

impl SingletonEntity for UpdateManager {}

//! Stub module for sync_queue types.
//!
//! This module provides minimal stubs for types that were used by the sync queue
//! but are now no-ops in the local version.

use std::sync::Arc;
use chrono::{DateTime, Utc};
use cuteui::AppContext;

use crate::cloud_object::ObjectType;
use crate::drive::CloudObjectTypeAndId;
use crate::server::ids::{ClientId, ServerId, SyncId};
use crate::workflows::workflow::Workflow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InitiatedBy {
    User,
    Cloud,
    System,
}

/// Serialized model data (stub for local version).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct SerializedModel(String);

impl SerializedModel {
    pub fn new(data: String) -> Self {
        Self(data)
    }

    pub fn model_as_str(&self) -> &str {
        &self.0
    }
}

impl From<SerializedModel> for String {
    fn from(model: SerializedModel) -> Self {
        model.0
    }
}

impl From<String> for SerializedModel {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Queue item for sync operations (stub for local version).
#[derive(Debug)]
pub enum QueueItem {
    CreateWorkflow {
        object_type: ObjectType,
        owner: crate::cloud_object::Owner,
        model: Arc<Workflow>,
        initial_folder_id: Option<SyncId>,
        entrypoint: crate::cloud_object::CloudObjectEventEntrypoint,
        id: ClientId,
        initiated_by: InitiatedBy,
    },
    UpdateWorkflow {
        model: Arc<Workflow>,
        id: SyncId,
        revision: Option<crate::cloud_object::Revision>,
    },
    UpdateWorkflowEnum {
        model: Arc<crate::workflows::workflow_enum::WorkflowEnum>,
        id: SyncId,
        revision: Option<crate::cloud_object::Revision>,
    },
    CreateObject {
        object_type: ObjectType,
        owner: crate::cloud_object::Owner,
        id: ClientId,
        title: Option<String>,
        serialized_model: Option<String>,
        initial_folder_id: Option<SyncId>,
        entrypoint: crate::cloud_object::CloudObjectEventEntrypoint,
        initiated_by: InitiatedBy,
    },
    RecordObjectAction {
        id_and_type: CloudObjectTypeAndId,
        action_type: crate::cloud_object::model::actions::ObjectActionType,
        data: Option<String>,
        action_timestamp: DateTime<Utc>,
    },
}

impl QueueItem {
    pub fn from_cached_objects(_objects: impl Iterator<Item = Box<dyn crate::cloud_object::CloudObject>>) -> Vec<Self> {
        Vec::new()
    }

    pub fn from_unsynced_actions(_actions: impl Iterator<Item = (CloudObjectTypeAndId, crate::cloud_object::model::actions::ObjectAction)>) -> Vec<Self> {
        Vec::new()
    }
}

/// Reason for object creation failure (stub for local version).
#[derive(Debug)]
pub enum CreationFailureReason {
    UniqueKeyConflict {
        id: String,
        initiated_by: InitiatedBy,
    },
    Other {
        id: String,
        initiated_by: InitiatedBy,
    },
    Denied {
        message: String,
        client_id: ClientId,
        initiated_by: InitiatedBy,
    },
}

/// Event from the sync queue (stub for local version).
#[derive(Debug)]
pub enum SyncQueueEvent {
    ObjectCreationSuccessful {
        server_creation_info: crate::cloud_object::ServerCreationInfo,
        client_id: ClientId,
        revision_and_editor: crate::cloud_object::RevisionAndLastEditor,
        metadata_ts: cute_graphql::scalars::time::ServerTimestamp,
        initiated_by: InitiatedBy,
    },
    ObjectUpdateSuccessful {
        server_id: ServerId,
        revision_and_editor: crate::cloud_object::RevisionAndLastEditor,
    },
    ObjectCreationFailure {
        reason: CreationFailureReason,
    },
    ObjectUpdateFailure {
        id: SyncId,
    },
    ObjectUpdateRejected {
        id: String,
        object: Arc<crate::cloud_object::ServerCloudObject>,
    },
    ObjectUpdateFeatureNotAvailable {
        id: String,
    },
    ReportObjectActionFailed {
        uid: String,
        action_timestamp: DateTime<Utc>,
    },
    ReportObjectActionSucceeded {
        uid: String,
        action_timestamp: DateTime<Utc>,
        action_history: crate::cloud_object::model::actions::ObjectActionHistory,
    },
}

/// Sync queue for cloud object operations (stub for local version).
pub struct SyncQueue;

impl SyncQueue {
    pub fn new(
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _ctx: &mut AppContext,
    ) -> Self {
        Self
    }

    pub fn enqueue(&mut self, _item: QueueItem, _ctx: &mut AppContext) {
        // No-op for local version
    }

    pub fn clear(&mut self) {
        // No-op for local version
    }

    pub fn is_dequeueing(&self) -> bool {
        false
    }
}

impl cuteui::Entity for SyncQueue {
    type Event = SyncQueueEvent;
}

impl cuteui::SingletonEntity for SyncQueue {}

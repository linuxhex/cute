//! Stub module for sync_queue types.
//!
//! This module provides minimal stubs for types that were used by the sync queue
//! but are now no-ops in the local version.

use std::sync::Arc;
use chrono::{DateTime, Utc};
use cuteui::AppContext;

use crate::local_storage_types::ObjectType;
use crate::local_storage_types::CloudObjectTypeAndId;
use crate::server::ids::{ClientId, SyncId};
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
        owner: crate::local_storage_types::Owner,
        model: Arc<Workflow>,
        initial_folder_id: Option<SyncId>,
        entrypoint: crate::local_storage_types::CloudObjectEventEntrypoint,
        id: ClientId,
        initiated_by: InitiatedBy,
    },
    UpdateWorkflow {
        model: Arc<Workflow>,
        id: SyncId,
        revision: Option<crate::local_storage_types::Revision>,
    },
    UpdateWorkflowEnum {
        model: Arc<crate::workflows::workflow_enum::WorkflowEnum>,
        id: SyncId,
        revision: Option<crate::local_storage_types::Revision>,
    },
    CreateObject {
        object_type: ObjectType,
        owner: crate::local_storage_types::Owner,
        id: ClientId,
        title: Option<String>,
        serialized_model: Option<String>,
        initial_folder_id: Option<SyncId>,
        entrypoint: crate::local_storage_types::CloudObjectEventEntrypoint,
        initiated_by: InitiatedBy,
    },
    RecordObjectAction {
        id_and_type: CloudObjectTypeAndId,
        action_type: crate::local_storage_types::model::actions::ObjectActionType,
        data: Option<String>,
        action_timestamp: DateTime<Utc>,
    },
}

impl QueueItem {
    pub fn from_cached_objects(_objects: impl Iterator<Item = Box<dyn crate::local_storage_types::CloudObject>>) -> Vec<Self> {
        Vec::new()
    }

    pub fn from_unsynced_actions(_actions: impl Iterator<Item = (CloudObjectTypeAndId, crate::local_storage_types::model::actions::ObjectAction)>) -> Vec<Self> {
        Vec::new()
    }
}



/// Sync queue for cloud object operations (stub for local version).
pub struct SyncQueue;

/// Empty event type for SyncQueue stub (never emits events).
#[derive(Debug)]
pub enum SyncQueueStubEvent {}

impl SyncQueue {
    pub fn new(
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _ctx: &mut AppContext,
    ) -> Self {
        Self
    }

    pub fn is_dequeueing(&self) -> bool {
        false
    }
}

impl cuteui::Entity for SyncQueue {
    type Event = SyncQueueStubEvent;
}

impl cuteui::SingletonEntity for SyncQueue {}

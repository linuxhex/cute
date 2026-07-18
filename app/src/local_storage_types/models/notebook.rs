use ai::document::AIDocumentId;
use cute_server_client::{
    cloud_object::{GenericCloudObject, GenericServerObject, ObjectType, ServerObjectModel},
    ids::{ServerId, SyncId},
};
use serde::{Deserialize, Serialize};
use crate::local_storage_types::{CloudModelType, CloudObjectUpsertParams};
use crate::persistence::ModelEvent;
use crate::{CloudObjectTypeAndId, CuteDriveItem};
use crate::appearance::Appearance;

/// Serialized representation of a notebook for sync queue
/// The AIDocumentID and ConversationID are stored here to avoid polluting the
/// generic CreateObjectRequest type.
#[derive(Serialize, Deserialize)]
pub struct SerializedNotebook {
    pub data: String,
    pub ai_document_id: Option<String>,
    pub conversation_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CloudNotebookModel {
    pub title: String,
    pub data: String,
    pub ai_document_id: Option<AIDocumentId>,
    /// This is the server-generated conversation token, not the client-side AIConversationId.
    pub conversation_id: Option<String>,
}

impl ServerObjectModel for CloudNotebookModel {
    fn object_type(&self) -> ObjectType {
        ObjectType::Notebook
    }
}

impl CloudModelType for CloudNotebookModel {
    type CloudObjectType = CloudNotebook;
    type IdType = NotebookId;

    fn model_type_name(&self) -> &'static str {
        "Notebook"
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::Notebook
    }

    fn cloud_object_type_and_id(&self, id: SyncId) -> CloudObjectTypeAndId {
        CloudObjectTypeAndId::Notebook(id)
    }

    fn display_name(&self) -> String {
        self.title.clone()
    }

    fn set_display_name(&mut self, name: &str) {
        self.title = name.to_string();
    }

    fn upsert_event(params: CloudObjectUpsertParams<Self>) -> ModelEvent {
        ModelEvent::UpsertNotebook {
            notebook: CloudNotebook::from(params),
        }
    }

    fn bulk_upsert_event(objects: Vec<CloudObjectUpsertParams<Self>>) -> ModelEvent {
        ModelEvent::UpsertNotebooks(objects.into_iter().map(CloudNotebook::from).collect())
    }

    fn renders_in_cute_drive(&self) -> bool {
        true
    }

    fn to_cute_drive_item(
        &self,
        _id: SyncId,
        _appearance: &Appearance,
        _object: &GenericCloudObject<NotebookId, Self>,
    ) -> Option<Box<dyn CuteDriveItem>> {
        // Notebook items not implemented in stub
        None
    }
}

/// This is the notebook_id in the database associated with this notebook.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct NotebookId(ServerId);
cute_server_client::server_id_traits! { NotebookId, "Notebook" }

impl From<NotebookId> for SyncId {
    fn from(id: NotebookId) -> Self {
        Self::ServerId(id.into())
    }
}

/// `CloudNotebook` is a notebook retrieved from the server.
pub type CloudNotebook = GenericCloudObject<NotebookId, CloudNotebookModel>;
pub type ServerNotebook = GenericServerObject<NotebookId, CloudNotebookModel>;

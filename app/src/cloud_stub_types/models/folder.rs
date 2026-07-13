use cute_server_client::{
    cloud_object::{GenericCloudObject, GenericServerObject, ObjectType, ServerObjectModel},
    ids::FolderId,
};
use crate::cloud_stub_types::{CloudModelType, CloudObjectUpsertParams};
use crate::persistence::ModelEvent;
use crate::server::ids::SyncId;
use crate::{CloudObjectTypeAndId, CuteDriveItem};
use crate::appearance::Appearance;

/// The model for a `CloudFolder`.
#[derive(Clone, Debug, PartialEq)]
pub struct CloudFolderModel {
    pub name: String,
    // TODO: since this is local only state, we should consider only surfacing it as part of the
    // CloudViewModel. Right now, every server folder uses CloudFolderModel, which means it
    // hardcodes a value of `false` for this property since it can't know what the local state is.
    pub is_open: bool,
    pub is_cute_pack: bool,
}

impl CloudFolderModel {
    pub fn new(name: &str, is_cute_pack: bool) -> Self {
        Self {
            name: name.to_owned(),
            is_open: false,
            is_cute_pack,
        }
    }
}

impl ServerObjectModel for CloudFolderModel {
    fn object_type(&self) -> ObjectType {
        ObjectType::Folder
    }
}

impl CloudModelType for CloudFolderModel {
    type CloudObjectType = CloudFolder;
    type IdType = FolderId;

    fn model_type_name(&self) -> &'static str {
        "Folder"
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::Folder
    }

    fn cloud_object_type_and_id(&self, id: SyncId) -> CloudObjectTypeAndId {
        CloudObjectTypeAndId::Folder(id)
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn set_display_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn upsert_event(params: CloudObjectUpsertParams<Self>) -> ModelEvent {
        ModelEvent::UpsertFolder {
            folder: CloudFolder::from(params),
        }
    }

    fn bulk_upsert_event(objects: Vec<CloudObjectUpsertParams<Self>>) -> ModelEvent {
        ModelEvent::UpsertFolders(objects.into_iter().map(CloudFolder::from).collect())
    }

    fn renders_in_cute_drive(&self) -> bool {
        true
    }

    fn to_cute_drive_item(
        &self,
        _id: SyncId,
        _appearance: &Appearance,
        _object: &GenericCloudObject<FolderId, Self>,
    ) -> Option<Box<dyn CuteDriveItem>> {
        // Folder items not implemented in stub
        None
    }
}

/// `CloudFolder` is a folder retrieved from the server.
pub type CloudFolder = GenericCloudObject<FolderId, CloudFolderModel>;
pub type ServerFolder = GenericServerObject<FolderId, CloudFolderModel>;

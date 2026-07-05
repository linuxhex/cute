use std::any::Any;
use std::sync::Arc;

use crate::cloud_object::CloudObject;
use crate::cloud_object::CloudObjectMetadata;
use crate::cloud_object::CloudObjectPermissions;
use crate::server::ids::{HashedSqliteId, ObjectUid, ServerId, SyncId};
use crate::persistence::ModelEvent;
use cute_server_client::ids::{HashableId, ToServerId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FolderId(pub String);

impl FolderId {
    pub fn from_hash(hash: &str) -> Option<Self> {
        Some(FolderId(hash.to_string()))
    }
}

impl From<String> for FolderId {
    fn from(s: String) -> Self {
        FolderId(s)
    }
}

impl HashableId for FolderId {
    fn to_hash(&self) -> String {
        format!("Folder-{}", self.0)
    }

    fn from_hash(hash: &str) -> Option<Self> {
        hash.strip_prefix("Folder-")
            .map(|s| s.to_string().into())
    }
}

impl ToServerId for FolderId {
    fn to_server_id(&self) -> ServerId {
        ServerId::from_string_lossy(self.0.clone())
    }
}

impl From<FolderId> for ServerId {
    fn from(id: FolderId) -> Self {
        ServerId::from_string_lossy(id.0)
    }
}

#[derive(Debug, Clone)]
pub struct CloudFolderModel {
    pub name: String,
    pub is_open: bool,
    pub is_warp_pack: bool,
}

#[derive(Debug, Clone)]
pub struct CloudFolder {
    pub id: SyncId,
    pub metadata: CloudObjectMetadata,
    pub permissions: CloudObjectPermissions,
    pub conflict_status: (),
    model: Arc<CloudFolderModel>,
}

impl CloudFolder {
    pub fn new(
        id: SyncId,
        model: CloudFolderModel,
        metadata: CloudObjectMetadata,
        permissions: CloudObjectPermissions,
    ) -> Self {
        Self {
            id,
            metadata,
            permissions,
            conflict_status: (),
            model: Arc::new(model),
        }
    }

    pub fn model(&self) -> &CloudFolderModel {
        &self.model
    }
}

impl<'a> From<&'a dyn CloudObject> for Option<&'a CloudFolder> {
    fn from(value: &'a dyn CloudObject) -> Self {
        value.as_any().downcast_ref::<CloudFolder>()
    }
}

impl<'a> From<&'a Box<dyn CloudObject>> for Option<&'a CloudFolder> {
    fn from(value: &'a Box<dyn CloudObject>) -> Self {
        value.as_any().downcast_ref::<CloudFolder>()
    }
}

impl<'a> From<&'a mut Box<dyn CloudObject>> for Option<&'a mut CloudFolder> {
    fn from(value: &'a mut Box<dyn CloudObject>) -> Self {
        value.as_any_mut().downcast_mut::<CloudFolder>()
    }
}

impl CloudObject for CloudFolder {
    fn model_type_name(&self) -> &'static str {
        "CloudFolder"
    }

    fn uid(&self) -> ObjectUid {
        ObjectUid::new()
    }

    fn sync_id(&self) -> SyncId {
        self.id
    }

    fn hashed_sqlite_id(&self) -> HashedSqliteId {
        HashedSqliteId::new()
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

    fn object_type(&self) -> crate::cloud_object::ObjectType {
        crate::cloud_object::ObjectType::Folder
    }

    fn cloud_object_type_and_id(&self) -> crate::drive::CloudObjectTypeAndId {
        crate::drive::CloudObjectTypeAndId::Folder(self.sync_id())
    }

    fn set_server_id(&mut self, _server_id: ServerId) {}

    fn upsert_event(&self) -> ModelEvent {
        unimplemented!()
    }

    fn display_name(&self) -> String {
        self.model.name.clone()
    }

    fn versions(&self, _app: &cuteui::AppContext) -> Option<cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput> {
        None
    }

    fn renders_in_warp_drive(&self) -> bool {
        true
    }

    fn to_warp_drive_item(&self, _appearance: &crate::appearance::Appearance) -> Option<Box<dyn crate::drive::items::WarpDriveItem>> {
        None
    }

    fn object_link(&self) -> Option<String> {
        None
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

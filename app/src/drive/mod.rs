pub mod cloud_object_styling;
pub mod drive_helpers;
pub mod export;
pub mod folders;
pub mod import;
pub mod items;
pub mod panel;
pub mod settings;
pub mod sharing;
pub mod workflows;

pub use panel::{DrivePanel, DrivePanelEvent};

use crate::cloud_object::{ObjectIdType, ObjectType};
use crate::server::ids::{HashedSqliteId, ServerId};

#[derive(Debug, Clone)]
pub enum DriveIndexVariant {
    Personal,
    Team,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CloudObjectTypeAndId {
    Notebook(crate::server::ids::SyncId),
    Workflow(crate::server::ids::SyncId),
    Folder(crate::server::ids::SyncId),
    EnvVarCollection(crate::server::ids::SyncId),
    GenericStringObject {
        object_type: String,
        id: crate::server::ids::SyncId,
    },
}

impl CloudObjectTypeAndId {
    pub fn from_generic_string_object(object_type: crate::cloud_object::GenericStringObjectFormat, id: crate::server::ids::SyncId) -> Self {
        Self::GenericStringObject {
            object_type: format!("{:?}", object_type),
            id,
        }
    }

    pub fn from_id_and_type(id: crate::server::ids::SyncId, object_type: crate::cloud_object::ObjectType) -> Self {
        match object_type {
            crate::cloud_object::ObjectType::Notebook => Self::Notebook(id),
            crate::cloud_object::ObjectType::Workflow => Self::Workflow(id),
            crate::cloud_object::ObjectType::Folder => Self::Folder(id),
            crate::cloud_object::ObjectType::GenericStringObject(
                crate::cloud_object::GenericStringObjectFormat::Json(
                    crate::cloud_object::JsonObjectType::EnvVarCollection
                )
            ) => Self::EnvVarCollection(id),
            _ => Self::GenericStringObject {
                object_type: format!("{:?}", object_type),
                id,
            },
        }
    }

    pub fn uid(&self) -> &crate::server::ids::SyncId {
        match self {
            Self::Notebook(id) => id,
            Self::Workflow(id) => id,
            Self::Folder(id) => id,
            Self::EnvVarCollection(id) => id,
            Self::GenericStringObject { id, .. } => id,
        }
    }

    pub fn object_type(&self) -> crate::cloud_object::ObjectType {
        match self {
            Self::Notebook(_) => crate::cloud_object::ObjectType::Notebook,
            Self::Workflow(_) => crate::cloud_object::ObjectType::Workflow,
            Self::Folder(_) => crate::cloud_object::ObjectType::Folder,
            Self::EnvVarCollection(_) => crate::cloud_object::ObjectType::GenericStringObject(
                crate::cloud_object::GenericStringObjectFormat::Json(
                    crate::cloud_object::JsonObjectType::EnvVarCollection
                )
            ),
            Self::GenericStringObject { object_type, .. } => {
                if object_type.contains("EnvVarCollection") {
                    crate::cloud_object::ObjectType::GenericStringObject(
                        crate::cloud_object::GenericStringObjectFormat::Json(
                            crate::cloud_object::JsonObjectType::EnvVarCollection
                        )
                    )
                } else {
                    crate::cloud_object::ObjectType::Notebook
                }
            }
        }
    }

    pub fn as_notebook_id(&self) -> Option<&crate::server::ids::SyncId> {
        match self {
            Self::Notebook(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_generic_string_object_id(&self) -> Option<&crate::server::ids::SyncId> {
        match self {
            Self::GenericStringObject { id, .. } => Some(id),
            Self::EnvVarCollection(id) => Some(id),
            _ => None,
        }
    }

    pub fn has_server_id(&self) -> bool {
        match self {
            Self::Notebook(crate::server::ids::SyncId::ServerId(_)) => true,
            Self::Workflow(crate::server::ids::SyncId::ServerId(_)) => true,
            Self::Folder(crate::server::ids::SyncId::ServerId(_)) => true,
            Self::EnvVarCollection(crate::server::ids::SyncId::ServerId(_)) => true,
            Self::GenericStringObject { id: crate::server::ids::SyncId::ServerId(_), .. } => true,
            _ => false,
        }
    }

    pub fn sqlite_uid_hash(&self) -> HashedSqliteId {
        match self {
            Self::Notebook(id) => id.sqlite_uid_hash(ObjectIdType::Notebook),
            Self::Workflow(id) => id.sqlite_uid_hash(ObjectIdType::Workflow),
            Self::Folder(id) => id.sqlite_uid_hash(ObjectIdType::Folder),
            Self::EnvVarCollection(id) => id.sqlite_uid_hash(ObjectIdType::GenericStringObject),
            Self::GenericStringObject { id, .. } => id.sqlite_uid_hash(ObjectIdType::GenericStringObject),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenWarpDriveObjectArgs {
    pub object_type: ObjectType,
    pub server_id: ServerId,
    pub settings: OpenWarpDriveObjectSettings,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpenWarpDriveObjectSettings {
    pub focused_folder_id: Option<ServerId>,
    pub invitee_email: Option<String>,
}

pub fn should_auto_open_welcome_folder() -> bool {
    false
}

pub fn write_has_auto_opened_welcome_folder_to_user_defaults() {
}

#[derive(Debug, Clone)]
pub enum DriveObjectType {
    Workflow,
    Notebook { is_ai_document: bool },
    AgentModeWorkflow,
    EnvVarCollection,
    Folder,
    AIFact,
}

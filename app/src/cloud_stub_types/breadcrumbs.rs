use cuteui::AppContext;

use super::{CloudObject, Space};
// Import cloud stub types for removed WarpDrive functionality
use crate::{CuteDriveItemId, CloudObjectTypeAndId, CloudFolder};
use crate::ui_components::breadcrumb::Breadcrumb;

// Encapsulates an object that can contain other objects, and keeps
// information necessary to show breadcrumbs.
#[derive(Clone, Debug)]
pub struct ContainingObject {
    pub name: String,
    pub kind: ContainingObjectKind,
}

impl Breadcrumb for ContainingObject {
    fn label(&self) -> String {
        self.name.clone()
    }

    fn enabled(&self) -> bool {
        true
    }
}

impl From<&CloudFolder> for ContainingObject {
    fn from(folder: &CloudFolder) -> Self {
        Self {
            name: folder.display_name().clone(),
            kind: ContainingObjectKind::Object(CloudObjectTypeAndId::Folder(folder.id)),
        }
    }
}

impl Space {
    pub fn into_containing_object(self, app: &AppContext) -> ContainingObject {
        ContainingObject {
            name: self.name(app).clone(),
            kind: ContainingObjectKind::Space(self),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ContainingObjectKind {
    Space(Space),
    Object(CloudObjectTypeAndId),
}

impl ContainingObjectKind {
    pub fn into_item_id(self) -> CuteDriveItemId {
        match self {
            ContainingObjectKind::Space(space) => {
                match space {
                    Space::Team { team_uid } => CuteDriveItemId::Space(team_uid),
                    Space::Personal => CuteDriveItemId::Folder("personal".to_string()), // or some default
                    Space::Shared => CuteDriveItemId::Folder("shared".to_string()), // or some default
                }
            },
            ContainingObjectKind::Object(object) => CuteDriveItemId::Object(object),
        }
    }
}

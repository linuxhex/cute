// Minimal breadcrumbs stub - CuteDrive navigation removed
// ContainingObject is retained for UI breadcrumb display but no longer
// converts to CuteDriveItemId

use cuteui::AppContext;
use crate::cloud_stub_types::CloudObject;
use crate::ui_components::breadcrumb::Breadcrumb;

/// Encapsulates an object that can contain other objects, and keeps
/// information necessary to show breadcrumbs.
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

impl From<&crate::cloud_stub_types::models::CloudFolder> for ContainingObject {
    fn from(folder: &crate::cloud_stub_types::models::CloudFolder) -> Self {
        Self {
            name: folder.display_name().clone(),
            kind: ContainingObjectKind::Object(super::CloudObjectTypeAndId::Folder(folder.id)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ContainingObjectKind {
    Space(super::Space),
    Object(super::CloudObjectTypeAndId),
}

impl super::Space {
    pub fn into_containing_object(self, app: &AppContext) -> ContainingObject {
        ContainingObject {
            name: self.name(app).clone(),
            kind: ContainingObjectKind::Space(self),
        }
    }
}

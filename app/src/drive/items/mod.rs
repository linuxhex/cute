pub mod ai_fact;
pub mod env_var_collection;
pub mod mcp_server;
pub mod notebook;
pub mod workflow;

use crate::appearance::Appearance;
use crate::cloud_object::Space;
use crate::drive::CloudObjectTypeAndId;
use cuteui::elements::MouseStateHandle;
use cuteui::Element;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WarpDriveItemId {
    Space(Space),
    Object(CloudObjectTypeAndId),
    AIFactCollection,
}

pub trait WarpDriveItem: std::fmt::Debug + Send + Sync {
    fn id(&self) -> &CloudObjectTypeAndId;
    fn title(&self) -> &str;
    fn icon(&self) -> crate::ui_components::icons::Icon;
    fn icon_color(&self, appearance: &Appearance) -> crate::themes::theme::Fill;
    fn subtitle(&self) -> Option<String> {
        None
    }
    fn is_folder(&self) -> bool {
        false
    }
    fn display_name(&self) -> Option<String> {
        None
    }
    fn sync_status_icon(
        &self,
        _is_dequeueing: bool,
        _mouse_state: MouseStateHandle,
        _appearance: &Appearance,
    ) -> Option<Box<dyn Element>> {
        None
    }
}

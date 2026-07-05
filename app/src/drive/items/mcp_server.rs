use crate::appearance::Appearance;
use crate::drive::CloudObjectTypeAndId;
use crate::drive::items::WarpDriveItem;
use crate::themes::theme::Fill;
use crate::ui_components::icons::Icon;

#[derive(Debug, Clone)]
pub struct WarpDriveMCPServer {
    id: CloudObjectTypeAndId,
    name: String,
}

impl WarpDriveMCPServer {
    pub fn new(id: CloudObjectTypeAndId, name: String) -> Self {
        Self { id, name }
    }
}

impl WarpDriveItem for WarpDriveMCPServer {
    fn id(&self) -> &CloudObjectTypeAndId {
        &self.id
    }

    fn title(&self) -> &str {
        &self.name
    }

    fn icon(&self) -> Icon {
        Icon::Dataflow
    }

    fn icon_color(&self, appearance: &Appearance) -> Fill {
        appearance
            .theme()
            .main_text_color(appearance.theme().surface_2())
    }
}

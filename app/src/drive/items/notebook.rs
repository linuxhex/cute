use crate::appearance::Appearance;
use crate::drive::CloudObjectTypeAndId;
use crate::drive::items::WarpDriveItem;
use crate::notebooks::CloudNotebook;
use crate::themes::theme::Fill;
use crate::ui_components::icons::Icon;

#[derive(Debug, Clone)]
pub struct WarpDriveNotebook {
    id: CloudObjectTypeAndId,
    notebook: CloudNotebook,
    is_ai_document: bool,
}

impl WarpDriveNotebook {
    pub fn new(id: CloudObjectTypeAndId, notebook: CloudNotebook, is_ai_document: bool) -> Self {
        Self {
            id,
            notebook,
            is_ai_document,
        }
    }
}

impl WarpDriveItem for WarpDriveNotebook {
    fn id(&self) -> &CloudObjectTypeAndId {
        &self.id
    }

    fn title(&self) -> &str {
        self.notebook.model().title.as_str()
    }

    fn icon(&self) -> Icon {
        Icon::Notebook
    }

    fn icon_color(&self, appearance: &Appearance) -> Fill {
        appearance
            .theme()
            .main_text_color(appearance.theme().surface_2())
    }
}

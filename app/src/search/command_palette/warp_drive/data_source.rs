use crate::search::command_palette::CommandPaletteMixer;
use cuteui::ModelContext;

/// Data source for warp drive items in the command palette.
#[derive(Debug, Clone, Default)]
pub struct DataSource {}

impl DataSource {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self::default()
    }
}

pub fn warp_drive_data_source(_mixer: &mut CommandPaletteMixer) {
}

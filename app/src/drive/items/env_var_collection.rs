#![allow(dead_code)]

use crate::appearance::Appearance;
use crate::drive::CloudObjectTypeAndId;
use crate::drive::items::WarpDriveItem;
use crate::env_vars::CloudEnvVarCollection;
use crate::themes::theme::Fill;
use crate::ui_components::icons::Icon;

#[derive(Debug, Clone)]
pub struct WarpDriveEnvVarCollection {
    id: CloudObjectTypeAndId,
    env_var_collection: CloudEnvVarCollection,
    title: String,
}

impl WarpDriveEnvVarCollection {
    pub fn new(id: CloudObjectTypeAndId, env_var_collection: CloudEnvVarCollection) -> Self {
        let title = env_var_collection
            .model()
            .string_model
            .title
            .clone()
            .unwrap_or_default();
        Self {
            id,
            env_var_collection,
            title,
        }
    }
}

impl WarpDriveItem for WarpDriveEnvVarCollection {
    fn id(&self) -> &CloudObjectTypeAndId {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn icon(&self) -> Icon {
        Icon::EnvVarCollection
    }

    fn icon_color(&self, appearance: &Appearance) -> Fill {
        Fill::Solid(appearance
            .theme()
            .main_text_color(appearance.theme().surface_2())
            .into_solid())
    }
}

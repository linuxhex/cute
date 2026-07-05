use crate::appearance::Appearance;
use crate::drive::CloudObjectTypeAndId;
use crate::drive::items::WarpDriveItem;
use crate::themes::theme::Fill;
use crate::ui_components::icons::Icon;
use crate::workflows::CloudWorkflow;

#[derive(Debug, Clone)]
pub struct WarpDriveWorkflow {
    id: CloudObjectTypeAndId,
    workflow: CloudWorkflow,
    is_agent_mode_workflow: bool,
}

impl WarpDriveWorkflow {
    pub fn new(id: CloudObjectTypeAndId, workflow: CloudWorkflow, is_agent_mode_workflow: bool) -> Self {
        Self {
            id,
            workflow,
            is_agent_mode_workflow,
        }
    }
}

impl WarpDriveItem for WarpDriveWorkflow {
    fn id(&self) -> &CloudObjectTypeAndId {
        &self.id
    }

    fn title(&self) -> &str {
        self.workflow.model().data.name()
    }

    fn icon(&self) -> Icon {
        if self.is_agent_mode_workflow {
            Icon::Prompt
        } else {
            Icon::Workflow
        }
    }

    fn icon_color(&self, appearance: &Appearance) -> Fill {
        appearance
            .theme()
            .main_text_color(appearance.theme().surface_2())
    }
}

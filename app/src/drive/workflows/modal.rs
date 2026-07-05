// Stub module for workflow modal

use cuteui::{AppContext, Element, Entity, View};

/// Events emitted by the workflow modal
#[derive(Debug, Clone)]
pub enum WorkflowModalEvent {
    Close,
    CreateWorkflow,
    AiAssistError(String),
    UpdatedWorkflow(crate::server::ids::ServerId),
    ViewInWarpDrive(crate::server::ids::ServerId),
    AiAssistUpgradeError(crate::server::ids::ServerId, crate::server::ids::ServerId),
}

pub struct WorkflowModal;

impl Entity for WorkflowModal {
    type Event = WorkflowModalEvent;
}

impl View for WorkflowModal {
    fn ui_name() -> &'static str {
        "WorkflowModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl WorkflowModal {
    pub fn new(_ai_client: std::sync::Arc<dyn crate::server::server_api::ai::AIClient>, _ctx: &mut cuteui::ViewContext<Self>) -> Self {
        Self
    }

    pub fn open_with_new(&mut self, _owner: crate::cloud_object::Owner, _initial_folder_id: Option<crate::server::ids::SyncId>, _ctx: &mut cuteui::ViewContext<Self>) {}

    pub fn is_open(&self) -> bool {
        false
    }
}

impl Default for WorkflowModal {
    fn default() -> Self {
        Self
    }
}

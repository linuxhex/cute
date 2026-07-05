use cuteui::{Element, Entity, View, ViewContext};

pub const MIN_SIDEBAR_WIDTH: f32 = 200.0;
pub const MAX_SIDEBAR_WIDTH_RATIO: f32 = 0.3;

#[derive(Debug, Clone)]
pub struct DrivePanel;

#[derive(Debug, Clone)]
pub enum DrivePanelEvent {
    Open,
    Close,
    RunWorkflow(crate::workflows::workflow::Workflow),
    InvokeEnvironmentVariables { env_var_collection: crate::drive::CloudObjectTypeAndId, in_subshell: bool },
    OpenTeamSettingsPage,
    OpenImportModal {
        owner: crate::cloud_object::Owner,
        initial_folder_id: Option<crate::server::ids::SyncId>,
    },
    OpenWorkflowModalWithNew {
        space: crate::cloud_object::Space,
        initial_folder_id: Option<crate::server::ids::SyncId>,
    },
    OpenWorkflowModalWithCloudWorkflow(crate::server::ids::SyncId),
    OpenSearch,
    OpenNotebook(crate::notebooks::manager::NotebookSource),
    OpenEnvVarCollection(crate::env_vars::manager::EnvVarCollectionSource),
    OpenWorkflowInPane(crate::workflows::WorkflowOpenSource, crate::workflows::WorkflowViewMode),
    OpenAIFactCollection,
    OpenMCPServerCollection,
    FocusWarpDrive,
    AttachPlanAsContext(crate::ai::document::ai_document_model::AIDocumentId),
}

impl Entity for DrivePanel {
    type Event = DrivePanelEvent;
}

impl View for DrivePanel {
    fn ui_name() -> &'static str {
        "DrivePanel"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::TypedActionView for DrivePanel {
    type Action = ();
}

impl DrivePanel {
    pub fn set_selected_object(&mut self, _id: Option<crate::drive::items::WarpDriveItemId>, _ctx: &mut ViewContext<Self>) {}

    pub fn create_workflow_with_content(&mut self, _space: crate::cloud_object::Space, _initial_folder_id: Option<crate::server::ids::SyncId>, _content: String, _is_for_agent_mode: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn open_cloud_object_dialog(&mut self, _object_type: crate::drive::DriveObjectType, _space: crate::cloud_object::Space, _initial_folder_id: Option<crate::server::ids::SyncId>, _ctx: &mut ViewContext<Self>) {}

    pub fn undo_trash(&mut self, _cloud_object_type_and_id: crate::drive::CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {}

    pub fn has_warp_drive_initialized_sections(&self, _app: &cuteui::AppContext) -> bool {
        false
    }

    pub fn reset_focused_index_in_warp_drive(&mut self, _should_scroll: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn scroll_item_into_view(&mut self, _item_id: crate::drive::items::WarpDriveItemId, _ctx: &mut ViewContext<Self>) {}

    pub fn expand_section_for_drive_item_id(&mut self, _item_id: crate::drive::items::WarpDriveItemId, _ctx: &mut ViewContext<Self>) {}

    pub fn initialize_drive_section_states(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn reset_and_open_to_main_index(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_focused_item(&mut self, _item_id: crate::drive::items::WarpDriveItemId, _ctx: &mut ViewContext<Self>) {}

    pub fn open_object_sharing_settings(&mut self, _object_type_and_id: crate::drive::CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {}

    pub fn move_object_to_team_owner(&mut self, _object_type_and_id: crate::drive::CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {}

    pub fn set_focused_index(&mut self, _ctx: &mut ViewContext<Self>) {}
}

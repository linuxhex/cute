mod model;

pub(crate) use model::should_disable_snapshot;
pub use model::{AmbientAgentViewModel, AmbientAgentViewModelEvent, Status};

use crate::ai::blocklist::BlocklistAIHistoryModel;
use crate::terminal::model::TerminalModel;
use crate::terminal::view::AgentViewController;
use cuteui::prelude::{ModelHandle, SingletonEntity};
use cuteui::{AppContext, Element};

/// Returns true while the active cloud-agent conversation has `exchange_count() == 0`.
/// This gates: the "Setting up environment" warping indicator, the pre-harness
/// input-box hiding, remote-input suppression, and setup-command rendering.
pub fn is_cloud_agent_pre_first_exchange(
    ambient_agent_view_model: Option<&ModelHandle<AmbientAgentViewModel>>,
    _agent_view_controller: &ModelHandle<AgentViewController>,
    _model: &TerminalModel,
    app: &AppContext,
) -> bool {
    let Some(view_model) = ambient_agent_view_model else {
        return false;
    };

    let view_model = view_model.as_ref(app);

    // If the harness command has started, we're past the pre-first-exchange phase.
    if view_model.harness_command_started() {
        return false;
    }

    // Check if the conversation has any exchanges.
    let Some(conversation_id) = view_model.conversation_id() else {
        return false;
    };

    BlocklistAIHistoryModel::as_ref(app)
        .conversation(conversation_id)
        .is_some_and(|conversation| conversation.exchange_count() == 0)
}

// ---------------------------------------------------------------------------
// Stubs: these types are imported elsewhere in the crate but their real
// implementations live in not-yet-integrated submodules. The minimal
// placeholders below keep the `crate::terminal::view::ambient_agent` exports
// resolving. They are intentionally empty.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct ModelSelector;

impl cuteui::Entity for ModelSelector {
    type Event = ();
}

impl cuteui::View for ModelSelector {
    fn ui_name() -> &'static str {
        "ModelSelector"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl ModelSelector {
    /// Returns whether the model selector menu is currently open.
    pub fn is_menu_open(&self) -> bool {
        false
    }

    /// Opens the model selector menu.
    pub fn open_menu(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthSecretFtuxView;

impl AuthSecretFtuxView {
    pub fn focus_dropdown_editor(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn select_previous_in_dropdown(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn select_next_in_dropdown(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }
}

impl cuteui::View for AuthSecretFtuxView {
    fn ui_name() -> &'static str {
        "AuthSecretFtuxView"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::Entity for AuthSecretFtuxView {
    type Event = ();
}

#[derive(Debug, Clone, Default)]
pub struct AuthSecretSelector {
    is_menu_open: bool,
}

impl AuthSecretSelector {
    pub fn delete_confirmation_dialog_element(&self) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }

    pub fn is_menu_open(&self) -> bool {
        self.is_menu_open
    }

    pub fn select_previous(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn select_next(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }
}

impl cuteui::View for AuthSecretSelector {
    fn ui_name() -> &'static str {
        "AuthSecretSelector"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::Entity for AuthSecretSelector {
    type Event = ();
}

#[derive(Debug, Clone, Default)]
pub struct HarnessSelector {
    is_menu_open: bool,
}

#[derive(Debug, Clone)]
pub enum HarnessSelectorEvent {
    Default,
    MenuVisibilityChanged { open: bool },
}

#[derive(Debug, Clone)]
pub enum HarnessSelectorAction {
    OpenMenu,
}

impl HarnessSelector {
    pub fn new(
        _menu_positioning_provider: std::sync::Arc<dyn crate::terminal::input::MenuPositioningProvider>,
        _view_model: cuteui::ModelHandle<model::AmbientAgentViewModel>,
        _ctx: &mut cuteui::ViewContext<Self>,
    ) -> Self {
        Self::default()
    }

    pub fn open_menu(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        self.is_menu_open = true;
    }

    pub fn is_menu_open(&self) -> bool {
        self.is_menu_open
    }
}

impl cuteui::View for HarnessSelector {
    fn ui_name() -> &'static str {
        "HarnessSelector"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::Entity for HarnessSelector {
    type Event = HarnessSelectorEvent;
}

impl cuteui::TypedActionView for HarnessSelector {
    type Action = HarnessSelectorAction;
}

#[derive(Debug, Clone, Default)]
pub struct HostSelector {
    is_menu_open: bool,
}

impl HostSelector {
    pub fn open_menu(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        self.is_menu_open = true;
    }

    pub fn is_menu_open(&self) -> bool {
        self.is_menu_open
    }
}

impl cuteui::View for HostSelector {
    fn ui_name() -> &'static str {
        "HostSelector"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::Entity for HostSelector {
    type Event = ();
}

#[derive(Debug, Clone, Default)]
pub struct AmbientAgentEntryBlock;

#[derive(Debug, Clone, Default)]
pub enum HandoffSubmissionState {
    #[default]
    Idle,
    Default,
}

#[derive(Debug, Clone, Default)]
pub struct PendingHandoff {
    pub forked_conversation_id: Option<String>,
    pub title: Option<String>,
    pub touched_workspace: Option<String>,
    pub snapshot_upload: SnapshotUploadStatus,
    pub submission_state: HandoffSubmissionState,
    pub auto_submit: bool,
}

#[derive(Debug, Clone, Default)]
pub enum SnapshotUploadStatus {
    #[default]
    Pending,
    Default,
}

/// Stub for first-time cloud agent setup view.
/// This view is used for creating the initial environment for cloud agent.
#[derive(Debug, Clone, Default)]
pub struct FirstTimeCloudAgentSetupView;

/// Stub action for FirstTimeCloudAgentSetupView.
#[derive(Debug, Clone)]
pub enum FirstTimeCloudAgentSetupAction {
    SetupComplete,
}

impl FirstTimeCloudAgentSetupView {
    pub fn new(_ctx: &mut cuteui::ViewContext<Self>) -> Self {
        Self::default()
    }
}

impl cuteui::View for FirstTimeCloudAgentSetupView {
    fn ui_name() -> &'static str {
        "FirstTimeCloudAgentSetupView"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn cuteui::Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl cuteui::Entity for FirstTimeCloudAgentSetupView {
    type Event = ();
}

impl cuteui::TypedActionView for FirstTimeCloudAgentSetupView {
    type Action = FirstTimeCloudAgentSetupAction;
}

/// Stub for cloud mode tip type used in agent tips.
/// Implements the AITip trait for use with AITipModel.
#[derive(Debug, Clone, Default)]
pub struct CloudModeTip;

impl crate::ai::agent_tips::AITip for CloudModeTip {
    fn keystroke(&self, _app: &cuteui::AppContext) -> Option<cuteui::keymap::Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        None
    }

    fn description(&self) -> &str {
        ""
    }
}

/// Stub function for rendering loading footer.
/// Placeholder for not-yet-integrated loading footer rendering.
pub fn render_loading_footer(_appearance: &crate::appearance::Appearance) -> Box<dyn cuteui::Element> {
    cuteui::elements::Empty::new().finish()
}

/// Stub function for creating cloud mode view.
/// Cloud features are disabled in local version.
/// This function should not be called in local builds.
pub fn create_cloud_mode_view(
    _resources: crate::pane_group::TerminalViewResources,
    _view_bounds_size: pathfinder_geometry::vector::Vector2F,
    _window_id: cuteui::WindowId,
    _enable_orchestration_polling: bool,
    _ctx: &mut cuteui::ViewContext<crate::pane_group::PaneGroup>,
) -> (
    cuteui::ViewHandle<crate::terminal::view::TerminalView>,
    cuteui::ModelHandle<Box<dyn crate::terminal::terminal_manager::TerminalManager>>,
) {
    // Cloud features are disabled in local version.
    // This stub should never be called in local builds.
    panic!("create_cloud_mode_view called in local build - cloud features are disabled")
}

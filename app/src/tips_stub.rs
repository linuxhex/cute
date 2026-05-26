// Minimal stubs for resource_center types for minimal terminal build

use serde::{Deserialize, Serialize};

/// Stub implementation for minimal terminal
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TipsCompleted {
    pub skipped_or_completed: bool,
}

impl TipsCompleted {
    pub fn new(_features_used: Vec<String>, skipped_or_completed: bool) -> Self {
        Self {
            skipped_or_completed,
        }
    }
}

/// Stub for Tip
#[derive(Clone, Debug, Default)]
pub struct Tip;

/// Stub for TipAction
#[derive(Clone, Debug, Default)]
pub struct TipAction;

/// Stub for TipHint
#[derive(Clone, Debug, Default)]
pub struct TipHint;

/// No-op stub for mark_feature_used_and_write_to_user_defaults
pub fn mark_feature_used_and_write_to_user_defaults(_feature: &str, _ctx: &mut cuteui::AppContext) {
    // No-op for minimal terminal
}

/// Stub for skip_tips_and_write_to_user_defaults
pub fn skip_tips_and_write_to_user_defaults(_ctx: &mut cuteui::AppContext) {
    // No-op for minimal terminal
}

/// Stub for ResourceCenterPage
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ResourceCenterPage {
    #[default]
    Main,
    Keybindings,
}

/// Stub for ResourceCenterEvent
#[derive(Clone, Debug)]
pub enum ResourceCenterEvent {
    Close,
    Escape,
}

/// Stub for ResourceCenterView (empty type)
pub struct ResourceCenterView;

impl ResourceCenterView {
    pub fn new(
        _ctx: &mut cuteui::ViewContext<Self>,
        _tips_completed: cuteui::ModelHandle<TipsCompleted>,
        _changelog_model: cuteui::ModelHandle<crate::changelog_model::ChangelogModel>,
    ) -> Self {
        Self
    }

    pub fn set_current_page(&mut self, _page: ResourceCenterPage, _ctx: &mut cuteui::ViewContext<Self>) {
        // No-op
    }

    pub fn get_current_page(&self) -> ResourceCenterPage {
        ResourceCenterPage::Main
    }

    pub fn set_action_target(
        &mut self,
        _window_id: cuteui::WindowId,
        _input_id: Option<crate::terminal::input::InputId>,
        _ctx: &mut cuteui::ViewContext<Self>,
    ) {
        // No-op
    }
}

impl cuteui::ActionView for ResourceCenterView {
    fn on_action(
        &mut self,
        _action: &cuteui::Action,
        _ctx: &mut cuteui::ViewContext<Self>,
    ) -> bool {
        false
    }
}

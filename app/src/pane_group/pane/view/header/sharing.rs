//! Support for pane contents that are shareable, like sessions and Warp Drive objects.
//!
//! This is tightly coupled to the pane header so that different overlays (context menus, etc.)
//! are correctly displayed.

use cute_core::ui::appearance::Appearance;
use cuteui::elements::MouseStateHandle;
use cuteui::AppContext;

use super::PaneHeader;
use crate::pane_group::BackingView;

/// Pane header component for sharing the pane contents.
/// In the local version, sharing is disabled so this is a minimal stub.
pub struct SharedPaneContent {
    _primary_button_handle: MouseStateHandle,
    _view_only_icon_handle: MouseStateHandle,
}

impl SharedPaneContent {
    pub fn new<P: BackingView>(_ctx: &mut cuteui::ViewContext<PaneHeader<P>>) -> Self {
        Self {
            _primary_button_handle: Default::default(),
            _view_only_icon_handle: Default::default(),
        }
    }
}

impl<P: BackingView> PaneHeader<P> {
    pub fn set_shareable_object(
        &mut self,
        _shareable_object: Option<crate::cloud_stub_types::sharing::ShareableObject>,
        _ctx: &mut cuteui::ViewContext<Self>,
    ) {
        // No-op in local version
    }

    pub fn is_sharing_dialog_enabled<C: cuteui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    pub fn has_shareable_shared_session<C: cuteui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    pub fn has_shareable_object<C: cuteui::ViewAsRef>(&self, _ctx: &C) -> bool {
        false
    }

    /// Render controls for sharing the pane contents.
    /// In the local version, sharing is disabled so this renders nothing.
    pub fn render_sharing_controls(
        &self,
        _element: &mut impl cuteui::elements::ParentElement,
        _appearance: &Appearance,
        _icon_color_override: Option<cute_core::ui::theme::Fill>,
        _button_size_override: Option<f32>,
        _app: &AppContext,
    ) {
        // No-op in local version - no sharing controls rendered
    }
}

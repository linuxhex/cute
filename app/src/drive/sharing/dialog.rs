use cuteui::{Element, TypedActionView, ViewContext};
use cuteui::elements::Empty;

use super::{ContentEditability, ShareableObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharingDialogSource {
    PaneHeader,
    Notebook,
    Workflow,
    EnvVarCollection,
    StartedSessionShare,
    InviteeRequest,
    AIBlockContextMenu,
    ConversationList,
    CommandPalette,
    OnboardingBlock,
}

pub struct SharingDialog {
    target: Option<ShareableObject>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharingDialogEvent {
    Close,
}

impl SharingDialog {
    pub fn new(target: Option<ShareableObject>, _ctx: &mut ViewContext<Self>) -> Self {
        Self { target }
    }

    pub fn set_target(&mut self, target: Option<ShareableObject>, ctx: &mut ViewContext<Self>) {
        self.target = target;
        ctx.notify();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn has_shared_session_target(&self) -> bool {
        self.target
            .as_ref()
            .is_some_and(|target| matches!(target, ShareableObject::Session { .. }))
    }

    pub fn editability(&self, _app: &cuteui::AppContext) -> ContentEditability {
        // Simplified: treat all objects as editable for now
        ContentEditability::Editable
    }

    pub fn is_unsharable_conversation(&self, _app: &cuteui::AppContext) -> bool {
        // Simplified: no unsharable conversations in current implementation
        false
    }

    pub fn copy_link(&self, ctx: &mut ViewContext<Self>) {
        if let Some(url) = self.target.as_ref().and_then(|target| target.link(ctx)) {
            ctx.clipboard().write(cuteui::clipboard::ClipboardContent::plain_text(url));
        }
    }

    pub fn show_qr_code(&mut self, ctx: &mut ViewContext<Self>) {
        // Simplified: just notify that we're showing QR code mode
        ctx.notify();
    }

    pub fn report_open(&self, _source: SharingDialogSource, _ctx: &mut ViewContext<Self>) {
        // No-op in simplified implementation
    }
}

impl cuteui::Entity for SharingDialog {
    type Event = SharingDialogEvent;
}

impl cuteui::View for SharingDialog {
    fn ui_name() -> &'static str {
        "SharingDialog"
    }

    fn render(&self, _app: &cuteui::AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for SharingDialog {
    type Action = SharingDialogEvent;
}

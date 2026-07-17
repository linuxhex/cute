use cuteui::elements::{Element, Empty};
use cuteui::{AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::view_components::DismissibleToast;
use crate::workspace::ToastStack;

#[derive(Debug, Clone)]
pub enum CreateEnvironmentModalEvent {
    Cancelled,
    Created { environment_id: String },
}

pub struct CreateEnvironmentModal {
    visible: bool,
}

impl CreateEnvironmentModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self { visible: false }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn show(&mut self, ctx: &mut ViewContext<Self>) {
        self.visible = true;
        // Removed: handoff environment creation modal not needed in local version.
        // Emit Cancelled immediately so callers are not left waiting.
        ctx.emit(CreateEnvironmentModalEvent::Cancelled);
        ctx.notify();
    }

    pub fn hide(&mut self, ctx: &mut ViewContext<Self>) {
        self.visible = false;
        ctx.notify();
    }

    fn cancel(&mut self, ctx: &mut ViewContext<Self>) {
        self.hide(ctx);
        ctx.emit(CreateEnvironmentModalEvent::Cancelled);
    }

    #[allow(dead_code)]
    fn show_error_toast(&self, message: String, ctx: &mut ViewContext<Self>) {
        let window_id = ctx.window_id();
        ToastStack::handle(ctx).update(ctx, |toast_stack, ctx| {
            toast_stack.add_ephemeral_toast(DismissibleToast::error(message), window_id, ctx);
        });
    }
}

impl Entity for CreateEnvironmentModal {
    type Event = CreateEnvironmentModalEvent;
}

impl TypedActionView for CreateEnvironmentModal {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for CreateEnvironmentModal {
    fn ui_name() -> &'static str {
        "CreateEnvironmentModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

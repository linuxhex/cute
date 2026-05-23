use pathfinder_geometry::vector::vec2f;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Align, ChildAnchor, ChildView, Container, OffsetPositioning, ParentAnchor, ParentOffsetBounds,
    Stack,
};
use warpui::keymap::{FixedBinding, Keystroke};
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::{
    AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

// TODO: Remove AI dependency
// use crate::ai::agent::conversation::AIConversationId;
use crate::appearance::Appearance;
use crate::ui_components::dialog::{dialog_styles, Dialog};
use crate::view_components::action_button::{
    ActionButton, DangerPrimaryTheme, KeystrokeSource, NakedTheme,
};

pub fn init(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([
        FixedBinding::new(
            "escape",
            DeleteConversationConfirmationAction::Cancel,
            id!(DeleteConversationConfirmationDialog::ui_name()),
        ),
        FixedBinding::new(
            "enter",
            DeleteConversationConfirmationAction::Confirm,
            id!(DeleteConversationConfirmationDialog::ui_name()),
        ),
    ]);
}

const DIALOG_WIDTH: f32 = 460.;

// TODO: Remove AI dependency
// #[derive(Clone)]
// pub struct DeleteConversationDialogSource {
//     pub conversation_id: AIConversationId,
//     pub conversation_title: String,
//     pub terminal_view_id: Option<warpui::EntityId>,
// }

pub struct DeleteConversationConfirmationDialog {
    cancel_button: ViewHandle<ActionButton>,
    delete_button: ViewHandle<ActionButton>,
    // TODO: Remove AI dependency
    // source: Option<DeleteConversationDialogSource>,
    conversation_title: Option<String>,
}

impl DeleteConversationConfirmationDialog {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let cancel_button = ctx.add_typed_action_view(|_| {
            ActionButton::new("Cancel", NakedTheme).on_click(|ctx| {
                ctx.dispatch_typed_action(DeleteConversationConfirmationAction::Cancel);
            })
        });

        let enter_keystroke = Keystroke::parse("enter").expect("Valid keystroke");
        let delete_button = ctx.add_typed_action_view(|ctx| {
            ActionButton::new("Delete", DangerPrimaryTheme)
                .with_keybinding(KeystrokeSource::Fixed(enter_keystroke), ctx)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(DeleteConversationConfirmationAction::Confirm);
                })
        });

        Self {
            cancel_button,
            delete_button,
            // TODO: Remove AI dependency
            // source: None,
            conversation_title: None,
        }
    }

    // TODO: Remove AI dependency
    // pub fn set_source(&mut self, source: DeleteConversationDialogSource) {
    //     self.source = Some(source);
    // }

    pub fn set_conversation_title(&mut self, title: String) {
        self.conversation_title = Some(title);
    }
}

impl Entity for DeleteConversationConfirmationDialog {
    type Event = DeleteConversationConfirmationEvent;
}

impl View for DeleteConversationConfirmationDialog {
    fn ui_name() -> &'static str {
        "DeleteConversationConfirmationDialog"
    }

    fn on_focus(&mut self, _focus_ctx: &warpui::FocusContext, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);

        let cancel_button = Container::new(ChildView::new(&self.cancel_button).finish())
            .with_margin_right(12.)
            .finish();

        let title = self
            .conversation_title
            .as_ref()
            .map(|s| format!("Delete '{}'?", s))
            .unwrap_or_else(|| "Delete conversation?".into());

        let dialog = Dialog::new(
            title,
            Some(
                "This conversation will be permanently deleted. This action cannot be undone."
                    .into(),
            ),
            UiComponentStyles {
                width: Some(DIALOG_WIDTH),
                ..dialog_styles(appearance)
            },
        )
        .with_bottom_row_child(cancel_button)
        .with_bottom_row_child(ChildView::new(&self.delete_button).finish())
        .build()
        .finish();

        let mut stack = Stack::new();
        stack.add_positioned_child(
            dialog,
            OffsetPositioning::offset_from_parent(
                vec2f(0., 0.),
                ParentOffsetBounds::WindowByPosition,
                ParentAnchor::Center,
                ChildAnchor::Center,
            ),
        );

        Container::new(Align::new(stack.finish()).finish())
            .with_background_color(Fill::blur().into())
            .with_corner_radius(app.windows().window_corner_radius())
            .finish()
    }
}

// TODO: Remove AI dependency
// pub enum DeleteConversationConfirmationEvent {
//     Confirm {
//         source: DeleteConversationDialogSource,
//     },
//     Cancel,
// }
pub enum DeleteConversationConfirmationEvent {
    Cancel,
}

#[derive(Debug)]
pub enum DeleteConversationConfirmationAction {
    Confirm,
    Cancel,
}

impl TypedActionView for DeleteConversationConfirmationDialog {
    type Action = DeleteConversationConfirmationAction;

    fn handle_action(
        &mut self,
        action: &DeleteConversationConfirmationAction,
        ctx: &mut ViewContext<Self>,
    ) {
        match action {
            DeleteConversationConfirmationAction::Confirm => {
                // TODO: Remove AI dependency
                // let Some(source) = self.source.clone() else {
                //     log::error!("Delete confirm button pressed with no source");
                //     return;
                // };
                // ctx.emit(DeleteConversationConfirmationEvent::Confirm { source });
                log::warn!("Delete confirm button pressed but AI dependency removed");
            }
            DeleteConversationConfirmationAction::Cancel => {
                ctx.emit(DeleteConversationConfirmationEvent::Cancel);
            }
        }
    }
}

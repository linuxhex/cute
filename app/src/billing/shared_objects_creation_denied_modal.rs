//! Shared objects creation denied modal.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use warpui::keymap::FixedBinding;
use warpui::presenter::ChildView;
use warpui::ui_components::components::UiComponentStyles;
use warpui::{
    AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

use super::shared_objects_creation_denied_body::{
    SharedObjectsCreationDeniedBody, SharedObjectsCreationDeniedBodyEvent,
};
use crate::drive::DriveObjectType;
use crate::modal::{Modal, ModalEvent};
use crate::server::ids::ServerId;
use crate::workspaces::workspace::CustomerType;

const DEFAULT_LIMIT_REACHED_MODAL_HEADER: &str = "Shared object limit reached";

pub struct SharedObjectsCreationDeniedModal {
    shared_objects_creation_denied_modal: ViewHandle<Modal<SharedObjectsCreationDeniedBody>>,
    team_uid: Option<ServerId>,
}

#[derive(Debug)]
pub enum SharedObjectsCreationDeniedModalAction {
    Close,
}

#[allow(dead_code)]
pub enum SharedObjectsCreationDeniedModalEvent {
    Close,
    TeamSettings,
}

pub fn init(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([FixedBinding::new(
        "escape",
        SharedObjectsCreationDeniedModalAction::Close,
        id!("SharedObjectsCreationDeniedModal"),
    )]);
}

impl SharedObjectsCreationDeniedModal {
    pub fn new(object_type: Option<DriveObjectType>, ctx: &mut ViewContext<Self>) -> Self {
        let shared_objects_creation_denied_body = ctx.add_typed_action_view(
            |_ctx: &mut ViewContext<'_, SharedObjectsCreationDeniedBody>| {
                SharedObjectsCreationDeniedBody::new(object_type)
            },
        );

        ctx.subscribe_to_view(
            &shared_objects_creation_denied_body,
            move |me, _, event, ctx| {
                me.handle_shared_objects_creation_denied_body_event(event, ctx);
            },
        );

        let shared_objects_creation_denied_modal = ctx.add_typed_action_view(|ctx| {
            Modal::new(
                Some(DEFAULT_LIMIT_REACHED_MODAL_HEADER.into()),
                shared_objects_creation_denied_body,
                ctx,
            )
            .with_modal_style(UiComponentStyles {
                width: Some(355.),
                ..Default::default()
            })
            .with_dismiss_on_click()
        });
        ctx.subscribe_to_view(
            &shared_objects_creation_denied_modal,
            |me, _, event, ctx| match event {
                ModalEvent::Close => me.close(ctx),
            },
        );

        Self {
            shared_objects_creation_denied_modal,
            team_uid: None,
        }
    }

    pub fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(SharedObjectsCreationDeniedModalEvent::Close);
    }

    #[allow(dead_code)]
    pub fn update_modal_state(
        &mut self,
        team_uid: ServerId,
        object_type: DriveObjectType,
        has_admin_permissions: bool,
        is_delinquent_due_to_payment_issue: bool,
        customer_type: CustomerType,
        ctx: &mut ViewContext<Self>,
    ) {
        self.team_uid = Some(team_uid);
        self.shared_objects_creation_denied_modal
            .update(ctx, |modal, ctx| {
                modal
                    .body()
                    .update(ctx, |shared_objects_creation_denied_body, ctx| {
                        shared_objects_creation_denied_body.update_state(
                            object_type,
                            has_admin_permissions,
                            is_delinquent_due_to_payment_issue,
                            customer_type,
                            ctx,
                        );
                    });
                ctx.notify();
            });
    }

    fn handle_shared_objects_creation_denied_body_event(
        &mut self,
        _event: &SharedObjectsCreationDeniedBodyEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        // Simplified: just close the modal
        ctx.emit(SharedObjectsCreationDeniedModalEvent::Close);
    }
}

impl Entity for SharedObjectsCreationDeniedModal {
    type Event = SharedObjectsCreationDeniedModalEvent;
}

impl View for SharedObjectsCreationDeniedModal {
    fn ui_name() -> &'static str {
        "SharedObjectsCreationDeniedModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        ChildView::new(&self.shared_objects_creation_denied_modal).finish()
    }
}

impl TypedActionView for SharedObjectsCreationDeniedModal {
    type Action = SharedObjectsCreationDeniedModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SharedObjectsCreationDeniedModalAction::Close => self.close(ctx),
        }
    }
}

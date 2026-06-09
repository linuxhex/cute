//! Shared objects creation denied body.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use warpui::elements::{Container, Flex, Padding, ParentElement, Text};
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::appearance::Appearance;
use crate::drive::DriveObjectType;
use crate::workspaces::workspace::CustomerType;

pub struct SharedObjectsCreationDeniedBody {
    object_type: Option<DriveObjectType>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum SharedObjectsCreationDeniedBodyAction {
    Upgrade,
    ManageBilling,
}

#[allow(dead_code)]
pub enum SharedObjectsCreationDeniedBodyEvent {
    Upgrade,
    ManageBilling,
}

impl SharedObjectsCreationDeniedBody {
    pub fn new(object_type: Option<DriveObjectType>) -> Self {
        Self { object_type }
    }

    #[allow(dead_code)]
    pub fn update_state(
        &mut self,
        object_type: DriveObjectType,
        _has_admin_permissions: bool,
        _is_delinquent_due_to_payment_issue: bool,
        _customer_type: CustomerType,
        ctx: &mut ViewContext<Self>,
    ) {
        self.object_type = Some(object_type);
        ctx.notify();
    }
}

impl Entity for SharedObjectsCreationDeniedBody {
    type Event = SharedObjectsCreationDeniedBodyEvent;
}

impl View for SharedObjectsCreationDeniedBody {
    fn ui_name() -> &'static str {
        "SharedObjectsCreationDeniedBody"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let text_color = theme.main_text_color(theme.surface_1()).into_solid();

        let message = match self.object_type {
            Some(ot) => format!("Shared {} limit reached", ot),
            None => "Shared object limit reached".to_string(),
        };

        Container::new(
            Flex::column()
                .with_child(
                    Text::new(message, appearance.ui_font_family(), 14.)
                        .with_color(text_color)
                        .finish(),
                )
                .finish(),
        )
        .with_padding(Padding::uniform(16.0))
        .finish()
    }
}

impl TypedActionView for SharedObjectsCreationDeniedBody {
    type Action = SharedObjectsCreationDeniedBodyAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SharedObjectsCreationDeniedBodyAction::Upgrade => {
                ctx.emit(SharedObjectsCreationDeniedBodyEvent::Upgrade)
            }
            SharedObjectsCreationDeniedBodyAction::ManageBilling => {
                ctx.emit(SharedObjectsCreationDeniedBodyEvent::ManageBilling)
            }
        }
    }
}

impl SingletonEntity for SharedObjectsCreationDeniedBody {}

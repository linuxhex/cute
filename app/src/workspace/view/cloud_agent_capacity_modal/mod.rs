//! Cloud agent capacity modal.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::appearance::Appearance;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Align, ChildAnchor, ConstrainedBox, Container, Flex, MainAxisSize, MouseStateHandle,
    OffsetPositioning, Padding, ParentAnchor, ParentElement, ParentOffsetBounds, Stack, Text,
};
use warpui::keymap::FixedBinding;
use warpui::platform::Cursor;
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::ui_components::blended_colors;

const MODAL_WIDTH: f32 = 360.;
const MODAL_HEIGHT: f32 = 200.;
const BUTTON_DIAMETER: f32 = 20.;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum CloudAgentCapacityModalVariant {
    #[default]
    ConcurrentLimit,
    OutOfCredits,
}

pub fn init(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([FixedBinding::new(
        "escape",
        CloudAgentCapacityModalAction::Close,
        id!("CloudAgentCapacityModal"),
    )]);
}

#[derive(Default)]
struct StateHandles {
    close_button: MouseStateHandle,
}

pub struct CloudAgentCapacityModal {
    state_handles: StateHandles,
    variant: CloudAgentCapacityModalVariant,
}

impl CloudAgentCapacityModal {
    pub fn new() -> Self {
        CloudAgentCapacityModal {
            state_handles: Default::default(),
            variant: CloudAgentCapacityModalVariant::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_variant(&mut self, variant: CloudAgentCapacityModalVariant) {
        self.variant = variant;
    }
}

impl Entity for CloudAgentCapacityModal {
    type Event = CloudAgentCapacityModalEvent;
}

impl View for CloudAgentCapacityModal {
    fn ui_name() -> &'static str {
        "CloudAgentCapacityModal"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let text_color = theme.main_text_color(theme.surface_1()).into_solid();

        let (title, message) = match self.variant {
            CloudAgentCapacityModalVariant::ConcurrentLimit => (
                "Concurrent agent limit reached",
                "You have reached the maximum number of concurrent agents.",
            ),
            CloudAgentCapacityModalVariant::OutOfCredits => (
                "Out of AI credits",
                "You have used all available AI credits.",  // Simplified: no billing period
            ),
        };

        let close_button = appearance
            .ui_builder()
            .close_button(BUTTON_DIAMETER, self.state_handles.close_button.clone())
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(|ctx, _, _| ctx.dispatch_typed_action(CloudAgentCapacityModalAction::Close))
            .finish();

        let content = Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_child(
                Text::new(title, appearance.ui_font_family(), 16.)
                    .with_color(text_color)
                    .finish(),
            )
            .with_child(
                Container::new(
                    Text::new(message, appearance.ui_font_family(), 14.)
                        .with_color(text_color)
                        .finish(),
                )
                .with_margin_top(8.)
                .finish(),
            )
            .finish();

        let modal_content = Container::new(content)
            .with_padding(Padding::uniform(32.))
            .with_background_color(blended_colors::neutral_1(theme))
            .finish();

        let modal_box = ConstrainedBox::new(modal_content)
            .with_width(MODAL_WIDTH)
            .with_height(MODAL_HEIGHT)
            .finish();

        let mut modal = Stack::new();
        modal.add_child(modal_box);
        modal.add_positioned_child(
            close_button,
            OffsetPositioning::offset_from_parent(
                vec2f(-8., 8.),
                ParentOffsetBounds::ParentByPosition,
                ParentAnchor::TopRight,
                ChildAnchor::TopRight,
            ),
        );

        let mut stack = Stack::new();
        stack.add_positioned_child(
            modal.finish(),
            OffsetPositioning::offset_from_parent(
                vec2f(0., 0.),
                ParentOffsetBounds::WindowByPosition,
                ParentAnchor::Center,
                ChildAnchor::Center,
            ),
        );

        Container::new(Align::new(stack.finish()).finish())
            .with_background(Fill::Solid(ColorU::new(97, 97, 97, 255)).with_opacity(50))
            .finish()
    }
}

impl TypedActionView for CloudAgentCapacityModal {
    type Action = CloudAgentCapacityModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CloudAgentCapacityModalAction::Close => {
                ctx.emit(CloudAgentCapacityModalEvent::Close);
            }
            CloudAgentCapacityModalAction::Upgrade => {
                ctx.emit(CloudAgentCapacityModalEvent::Close);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CloudAgentCapacityModalEvent {
    Close,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum CloudAgentCapacityModalAction {
    Close,
    Upgrade,
}

//! Free tier limit hit modal.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::appearance::Appearance;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Align, ChildAnchor, ConstrainedBox, Container, Flex,
    MainAxisSize, MouseStateHandle, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Stack,
};
use warpui::keymap::FixedBinding;
use warpui::platform::Cursor;
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::ui_components::blended_colors;

const BUTTON_DIAMETER: f32 = 20.;
const MODAL_WIDTH: f32 = 400.;
const MODAL_HEIGHT: f32 = 200.;

pub fn init(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([FixedBinding::new(
        "escape",
        FreeTierLimitHitModalAction::Close,
        id!("FreeTierLimitHitModal"),
    )]);
}

#[derive(Default)]
struct StateHandles {
    close_button: MouseStateHandle,
}

pub struct FreeTierLimitHitModal {
    state_handles: StateHandles,
}

impl FreeTierLimitHitModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        FreeTierLimitHitModal {
            state_handles: Default::default(),
        }
    }
}

impl Entity for FreeTierLimitHitModal {
    type Event = FreeTierLimitHitModalEvent;
}

impl View for FreeTierLimitHitModal {
    fn ui_name() -> &'static str {
        "FreeTierLimitHitModal"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let text_color = theme.main_text_color(theme.surface_1()).into_solid();

        let close_button = appearance
            .ui_builder()
            .close_button(BUTTON_DIAMETER, self.state_handles.close_button.clone())
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(|ctx, _, _| ctx.dispatch_typed_action(FreeTierLimitHitModalAction::Close))
            .finish();

        let content = Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_child(
                warpui::elements::Text::new("AI credits limit reached", appearance.ui_font_family(), 18.)
                    .with_color(text_color)
                    .finish(),
            )
            .with_child(
                Container::new(
                    warpui::elements::Text::new(
                        "You have reached the AI credits limit for this session.",
                        appearance.ui_font_family(),
                        14.,
                    )
                    .with_color(text_color)
                    .finish(),
                )
                .with_margin_top(8.)
                .finish(),
            )
            .finish();

        let modal_content = Container::new(content)
            .with_uniform_padding(32.)
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

impl TypedActionView for FreeTierLimitHitModal {
    type Action = FreeTierLimitHitModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            FreeTierLimitHitModalAction::Close => {
                ctx.emit(FreeTierLimitHitModalEvent::Close);
            }
            FreeTierLimitHitModalAction::OpenUpgrade => {
                ctx.emit(FreeTierLimitHitModalEvent::Close);
            }
            FreeTierLimitHitModalAction::OpenUrl(_url) => {
                // Simplified: no URL opening
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FreeTierLimitHitModalEvent {
    #[allow(dead_code)]
    MaybeOpen,
    Close,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum FreeTierLimitHitModalAction {
    Close,
    OpenUpgrade,
    OpenUrl(String),
}

impl SingletonEntity for FreeTierLimitHitModal {}

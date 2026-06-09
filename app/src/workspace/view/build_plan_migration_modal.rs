//! Build plan migration modal.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use warp_core::ui::appearance::Appearance;
use warp_core::ui::theme::Fill;
use warpui::elements::{
    Align, ChildAnchor, ConstrainedBox, Container, CornerRadius, Flex,
    MainAxisSize, MouseStateHandle, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Radius, Stack,
};
use warpui::keymap::FixedBinding;
use warpui::ui_components::components::UiComponent;
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::ui_components::blended_colors;

const BUTTON_DIAMETER: f32 = 20.;
const MODAL_WIDTH: f32 = 400.;
const MODAL_HEIGHT: f32 = 200.;
const PANEL_PADDING: f32 = 24.;
const CORNER_RADIUS: f32 = 10.;

#[allow(dead_code)]
pub fn init(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    app.register_fixed_bindings([FixedBinding::new(
        "escape",
        BuildPlanMigrationModalViewAction::Close,
        id!("BuildPlanMigrationModal"),
    )]);
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BuildPlanMigrationModalViewAction {
    GetStartedClicked,
    Close,
    OpenUrl(&'static str),
}

#[derive(Default)]
struct StateHandles {
    close_button: MouseStateHandle,
}

pub struct BuildPlanMigrationModal {
    state_handles: StateHandles,
    is_updating: bool,
}

impl BuildPlanMigrationModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        BuildPlanMigrationModal {
            state_handles: Default::default(),
            is_updating: false,
        }
    }

    fn render_close_button(&self, appearance: &Appearance) -> Box<dyn Element> {
        appearance
            .ui_builder()
            .close_button(BUTTON_DIAMETER, self.state_handles.close_button.clone())
            .build()
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(BuildPlanMigrationModalViewAction::Close)
            })
            .finish()
    }

    fn render_content(&self, appearance: &Appearance) -> Box<dyn Element> {
        let theme = appearance.theme();
        let text_color = blended_colors::text_main(theme, blended_colors::neutral_2(theme));

        Flex::column()
            .with_main_axis_size(MainAxisSize::Min)
            .with_child(
                warpui::elements::Text::new("Plan Updated", appearance.ui_font_family(), 24.)
                    .with_color(text_color)
                    .finish(),
            )
            .with_child(
                Container::new(
                    warpui::elements::Text::new(
                        "Your workspace has been updated to the new plan.",
                        appearance.ui_font_family(),
                        14.,
                    )
                    .with_color(blended_colors::text_sub(theme, blended_colors::neutral_2(theme)))
                    .finish(),
                )
                .with_margin_top(12.)
                .finish(),
            )
            .finish()
    }
}

impl Entity for BuildPlanMigrationModal {
    type Event = BuildPlanMigrationModalEvent;
}

impl View for BuildPlanMigrationModal {
    fn ui_name() -> &'static str {
        "BuildPlanMigrationModal"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        let content = Container::new(self.render_content(appearance))
            .with_background_color(blended_colors::neutral_1(theme))
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(CORNER_RADIUS)))
            .with_uniform_padding(PANEL_PADDING)
            .finish();

        let modal_box = ConstrainedBox::new(content)
            .with_width(MODAL_WIDTH)
            .with_height(MODAL_HEIGHT)
            .finish();

        let close_button = self.render_close_button(appearance);

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

impl TypedActionView for BuildPlanMigrationModal {
    type Action = BuildPlanMigrationModalViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            BuildPlanMigrationModalViewAction::GetStartedClicked => {
                self.is_updating = false;
                ctx.emit(BuildPlanMigrationModalEvent::Close);
            }
            BuildPlanMigrationModalViewAction::Close => {
                ctx.emit(BuildPlanMigrationModalEvent::Close);
            }
            BuildPlanMigrationModalViewAction::OpenUrl(_url) => {
                // Simplified: no URL opening
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum BuildPlanMigrationModalEvent {
    Close,
    ShowToast {
        message: String,
        flavor: crate::view_components::ToastFlavor,
    },
}

impl SingletonEntity for BuildPlanMigrationModal {}

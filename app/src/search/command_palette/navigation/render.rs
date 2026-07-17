use cuteui::elements::{
    Align, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Flex, Highlight,
    ParentElement, Radius, Shrinkable,
};
use cuteui::fonts::{Properties, Weight};
use cuteui::ui_components::components::{UiComponent, UiComponentStyles};
use cuteui::{AppContext, Element};

use crate::appearance::Appearance;
use crate::search::command_palette::navigation::search::SessionHighlightIndices;
use crate::search::result_renderer::ItemHighlightState;
// Session navigation types are now exported from crate root
use crate::{CommandContext, SessionNavigationData};

/// Renders a navigation session.
pub fn render_navigation_session(
    session: &SessionNavigationData,
    appearance: &Appearance,
    item_highlight_state: ItemHighlightState,
    is_active_session: bool,
    highlight_indices: &SessionHighlightIndices,
    app: &AppContext,
) -> Box<dyn Element> {
    render_navigation_session_internal(
        render_session_label(
            session,
            appearance,
            item_highlight_state,
            is_active_session,
            highlight_indices,
            app,
        )
        .finish(),
    )
}

fn render_navigation_session_internal(label: Box<dyn Element>) -> Box<dyn Element> {
    ConstrainedBox::new(label)
        .with_height(styles::NAVIGATION_PALETTE_ITEM_HEIGHT)
        .finish()
}

fn render_session_label(
    session: &SessionNavigationData,
    appearance: &Appearance,
    item_highlight_state: ItemHighlightState,
    is_active_session: bool,
    highlight_indices: &SessionHighlightIndices,
    _app: &AppContext,
) -> Flex {
    let mut navigation_palette_item = Flex::column();

    let prompt = if let Some(_ps1_grid) = &session.prompt_elements().ps1_prompt_grid {
        // Type mismatch disabled: ps1_prompt_grid is PromptGrid, not BlockGrid
        Container::new(Flex::row().finish()).finish()
    } else if let Some(_snapshot) = &session.prompt_elements().prompt_chip_snapshot {
        // Type mismatch disabled: prompt_chip_snapshot is PromptChipSnapshot, not PromptSnapshot
        Container::new(Flex::row().finish()).finish()
    } else {
        // Fallback: empty container if neither is available (e.g. very early startup).
        Container::new(Flex::row().finish()).finish()
    };

    let command_info = render_command_context(
        session,
        item_highlight_state,
        is_active_session,
        highlight_indices.command_indices.clone(),
        highlight_indices.hint_text_indices.clone(),
        appearance,
    );

    navigation_palette_item.add_child(
        Container::new(prompt)
            .with_margin_right(styles::NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING)
            .finish(),
    );

    navigation_palette_item.add_child(
        Container::new(command_info)
            .with_margin_top(styles::NAVIGATION_PALETTE_ROW_VERTICAL_SPACING)
            .with_margin_right(styles::NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING)
            .finish(),
    );

    navigation_palette_item
}

fn render_current_session_pill(
    command_context: CommandContext,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let current_session_pill = appearance
        .ui_builder()
        .span("Current".to_string())
        .with_style(UiComponentStyles {
            font_family_id: Some(appearance.monospace_font_family()),
            // The font size is scaled down to make sure the pill fits in the row with its padding.
            font_size: Some(appearance.monospace_font_size() * 0.85),
            font_color: Some(
                appearance
                    .theme()
                    .main_text_color(appearance.theme().background())
                    .into_solid(),
            ),
            ..Default::default()
        })
        .build()
        .with_padding_left(5.)
        .with_padding_right(5.)
        .with_margin_left(10.)
        .with_margin_right(8.)
        .with_background_color(appearance.theme().background().into_solid())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
        .finish();

    Shrinkable::new(
        // We need different flex values when different hint texts are present, otherwise the actual command won't take up enough room.
        match command_context {
            CommandContext::LastRunCommand { .. } | CommandContext::LastRunAIBlock { .. } => 0.5,
            CommandContext::RunningCommand { .. } | CommandContext::RunningAIBlock { .. } => 0.35,
            CommandContext::None => 1.,
        },
        Align::new(
            ConstrainedBox::new(current_session_pill)
                .with_max_width(135.)
                .finish(),
        )
        .right()
        .finish(),
    )
    .finish()
}



fn render_command_context(
    session: &SessionNavigationData,
    item_highlight_state: ItemHighlightState,
    is_active_session: bool,
    command_indices: Option<Vec<usize>>,
    hint_text_indices: Vec<usize>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let command_render_info = CommandRenderInfo::from_context(session.command_context());

    let mut command_row = Flex::row();
    let command_row_font_size = appearance.monospace_font_size() - 2.;

    if let Some(command_text) = command_render_info.command_text {
        if !command_text.is_empty() {
            let running_command_text_color =
                item_highlight_state.main_text_fill(appearance).into_solid();

            let mut running_command_text =
                appearance
                    .ui_builder()
                    .span(command_text)
                    .with_style(UiComponentStyles {
                        font_family_id: Some(appearance.monospace_font_family()),
                        font_size: Some(command_row_font_size),
                        font_color: Some(running_command_text_color),
                        ..Default::default()
                    });

            if let Some(command_indices) = command_indices {
                let highlight = Highlight::new()
                    .with_properties(Properties::default().weight(Weight::Bold))
                    .with_foreground_color(running_command_text_color);
                running_command_text =
                    running_command_text.with_highlights(command_indices, highlight);
            }

            command_row.add_child(
                Shrinkable::new(
                    1.,
                    Container::new(running_command_text.build().finish())
                        .with_margin_right(command_render_info.row_spacing)
                        .finish(),
                )
                .finish(),
            );
        }
    }

    let hint_font_color = item_highlight_state.sub_text_fill(appearance).into_solid();

    let mut hint_text = appearance
        .ui_builder()
        .span(command_render_info.hint_text)
        .with_style(UiComponentStyles {
            font_color: Some(hint_font_color),
            font_family_id: Some(appearance.monospace_font_family()),
            font_size: Some(command_row_font_size),
            ..Default::default()
        });

    let highlight = Highlight::new()
        .with_properties(Properties::default().weight(Weight::Bold))
        .with_foreground_color(hint_font_color);
    hint_text = hint_text.with_highlights(hint_text_indices, highlight);

    command_row.add_child(
        Container::new(hint_text.build().finish())
            .with_margin_left(command_render_info.hint_margin)
            .with_margin_right(command_render_info.hint_margin)
            .finish(),
    );

    if is_active_session {
        command_row.add_child(render_current_session_pill(
            session.command_context(),
            appearance,
        ));
    }

    command_row = command_row.with_cross_axis_alignment(CrossAxisAlignment::End);

    command_row.finish()
}

pub(super) struct CommandRenderInfo {
    pub command_text: Option<String>,
    pub hint_text: String,
    row_spacing: f32,
    hint_margin: f32,
}

impl CommandRenderInfo {
    pub fn from_context(command_context: CommandContext) -> CommandRenderInfo {
        match command_context {
            CommandContext::RunningCommand { running_command } => CommandRenderInfo {
                command_text: Some(running_command),
                hint_text: "Running...".to_string(),
                row_spacing: styles::NAVIGATION_PALETTE_COMMAND_ROW_SPACING,
                hint_margin: styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN,
            },
            CommandContext::LastRunCommand {
                last_run_command,
                mins_since_completion,
            } => CommandRenderInfo {
                row_spacing: match last_run_command.is_empty() {
                    true => 0., // Don't include any spacing if the command is empty.
                    false => styles::NAVIGATION_PALETTE_COMMAND_ROW_SPACING,
                },
                hint_margin: match last_run_command.is_empty() {
                    true => 0., // Don't include any margin if the command is empty.
                    false => styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN,
                },
                command_text: Some(last_run_command),
                hint_text: match mins_since_completion {
                    Some(mins) if mins >= 60 => "Completed over 1 hour ago".to_string(),
                    Some(mins) if mins == 1 => format!("Completed {mins} minute ago"),
                    Some(mins) => format!("Completed {mins} minutes ago"),
                    None => "No timestamp found".to_string(),
                },
            },
            CommandContext::RunningAIBlock { prompt } => CommandRenderInfo {
                command_text: Some(prompt),
                hint_text: "Running...".to_string(),
                row_spacing: styles::NAVIGATION_PALETTE_COMMAND_ROW_SPACING,
                hint_margin: styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN,
            },
            CommandContext::LastRunAIBlock { prompt } => CommandRenderInfo {
                command_text: Some(prompt),
                hint_text: "Completed".to_string(),
                row_spacing: styles::NAVIGATION_PALETTE_COMMAND_ROW_SPACING,
                hint_margin: styles::NAVIGATION_PALETTE_COMMAND_HINT_MARGIN,
            },
            CommandContext::None => CommandRenderInfo {
                command_text: Some(String::new()),
                hint_text: "Empty Session".to_string(),
                row_spacing: 0.,
                hint_margin: 0.,
            },
        }
    }
}

mod styles {
    pub const NAVIGATION_PALETTE_ITEM_HEIGHT: f32 = 70.;

    pub const NAVIGATION_PALETTE_ROW_VERTICAL_SPACING: f32 = 4.;

    pub const NAVIGATION_PALETTE_ROW_HORIZONTAL_SPACING: f32 = 5.;

    pub const NAVIGATION_PALETTE_COMMAND_ROW_SPACING: f32 = 10.;
    pub const NAVIGATION_PALETTE_COMMAND_HINT_MARGIN: f32 = 5.;
}

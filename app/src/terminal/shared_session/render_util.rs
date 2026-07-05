use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use session_sharing_protocol::common::{ParticipantInfo, SessionId};
use session_sharing_protocol::sharer::{
    AddGuestsResponse, FailedToInitializeSessionReason, SessionEndedReason,
};
use cuteui::elements::{
    ChildAnchor, CornerRadius, Fill, Hoverable, MouseStateHandle, OffsetPositioning, ParentAnchor,
    ParentElement, ParentOffsetBounds, Radius, Stack,
};
use cuteui::fonts::Weight;
use cuteui::ui_components::components::{UiComponent, UiComponentStyles};
use cuteui::{AppContext, Element, SingletonEntity};
use std::sync::OnceLock;

use crate::appearance::Appearance;
use crate::ui_components::avatar::{Avatar, AvatarContent};

/// Color for muted/disconnected participants
pub static MUTED_PARTICIPANT_COLOR: OnceLock<ColorU> = OnceLock::new();
pub fn get_muted_participant_color() -> ColorU {
    *MUTED_PARTICIPANT_COLOR.get_or_init(|| ColorU::new(150, 150, 150, 255))
}

/// Border color for muted participant avatars
pub static MUTED_AVATAR_BORDER_COLOR: OnceLock<ColorU> = OnceLock::new();
pub fn get_muted_avatar_border_color() -> ColorU {
    *MUTED_AVATAR_BORDER_COLOR.get_or_init(|| ColorU::new(120, 120, 120, 255))
}

/// Participant with color for rendering
pub struct Participant {
    pub info: ParticipantInfo,
    pub color: ColorU,
}

/// Participant at a selected block with rendering information
pub struct ParticipantAtBlock {
    pub participant: Participant,
    pub should_show_avatar: bool,
    pub is_top_of_continuous_selection: bool,
    pub is_bottom_of_continuous_selection: bool,
}

pub fn shared_session_indicator_color(appearance: &Appearance) -> ColorU {
    appearance.theme().terminal_colors().normal.red.into()
}

/// Diameter including the border
pub const SHARED_SESSION_AVATAR_DIAMETER: f32 = 20.;
pub const SHARED_SESSION_AVATAR_EXECUTOR_DIAMETER: f32 = 16.;

const SHARED_SESSION_DIAMETER_BORDER_WIDTH: f32 = 1.;

/// Shared helper function for rendering avatar in pane header and selected blocks.
/// Actions on hover and click are handled separately.
pub fn non_hoverable_participant_avatar(
    display_name: String,
    image_url: Option<String>,
    participant_color: ColorU,
    is_muted: bool,
    is_executor: bool,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let background = if is_muted {
        get_muted_participant_color()
    } else {
        participant_color
    };
    let border_color = if is_muted {
        get_muted_avatar_border_color().into()
    } else if image_url.is_none() {
        appearance.theme().surface_2()
    } else {
        participant_color.into()
    };
    let diameter = if is_executor {
        SHARED_SESSION_AVATAR_EXECUTOR_DIAMETER
    } else {
        SHARED_SESSION_AVATAR_DIAMETER
    };
    let font = if is_executor { 10. } else { 12. };
    Avatar::new(
        image_url
            .map(|url| AvatarContent::Image {
                url,
                display_name: display_name.clone(),
            })
            .unwrap_or(AvatarContent::DisplayName(display_name)),
        UiComponentStyles {
            width: Some(diameter - 2. * SHARED_SESSION_DIAMETER_BORDER_WIDTH),
            height: Some(diameter - 2. * SHARED_SESSION_DIAMETER_BORDER_WIDTH),
            border_radius: Some(CornerRadius::with_all(Radius::Percentage(50.))),
            border_width: Some(SHARED_SESSION_DIAMETER_BORDER_WIDTH),
            border_color: Some(border_color.into()),
            background: Some(background.into()),
            font_color: Some(ColorU::black()),
            font_family_id: Some(appearance.ui_font_family()),
            font_weight: Some(Weight::Bold),
            font_size: Some(font),
            ..Default::default()
        },
    )
    .build()
    .finish()
}

/// Struct containing just fields from the [`Participant`] needed for rendering the avatar,
/// to avoid unnecessary cloning of the other fields in the participant.
#[derive(Clone)]
pub struct ParticipantAvatarParams {
    pub display_name: String,
    pub image_url: Option<String>,
    pub participant_color: ColorU,
    pub is_muted: bool,
    pub tooltip_parent_anchor: ParentAnchor,
    pub tooltip_child_anchor: ChildAnchor,
}

impl ParticipantAvatarParams {
    pub fn new(participant: &Participant, is_self_reconnecting: bool) -> Self {
        Self {
            display_name: participant.info.profile_data.display_name.clone(),
            image_url: participant.info.profile_data.photo_url.clone(),
            participant_color: participant.color.to_owned(),
            is_muted: is_self_reconnecting,
            tooltip_parent_anchor: ParentAnchor::TopRight,
            tooltip_child_anchor: ChildAnchor::BottomRight,
        }
    }
}

/// Helper function to render participant avatar and handle hover in selected blocks.
pub fn participant_avatar_for_selected_block(
    params: ParticipantAvatarParams,
    mouse_state_handle: MouseStateHandle,
    app: &AppContext,
) -> Box<dyn Element> {
    let appearance = Appearance::as_ref(app);
    let avatar = non_hoverable_participant_avatar(
        params.display_name.clone(),
        params.image_url,
        params.participant_color,
        params.is_muted,
        false,
        app,
    );

    Hoverable::new(mouse_state_handle, |state| {
        let mut stack = Stack::new().with_child(avatar);
        if state.is_hovered() {
            let tooltip_background = appearance.theme().tooltip_background();
            let tool_tip = appearance
                .ui_builder()
                .tool_tip(params.display_name)
                .with_style(UiComponentStyles {
                    font_size: Some(12.),
                    background: Some(Fill::Solid(tooltip_background)),
                    font_color: Some(appearance.theme().background().into_solid()),
                    ..Default::default()
                });
            stack.add_positioned_overlay_child(
                tool_tip.build().finish(),
                OffsetPositioning::offset_from_parent(
                    vec2f(0., 0.),
                    ParentOffsetBounds::WindowByPosition,
                    params.tooltip_parent_anchor,
                    params.tooltip_child_anchor,
                ),
            );
        }
        stack.finish()
    })
    .finish()
}

/// Returns the text selection color for a participant
pub fn text_selection_color(participant_color: ColorU) -> ColorU {
    participant_color
}

/// Returns a user-friendly string for session termination reason
pub fn session_terminated_reason_string(
    _reason: SessionEndedReason,
    _max_session_size: usize,
) -> String {
    "Session ended".to_string()
}

/// Returns the join link for a session
pub fn join_link(session_id: &SessionId) -> String {
    format!("https://app.cute.dev/join/{}", session_id)
}

/// Returns a user-friendly error string for failed guest addition
pub fn failed_to_add_guests_user_error(reason: &AddGuestsResponse) -> String {
    match reason {
        AddGuestsResponse::Error(_) => "Failed to add guests to session".to_string(),
        _ => "Unknown error".to_string(),
    }
}

/// Returns a user-friendly error string for failed session initialization
pub fn failed_to_initialize_session_user_error(reason: &FailedToInitializeSessionReason) -> String {
    match reason {
        FailedToInitializeSessionReason::NoUserQuotaRemaining { .. } => {
            "Session quota exceeded".to_string()
        }
        _ => "Failed to initialize session".to_string(),
    }
}

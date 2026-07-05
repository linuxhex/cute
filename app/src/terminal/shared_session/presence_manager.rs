//! Presence manager for shared sessions.

use session_sharing_protocol::common::{ParticipantId, ParticipantList, Role};
use cuteui::{Entity, ModelContext};
use crate::editor::{CursorColors, PeerSelectionData};
use crate::themes::theme::Fill;
use crate::terminal::BlockList;
use crate::terminal::shared_session::render_util::ParticipantAtBlock;

/// Manages presence information for participants in a shared session.
#[derive(Debug, Clone, Default)]
pub struct PresenceManager {
    viewer_id: Option<ParticipantId>,
    sharer_id: Option<ParticipantId>,
    firebase_uid: Option<String>,
    is_reconnecting: bool,
}

impl PresenceManager {
    pub fn new_for_viewer(
        viewer_id: ParticipantId,
        firebase_uid: crate::auth::UserUid,
        _participant_list: ParticipantList,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            viewer_id: Some(viewer_id),
            sharer_id: None,
            firebase_uid: Some(firebase_uid.as_string()),
            is_reconnecting: false,
        }
    }

    pub fn new_for_sharer(
        sharer_id: ParticipantId,
        firebase_uid: crate::auth::UserUid,
    ) -> Self {
        Self {
            viewer_id: Some(sharer_id.clone()),
            sharer_id: Some(sharer_id),
            firebase_uid: Some(firebase_uid.as_string()),
            is_reconnecting: false,
        }
    }

    pub fn id(&self) -> ParticipantId {
        self.viewer_id.clone().unwrap_or_default()
    }

    pub fn sharer_id(&self) -> ParticipantId {
        self.sharer_id.clone().unwrap_or_default()
    }

    pub fn firebase_uid(&self) -> &str {
        self.firebase_uid.as_deref().unwrap_or("")
    }

    pub fn role(&self) -> Option<Role> {
        None
    }

    pub fn is_reconnecting(&self) -> bool {
        self.is_reconnecting
    }

    pub fn set_is_reconnecting(&mut self, is_reconnecting: bool, _ctx: &mut ModelContext<Self>) {
        self.is_reconnecting = is_reconnecting;
    }

    pub fn get_present_viewers(&self) -> impl Iterator<Item = &session_sharing_protocol::common::ParticipantInfo> {
        std::iter::empty()
    }

    pub fn get_sharer(&self) -> Option<&session_sharing_protocol::common::ParticipantInfo> {
        None
    }

    pub fn absent_viewers(&self) -> impl Iterator<Item = &session_sharing_protocol::common::ParticipantInfo> {
        std::iter::empty()
    }

    pub fn viewer_firebase_uid(&self, _participant_id: &ParticipantId) -> Option<&str> {
        None
    }

    pub fn present_viewer_ids_for_uid(&self, _uid: &str) -> impl Iterator<Item = ParticipantId> + '_ {
        std::iter::empty()
    }

    pub fn viewer_role(&self, _participant_id: &ParticipantId) -> Option<Role> {
        None
    }

    pub fn update_participant_role(
        &mut self,
        _participant_id: &ParticipantId,
        _role: Role,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn update_participants(
        &mut self,
        _participant_list: ParticipantList,
        _ctx: &mut ModelContext<Self>,
    ) {
        // TODO: Implement participant list update logic
    }

    pub fn update_participant_presence(
        &mut self,
        _update: session_sharing_protocol::common::ParticipantPresenceUpdate,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn get_participant(&self, _participant_id: &ParticipantId) -> Option<&session_sharing_protocol::common::ParticipantInfo> {
        None
    }

    pub fn input_data_for_participant(
        &self,
        _participant: &session_sharing_protocol::common::ParticipantInfo,
    ) -> (String, PeerSelectionData) {
        (
            String::new(),
            PeerSelectionData {
                colors: CursorColors {
                    cursor: Fill::black(),
                    selection: Fill::black(),
                },
                display_name: String::new(),
                image_url: None,
                should_draw_cursors: false,
            },
        )
    }

    pub fn all_present_participants(&self) -> impl Iterator<Item = &session_sharing_protocol::common::ParticipantInfo> {
        std::iter::empty()
    }

    pub fn single_distinct_present_viewer_uid(&self) -> Option<&str> {
        None
    }

    pub fn make_all_participants_readers(&mut self, _ctx: &mut ModelContext<Self>) {
    }

    pub fn on_role_requested(
        &mut self,
        _participant_id: ParticipantId,
        _role_request_id: session_sharing_protocol::common::RoleRequestId,
        _role: Role,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn on_role_request_cancelled(
        &mut self,
        _participant_id: ParticipantId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn on_role_request_responded_to(
        &mut self,
        _participant_id: ParticipantId,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn present_viewer_id_for_uid(&self, _uid: crate::auth::UserUid) -> Option<&ParticipantId> {
        None
    }

    /// Static helper to get single distinct viewer UID from viewers list.
    pub fn single_distinct_present_viewer_uid_from_viewers(
        _viewers: &[session_sharing_protocol::common::ParticipantInfo],
    ) -> Option<&str> {
        None
    }

    /// Get participants who have selected the given block.
    pub fn get_participants_at_selected_block(
        &self,
        _block_index: usize,
        _block_list: &BlockList,
    ) -> Vec<ParticipantAtBlock> {
        // Stub implementation - returns empty list
        Vec::new()
    }
}

#[derive(Debug, Clone)]
pub enum PresenceManagerEvent {
    ParticipantListUpdated,
}

impl Entity for PresenceManager {
    type Event = PresenceManagerEvent;
}

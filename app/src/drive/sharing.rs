pub mod dialog;

use crate::server::ids::ServerId;
use cute_graphql::object_permissions::AccessLevel;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SharingAccessLevel {
    Owner,
    Full,
    Edit,
    View,
}

impl SharingAccessLevel {
    pub fn can_trash(&self) -> bool {
        matches!(self, SharingAccessLevel::Owner | SharingAccessLevel::Full)
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, SharingAccessLevel::Owner | SharingAccessLevel::Full | SharingAccessLevel::Edit)
    }
}

impl From<AccessLevel> for SharingAccessLevel {
    fn from(server_access: AccessLevel) -> Self {
        match server_access {
            AccessLevel::Viewer => Self::View,
            AccessLevel::Editor => Self::Edit,
            AccessLevel::Full => Self::Full,
        }
    }
}

impl From<SharingAccessLevel> for AccessLevel {
    fn from(val: SharingAccessLevel) -> Self {
        match val {
            SharingAccessLevel::Owner | SharingAccessLevel::Full => AccessLevel::Full,
            SharingAccessLevel::Edit => AccessLevel::Editor,
            SharingAccessLevel::View => AccessLevel::Viewer,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentEditability {
    Editable,
    ReadOnly,
    RequiresLogin,
}

impl ContentEditability {
    pub fn can_edit(&self) -> bool {
        matches!(self, ContentEditability::Editable)
    }
}

pub trait IsShareable {
    fn server_id(&self) -> ServerId;
}

/// Enum representing shareable objects in the UI
#[derive(Debug, Clone)]
pub enum ShareableObject {
    /// A terminal session
    Session {
        handle: cuteui::ViewHandle<crate::terminal::view::TerminalView>,
        session_id: session_sharing_protocol::common::SessionId,
        started_at: chrono::DateTime<chrono::Local>,
    },
    /// A Warp Drive object
    WarpDriveObject(ServerId),
    /// An AI conversation
    AIConversation(crate::ai::agent::conversation::AIConversationId),
}

impl ShareableObject {
    /// Returns a shareable link for this object, if available.
    pub fn link(&self, _ctx: &cuteui::AppContext) -> Option<String> {
        match self {
            ShareableObject::Session { session_id, .. } => {
                Some(session_id.to_string())
            }
            ShareableObject::WarpDriveObject(_server_id) => {
                // TODO: Implement link generation for Warp Drive objects
                None
            }
            ShareableObject::AIConversation(conversation_id) => {
                Some(conversation_id.to_string())
            }
        }
    }
}

use byte_unit::Byte;
use instant::Duration;
use serde::{Deserialize, Serialize};
use session_sharing_protocol::common::Role;
use cuteui::keymap::ContextPredicate;
use cuteui::{id, AppContext};

use super::model::terminal_model::BlockIndex;
use super::TerminalModel;

pub mod history_model;
pub mod manager;
pub mod participant_avatar_view;
pub mod presence_manager;
pub mod render_util;
pub mod role_change_modal;
pub mod remote_update_guard;
pub mod session_permissions_manager;
pub mod settings;
pub mod share_session_modal;

// Re-export for convenience
pub use history_model::SharedSessionHistoryModel;
pub use manager::{Manager, ManagerEvent};
pub use participant_avatar_view::{ParticipantAvatarView, ParticipantAvatarEvent};
pub use presence_manager::{PresenceManager, PresenceManagerEvent};
pub use role_change_modal::{RoleChangeModal, RoleChangeModalEvent, RoleChangeCloseSource, RoleChangeOpenSource};
pub use remote_update_guard::RemoteUpdateGuard;
pub use session_permissions_manager::{SessionPermissionsManager, SessionPermissionsManagerEvent};
pub use share_session_modal::{ShareSessionModal, ShareSessionModalEvent};

#[cfg(test)]
pub use tests::MAX_BYTES_SHAREABLE;

const SELECTION_THROTTLE_PERIOD: Duration = Duration::from_millis(20);

#[derive(Debug, Clone, Default)]
pub enum IsSharedSessionCreator {
    Yes {
        source: SharedSessionSource,
    },
    #[default]
    No,
}

#[derive(Debug, Clone)]
pub enum SharedSessionStatus {
    NotShared,
    SharePendingPreBootstrap {
        source: SharedSessionSource,
    },
    SharePending,
    ActiveSharer,
    ActiveViewer {
        role: Role,
    },
    FinishedViewer,
}

impl SharedSessionStatus {
    pub fn reader() -> Self {
        Self::NotShared
    }

    pub fn executor() -> Self {
        Self::NotShared
    }

    pub fn is_view_pending(&self) -> bool {
        matches!(self, Self::SharePendingPreBootstrap { .. })
    }

    pub fn is_active_viewer(&self) -> bool {
        matches!(self, Self::ActiveViewer { .. })
    }

    pub fn is_finished_viewer(&self) -> bool {
        matches!(self, Self::FinishedViewer)
    }

    pub fn is_viewer(&self) -> bool {
        matches!(self, Self::ActiveViewer { .. } | Self::FinishedViewer)
    }

    pub fn is_executor(&self) -> bool {
        matches!(self, Self::ActiveViewer { role: Role::Executor | Role::Full })
    }

    pub fn is_reader(&self) -> bool {
        matches!(self, Self::ActiveViewer { role: Role::Reader })
    }

    pub fn is_share_pending(&self) -> bool {
        matches!(self, Self::SharePendingPreBootstrap { .. } | Self::SharePending)
    }

    pub fn is_active_sharer(&self) -> bool {
        matches!(self, Self::ActiveSharer)
    }

    pub fn is_sharer(&self) -> bool {
        matches!(self, Self::ActiveSharer | Self::SharePendingPreBootstrap { .. })
    }

    pub fn is_sharer_or_viewer(&self) -> bool {
        self.is_sharer() || self.is_viewer()
    }

    pub fn as_keymap_context(&self) -> &'static str {
        match self {
            Self::NotShared => "SharedSessionStatus_NotShared",
            Self::SharePendingPreBootstrap { .. } | Self::SharePending => "SharedSessionStatus_SharePending",
            Self::ActiveSharer => "SharedSessionStatus_ActiveSharer",
            Self::ActiveViewer { role: Role::Reader } => "SharedSessionStatus_ActiveViewer_Reader",
            Self::ActiveViewer { role: Role::Executor | Role::Full } => "SharedSessionStatus_ActiveViewer_Executor",
            Self::FinishedViewer => "SharedSessionStatus_FinishedViewer",
        }
    }

    pub fn active_viewer_keymap_context() -> ContextPredicate {
        id!(Self::reader().as_keymap_context()) | id!(Self::executor().as_keymap_context())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedSessionScrollbackType {
    None,
    FromBlock {
        block_index: BlockIndex,
    },
    All,
}

impl SharedSessionScrollbackType {
    pub fn first_block_index(self, model: &TerminalModel) -> BlockIndex {
        match self {
            Self::None => model.block_list().active_block_index(),
            Self::FromBlock { block_index } => model
                .block_list()
                .blocks()
                .iter()
                .skip(block_index.into())
                .find(|block| {
                    block.is_scrollback_block_for_shared_session(
                        model.block_list().agent_view_state(),
                    )
                })
                .map_or(model.block_list().active_block_index(), |block| {
                    block.index()
                }),
            Self::All => Self::FromBlock {
                block_index: BlockIndex::zero(),
            }
            .first_block_index(model),
        }
    }
}

#[cfg(not(test))]
pub fn max_session_size(_ctx: &AppContext) -> Byte {
    Byte::from_u64_with_unit(100, byte_unit::Unit::MB).unwrap()
}

#[cfg(test)]
pub fn max_session_size(_ctx: &AppContext) -> Byte {
    Byte::from_u64(MAX_BYTES_SHAREABLE as u64)
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum SharedSessionActionSource {
    BlocklistContextMenu {
        block_index: Option<BlockIndex>,
    },
    Tab,
    PaneHeader,
    CommandPalette,
    OnboardingBlock,
    Closed {
        is_confirm_close_session: bool,
    },
    InactivityModal,
    NonUser,
    SharingDialog,
    RightClickMenu,
    FooterChip,
}

#[derive(Debug, Clone)]
pub struct SharedSessionSource {
    pub source_type: SessionSourceType,
    pub source_task_id: Option<String>,
}

impl SharedSessionSource {
    pub fn user(source_task_id: Option<String>) -> Self {
        Self {
            source_type: SessionSourceType::User,
            source_task_id,
        }
    }

    pub fn ambient_agent(task_id: Option<String>) -> Self {
        Self {
            source_type: SessionSourceType::AmbientAgent {
                task_id: task_id.clone(),
            },
            source_task_id: task_id,
        }
    }

    pub fn orchestrator_task_id(&self) -> Option<&str> {
        self.source_task_id.as_deref().or(match &self.source_type {
            SessionSourceType::AmbientAgent { task_id } => task_id.as_deref(),
            SessionSourceType::User => None,
        })
    }
}

impl Default for SharedSessionSource {
    fn default() -> Self {
        Self::user(None)
    }
}

// Re-export SessionSourceType from session_sharing_protocol
pub use session_sharing_protocol::sharer::SessionSourceType;

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;

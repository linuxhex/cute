// 已注释：清理 shared_session 共享会话功能
// 原始文件已简化，保留基本结构以避免编译错误

use byte_unit::Byte;
use instant::Duration;
use serde::{Deserialize, Serialize};
// use session_sharing_protocol::common::Role;
use cuteui::keymap::ContextPredicate;
use cuteui::{id, AppContext};

use super::model::terminal_model::BlockIndex;
use super::TerminalModel;

pub mod settings;

// 简化的枚举类型
#[derive(Debug, Clone, Default)]
pub enum IsSharedSessionCreator {
    Yes {
        source: String,  // 简化类型
    },
    #[default]
    No,
}

// 简化的 SharedSessionStatus
#[derive(Debug, Clone)]
pub enum SharedSessionStatus {
    NotShared,
    SharePendingPreBootstrap {
        source: String,  // 简化类型
    },
    SharePending,
    ActiveSharer,
    ActiveViewer {
        role: String,  // 简化类型
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
        false  // 默认返回 false
    }

    pub fn is_active_viewer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_finished_viewer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_viewer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_executor(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_reader(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_share_pending(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_active_sharer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_sharer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn is_sharer_or_viewer(&self) -> bool {
        false  // 默认返回 false
    }

    pub fn as_keymap_context(&self) -> &'static str {
        "SharedSessionStatus_NotShared"  // 默认值
    }

    pub fn active_viewer_keymap_context() -> ContextPredicate {
        id!("SharedSessionStatus_NotShared")
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
        model.block_list().active_block_index()  // 简化实现
    }
}

#[cfg(not(test))]
pub fn max_session_size(_ctx: &AppContext) -> Byte {
    Byte::from_u64_with_unit(100, byte_unit::Unit::MB).unwrap()
}

// 简化的 SharedSessionActionSource
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

// 简化的 SharedSessionSource
#[derive(Debug, Clone)]
pub struct SharedSessionSource {
    pub source_type: String,  // 简化类型
    pub source_task_id: Option<String>,
}

impl SharedSessionSource {
    pub fn user(source_task_id: Option<String>) -> Self {
        Self {
            source_type: "User".to_string(),
            source_task_id,
        }
    }

    pub fn ambient_agent(task_id: Option<String>) -> Self {
        Self {
            source_type: "AmbientAgent".to_string(),
            source_task_id: task_id,
        }
    }

    pub fn orchestrator_task_id(&self) -> Option<&str> {
        self.source_task_id.as_deref()
    }
}

impl Default for SharedSessionSource {
    fn default() -> Self {
        Self::user(None)
    }
}

// 简化的 SessionSourceType
#[derive(Debug, Clone)]
pub enum SessionSourceType {
    User,
    AmbientAgent {
        task_id: Option<String>,
    },
}

pub mod common;
pub mod sharer;
pub mod viewer;

// Re-export commonly used types
pub use common::{BlockId, BufferId, InputMode, InputType, ParticipantId, ParticipantInfo, ProfileData, Role, SessionId};
pub use sharer::TeamAcl;

/// Re-export color types from pathfinder_color
pub mod color {
    pub use pathfinder_color::ColorU;
}

/// Stub module for cute_terminal types to avoid circular dependency.
/// The real types are in the cute_terminal crate.
pub mod cute_terminal {
    pub mod model {
        /// Stub type for BlockList. The real type is in cute_terminal::model::BlockList.
        /// This is used only for type signatures in this crate.
        pub struct BlockList {
            // Empty stub - the real implementation is in cute_terminal crate
        }
    }
}

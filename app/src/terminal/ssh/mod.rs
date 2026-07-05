use std::time::Duration;

pub mod error;
pub mod install_tmux;
pub mod root_access;
pub mod ssh_detection;
pub mod util;
pub mod cuteify;

/// Backward compatibility re-export module.
/// The warpify module has been renamed to cuteify.
pub mod warpify {
    pub use super::cuteify::*;
}

pub const SSH_WARPIFY_TIMEOUT_DURATION: Duration = Duration::from_secs(8);

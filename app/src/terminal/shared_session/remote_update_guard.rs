//! Guard for remote updates in shared sessions.

use std::sync::atomic::{AtomicBool, Ordering};

/// Guard to prevent remote updates during certain operations.
#[derive(Debug)]
pub struct RemoteUpdateGuard {
    active: AtomicBool,
}

impl RemoteUpdateGuard {
    pub fn new() -> Self {
        Self {
            active: AtomicBool::new(false),
        }
    }

    pub fn activate(&self) {
        self.active.store(true, Ordering::SeqCst);
    }

    pub fn deactivate(&self) {
        self.active.store(false, Ordering::SeqCst);
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    /// Start a remote update session. Returns a reference to the guard.
    pub fn start_remote_update(&self) -> &Self {
        self.activate();
        self
    }

    /// Check if broadcasting is allowed (not currently in a remote update session).
    pub fn should_broadcast(&self) -> bool {
        !self.is_active()
    }
}

impl Default for RemoteUpdateGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RemoteUpdateGuard {
    fn clone(&self) -> Self {
        Self {
            active: AtomicBool::new(self.active.load(Ordering::SeqCst)),
        }
    }
}

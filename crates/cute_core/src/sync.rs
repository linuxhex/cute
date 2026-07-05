/// Stub module for sync-related types.
/// This module provides minimal stubs for sync functionality.

/// A unique identifier for a sync operation or entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SyncId(pub u64);

impl SyncId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
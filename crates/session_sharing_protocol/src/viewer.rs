use serde::{Deserialize, Serialize};

/// Reason for a role being updated for a viewer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleUpdatedReason {
    SharerChangedRole,
    LinkSharingChangedRole,
    TeamAclChangedRole,
    UserDirectAclChangedRole,
    InactivityLimitReached,
}
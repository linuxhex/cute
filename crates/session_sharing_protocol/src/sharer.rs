use serde::{Deserialize, Serialize};

use crate::common::Role;

/// Source type for a shared session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SessionSourceType {
    User,
    AmbientAgent {
        task_id: Option<String>,
    },
}

impl SessionSourceType {
    pub fn orchestrator_task_id(&self) -> Option<&str> {
        match self {
            SessionSourceType::AmbientAgent { task_id } => task_id.as_deref(),
            SessionSourceType::User => None,
        }
    }
}

/// Reason for a role update
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleUpdateReason {
    SharerChangedRole,
    LinkSharingChangedRole,
    TeamAclChangedRole,
    UserDirectAclChangedRole,
    InactivityLimitReached,
}

/// Reason for a session ending
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SessionEndedReason {
    SharerEndedSession,
    SharerDisconnected,
    ServerError,
    SessionExpired,
    EndedBySharer,
    InactivityLimitReached,
}

/// Stub type for TeamAcl
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeamAcl {
    // TODO: Add fields
}

/// Add guests response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AddGuestsResponse {
    Ok,
    Error(AddGuestsError),
}

/// Add guests error
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddGuestsError {
    pub message: String,
}

/// Failed to initialize session reason
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FailedToInitializeSessionReason {
    NoUserQuotaRemaining {
        quota_type: QuotaType,
    },
    ServerError {
        message: String,
    },
    NetworkError,
    InvalidSession,
    SessionAlreadyActive,
}

/// Lifetime for a shared session
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Lifetime {
    Ephemeral,
    Lingering,
}

/// Link access level update response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LinkAccessLevelUpdateResponse {
    Ok { role: Role },
    Error,
}

/// Quota type
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuotaType {
    SessionsCreated,
    SessionsJoined,
}

/// Remove guest response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RemoveGuestResponse {
    Ok,
    Error(RemoveGuestError),
}

/// Remove guest error
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoveGuestError {
    pub message: String,
}

/// Team access level update response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TeamAccessLevelUpdateResponse {
    Success { team_acl: TeamAcl },
    Error(String),
}

/// Update pending user role response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UpdatePendingUserRoleResponse {
    Ok,
    Error(String),
}

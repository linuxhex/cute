//! Stub for remote_server::transport after removal.

use serde::{Deserialize, Serialize};

/// Stub for UserFacingError.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserFacingError {
    pub message: String,
    pub stage: SetupStage,
    pub body: String,
    pub detail: String,
}

/// Stub for SetupStage.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SetupStage {
    InstallBinary,
    CheckBinary,
    StartDaemon,
    Connect,
    Unknown,
}

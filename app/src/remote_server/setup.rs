//! Stub for remote_server::setup after removal.
#![allow(dead_code)]

/// Returns the data directory for the remote server daemon.
pub fn remote_server_daemon_data_dir(_identity_key: &str) -> String {
    // Return a default path
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.warp/remote_server", home)
}

/// Stub for remote platform info.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemotePlatform;

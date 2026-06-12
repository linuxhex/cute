//! Stub for remote_server::setup after removal.

use std::path::PathBuf;

/// Returns the data directory for the remote server daemon.
pub fn remote_server_daemon_data_dir(_identity_key: &str) -> String {
    // Return a default path
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.warp/remote_server", home)
}

/// Returns the default socket path for the remote server daemon.
pub fn remote_server_daemon_socket_path(_identity_key: &str) -> PathBuf {
    PathBuf::from("/tmp/warp_remote_server.sock")
}

/// Stub for remote platform info.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemotePlatform {
    pub os: RemoteOs,
    pub arch: RemoteArch,
}

/// Stub for remote OS type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoteOs {
    Linux,
    MacOs,
}

impl RemoteOs {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::MacOs => "macos",
        }
    }
}

/// Stub for remote architecture type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RemoteArch {
    X86_64,
    Aarch64,
}

impl RemoteArch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Aarch64 => "aarch64",
        }
    }
}

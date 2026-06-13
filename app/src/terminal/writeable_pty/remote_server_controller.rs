//! Stub for remote_server_controller after remote_server removal.

/// Returns a connection label from user and host.
pub fn connection_label_from_user_and_host(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}

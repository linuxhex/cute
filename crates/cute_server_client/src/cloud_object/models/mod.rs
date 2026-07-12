// Minimal stub types for cloud object models
// These types provide minimal definitions to allow compilation without actual cloud functionality

/// State of an MCP server.
/// This is a minimal stub type for MCP server functionality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MCPServerState {
    /// The server is not connected.
    Disconnected,
    /// The server is connecting.
    Connecting,
    /// The server is connected and ready.
    Connected,
    /// The server encountered an error.
    Error,
    /// The server is being initialized.
    Initializing,
    /// The server is running.
    Running,
    /// The server failed to start.
    FailedToStart,
    /// The server is not running.
    NotRunning,
    /// The server is starting.
    Starting,
    /// The server is shutting down.
    ShuttingDown,
}

impl Default for MCPServerState {
    fn default() -> Self {
        MCPServerState::Disconnected
    }
}

impl MCPServerState {
    /// Returns true if the server is in a connected state.
    pub fn is_connected(&self) -> bool {
        matches!(self, MCPServerState::Connected)
    }

    /// Returns true if the server is in an error state.
    pub fn is_error(&self) -> bool {
        matches!(self, MCPServerState::Error)
    }

    /// Returns true if the server is running.
    pub fn is_running(&self) -> bool {
        matches!(self, MCPServerState::Running)
    }

    /// Returns true if the server is not running.
    pub fn is_not_running(&self) -> bool {
        matches!(self, MCPServerState::NotRunning)
    }
}

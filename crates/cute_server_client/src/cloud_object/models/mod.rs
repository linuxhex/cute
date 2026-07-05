#[derive(Debug, Clone)]
pub struct SuggestedLoggingId(pub String);

#[derive(Debug, Clone)]
pub struct CloudObjectLookup;

#[derive(Debug, Clone)]
pub struct Space;

#[derive(Debug, Clone)]
pub struct Owner;

#[derive(Debug, Clone)]
pub struct CloudObject;

#[derive(Debug, Clone)]
pub struct Notebook;

#[derive(Debug, Clone)]
pub struct Workflow;

#[derive(Debug, Clone)]
pub struct Folder;

#[derive(Debug, Clone)]
pub struct EnvVarCollection;

#[derive(Debug, Clone)]
pub struct AIFact;

#[derive(Debug, Clone)]
pub struct CloudAgentConfig;

#[derive(Debug, Clone)]
pub struct CloudEnvironment;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CloudObjectId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotebookId(pub String);

#[derive(Debug, Clone)]
pub struct CloudWorkflow;

#[derive(Debug, Clone)]
pub struct CloudEnvVarCollection;

#[derive(Debug, Clone)]
pub struct CloudNotebook;

#[derive(Debug, Clone)]
pub struct CloudFolder;

#[derive(Debug, Clone)]
pub struct CloudObjectLocation;

#[derive(Debug, Clone)]
pub struct CloudObjectTypeAndId;

#[derive(Debug, Clone)]
pub struct CloudObjectMetadata;

#[derive(Debug, Clone)]
pub struct CloudObjectPermissions;

#[derive(Debug, Clone)]
pub struct CloudObjectStatuses;

#[derive(Debug, Clone)]
pub struct CloudObjectSyncStatus;

#[derive(Debug, Clone)]
pub struct CloudObjectGuest;

impl CloudObjectLocation {
    pub fn Folder(_id: CloudObjectId) -> Self { CloudObjectLocation }
}

// MCP server related model stubs

#[derive(Debug, Clone)]
pub struct CLIServer {
    pub command: String,
    pub args: Vec<String>,
    pub cwd_parameter: Option<String>,
    pub static_env_vars: Vec<StaticEnvVar>,
}

#[derive(Debug, Clone)]
pub struct JSONMCPServer;

#[derive(Debug, Clone)]
pub enum JSONTransportType {
    Stdio,
    Sse,
    Http,
}

#[derive(Debug, Clone)]
pub struct ServerSentEvents {
    pub headers: Vec<StaticHeader>,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct StaticEnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct StaticHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct CloudMCPServer;

#[derive(Debug, Clone)]
pub struct CloudMCPServerModel;

#[derive(Debug, Clone)]
pub struct MCPServer;

#[derive(Debug, Clone, Copy)]
pub enum MCPServerState {
    NotRunning,
    Starting,
    Authenticating,
    Running,
    ShuttingDown,
    FailedToStart,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone)]
pub enum TransportType {
    Stdio,
    Sse,
    Http,
    CLIServer(CLIServer),
    ServerSentEvents(ServerSentEvents),
}

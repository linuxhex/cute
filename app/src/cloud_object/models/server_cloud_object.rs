use anyhow::Result;
use cute_server_client::{
    cloud_object::ServerMetadata,
    ids::ObjectUid,
};

use super::{
    ServerAIExecutionProfile, ServerAIFact, ServerAmbientAgentEnvironment, ServerCloudAgentConfig,
    ServerEnvVarCollection, ServerFolder, ServerMCPServer, ServerNotebook, ServerPreference,
    ServerScheduledAmbientAgent, ServerTemplatableMCPServer, ServerWorkflow, ServerWorkflowEnum,
};

#[derive(Clone, Debug)]
pub enum ServerCloudObject {
    Notebook(ServerNotebook),
    Workflow(Box<ServerWorkflow>),
    Folder(ServerFolder),
    Preference(ServerPreference),
    EnvVarCollection(ServerEnvVarCollection),
    WorkflowEnum(ServerWorkflowEnum),
    AIFact(ServerAIFact),
    MCPServer(ServerMCPServer),
    AIExecutionProfile(ServerAIExecutionProfile),
    TemplatableMCPServer(ServerTemplatableMCPServer),
    AmbientAgentEnvironment(ServerAmbientAgentEnvironment),
    ScheduledAmbientAgent(ServerScheduledAmbientAgent),
    CloudAgentConfig(ServerCloudAgentConfig),
}

impl ServerCloudObject {
    pub fn metadata(&self) -> &ServerMetadata {
        match self {
            ServerCloudObject::Notebook(notebook) => &notebook.metadata,
            ServerCloudObject::Workflow(workflow) => &workflow.metadata,
            ServerCloudObject::Folder(folder) => &folder.metadata,
            ServerCloudObject::Preference(preferences) => &preferences.metadata,
            ServerCloudObject::EnvVarCollection(env_var_collection) => &env_var_collection.metadata,
            ServerCloudObject::WorkflowEnum(workflow_enum) => &workflow_enum.metadata,
            ServerCloudObject::AIFact(aifact) => &aifact.metadata,
            ServerCloudObject::MCPServer(mcp_server) => &mcp_server.metadata,
            ServerCloudObject::TemplatableMCPServer(templatable_mcp_server) => {
                &templatable_mcp_server.metadata
            }
            ServerCloudObject::AIExecutionProfile(ai_execution_profile) => {
                &ai_execution_profile.metadata
            }
            ServerCloudObject::AmbientAgentEnvironment(ambient_agent_environment) => {
                &ambient_agent_environment.metadata
            }
            ServerCloudObject::ScheduledAmbientAgent(scheduled_ambient_agent) => {
                &scheduled_ambient_agent.metadata
            }
            ServerCloudObject::CloudAgentConfig(cloud_agent_config) => &cloud_agent_config.metadata,
        }
    }

    pub fn uid(&self) -> ObjectUid {
        match self {
            ServerCloudObject::Notebook(notebook) => notebook.id.uid(),
            ServerCloudObject::Workflow(workflow) => workflow.id.uid(),
            ServerCloudObject::Folder(folder) => folder.id.uid(),
            ServerCloudObject::Preference(preferences) => preferences.id.uid(),
            ServerCloudObject::EnvVarCollection(env_var_collection) => env_var_collection.id.uid(),
            ServerCloudObject::WorkflowEnum(workflow_enum) => workflow_enum.id.uid(),
            ServerCloudObject::AIFact(aifact) => aifact.id.uid(),
            ServerCloudObject::MCPServer(mcp_server) => mcp_server.id.uid(),
            ServerCloudObject::AIExecutionProfile(ai_execution_profile) => {
                ai_execution_profile.id.uid()
            }
            ServerCloudObject::TemplatableMCPServer(templatable_mcp_server) => {
                templatable_mcp_server.id.uid()
            }
            ServerCloudObject::AmbientAgentEnvironment(ambient_agent_environment) => {
                ambient_agent_environment.id.uid()
            }
            ServerCloudObject::ScheduledAmbientAgent(scheduled_ambient_agent) => {
                scheduled_ambient_agent.id.uid()
            }
            ServerCloudObject::CloudAgentConfig(cloud_agent_config) => cloud_agent_config.id.uid(),
        }
    }
}

pub trait TryFromGql: Sized {
    type GqlType;

    fn try_from_gql(_value: Self::GqlType) -> Result<Self> {
        Err(anyhow::anyhow!("cloud sync has been removed"))
    }
}

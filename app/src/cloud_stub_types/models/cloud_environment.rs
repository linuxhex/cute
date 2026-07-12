use std::fmt;

use cute_server_client::{
    cloud_object::{GenericCloudObject, GenericServerObject, GenericStringModel, GenericStringObjectFormat, JsonObjectType},
    ids::GenericStringObjectId,
};
use serde::{Deserialize, Serialize};

use super::{JsonModel, JsonSerializer};
use crate::cloud_stub_types::model::generic_string_model::StringModel;
use crate::cloud_stub_types::GenericStringObjectUniqueKey;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GithubRepo {
    /// Repository owner (e.g. "warpdotdev")
    pub owner: String,
    /// Repository name (e.g. "warp-internal")
    pub repo: String,
}

impl GithubRepo {
    pub fn new(owner: String, repo: String) -> Self {
        Self { owner, repo }
    }
}

impl fmt::Display for GithubRepo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BaseImage {
    DockerImage(String),
}

impl fmt::Display for BaseImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaseImage::DockerImage(s) => s.fmt(f),
        }
    }
}

/// GCP provider configuration for workload identity federation
/// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// pub struct GcpProviderConfig {
///     pub project_number: String,
///     pub workload_identity_federation_pool_id: String,
///     pub workload_identity_federation_provider_id: String,
///     /// Service account email for impersonation. When set, the federated token
///     /// is exchanged for a service account access token.
///     #[serde(default, skip_serializing_if = "Option::is_none")]
///     pub service_account_email: Option<String>,
/// }
/// 
/// /// AWS provider configuration for OIDC federation
/// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// pub struct AwsProviderConfig {
///     pub role_arn: String,
/// }
/// 
/// /// Cloud provider configurations for automatic authentication
/// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
/// pub struct ProvidersConfig {
///     #[serde(default, skip_serializing_if = "Option::is_none")]
///     pub gcp: Option<GcpProviderConfig>,
///     #[serde(default, skip_serializing_if = "Option::is_none")]
///     pub aws: Option<AwsProviderConfig>,
/// }
/// 
/// impl ProvidersConfig {
///     pub fn is_empty(&self) -> bool {
///         self.gcp.is_none() && self.aws.is_none()
///     }
/// }

// Stub types to maintain compatibility
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GcpProviderConfig {
    #[serde(default)]
    pub project_number: String,
    #[serde(default)]
    pub workload_identity_federation_pool_id: String,
    #[serde(default)]
    pub workload_identity_federation_provider_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_account_email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AwsProviderConfig {
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ProvidersConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gcp: Option<GcpProviderConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aws: Option<AwsProviderConfig>,
}

impl ProvidersConfig {
    pub fn is_empty(&self) -> bool {
        // Always return true as providers are disabled
        true
    }
}

/// An AmbientAgentEnvironment represents an environment that we would run a Warp agent in.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AmbientAgentEnvironment {
    /// Environment name
    #[serde(default)]
    pub name: String,
    /// Optional description of the environment (max 240 characters)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of GitHub repositories
    #[serde(default)]
    pub github_repos: Vec<GithubRepo>,
    /// Base image specification
    #[serde(flatten)]
    pub base_image: BaseImage,
    /// List of setup commands to run after cloning
    #[serde(default)]
    pub setup_commands: Vec<String>,
    /// Optional cloud provider configurations for automatic auth.
    /// #[serde(default, skip_serializing_if = "ProvidersConfig::is_empty")]
    /// pub providers: ProvidersConfig,
    #[serde(default, skip_serializing)]
    pub providers: ProvidersConfig,
}

impl AmbientAgentEnvironment {
    pub fn new(
        name: String,
        description: Option<String>,
        github_repos: Vec<GithubRepo>,
        docker_image: String,
        setup_commands: Vec<String>,
    ) -> Self {
        Self {
            name,
            description,
            github_repos,
            base_image: BaseImage::DockerImage(docker_image),
            setup_commands,
            providers: ProvidersConfig::default(),
        }
    }
}

impl JsonModel for AmbientAgentEnvironment {
    fn json_object_type() -> JsonObjectType {
        JsonObjectType::CloudEnvironment
    }
}

impl StringModel for AmbientAgentEnvironment {
    type CloudObjectType = GenericCloudObject<GenericStringObjectId, GenericStringModel<Self, JsonSerializer>>;

    fn model_type_name(&self) -> &'static str {
        "Ambient Agent Environment"
    }

    fn should_enforce_revisions() -> bool {
        false
    }

    fn model_format() -> GenericStringObjectFormat {
        GenericStringObjectFormat::Json(Self::json_object_type())
    }

    fn should_show_activity_toasts() -> bool {
        false
    }

    fn warn_if_unsaved_at_quit() -> bool {
        false
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn uniqueness_key(&self) -> Option<GenericStringObjectUniqueKey> {
        None
    }
}

pub type CloudAmbientAgentEnvironment =
    GenericCloudObject<GenericStringObjectId, CloudAmbientAgentEnvironmentModel>;
pub type CloudAmbientAgentEnvironmentModel =
    GenericStringModel<AmbientAgentEnvironment, JsonSerializer>;
pub type ServerAmbientAgentEnvironment =
    GenericServerObject<GenericStringObjectId, CloudAmbientAgentEnvironmentModel>;

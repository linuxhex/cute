use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use cute_graphql::mutations::create_simple_integration::{
    CreateSimpleIntegrationOutput,
};
use cute_graphql::queries::get_integrations_using_environment::{
    GetIntegrationsUsingEnvironmentOutput,
};
use cute_graphql::queries::get_oauth_connect_tx_status::{
    OauthConnectTxStatus,
};
use cute_graphql::queries::get_simple_integrations::{
    SimpleIntegrationsOutput,
};
use cute_graphql::queries::suggest_cloud_environment_image::{
    SuggestCloudEnvironmentImageResult,
};
use cute_graphql::queries::user_repo_auth_status::{
    UserRepoAuthStatusOutput,
};
use cute_graphql::queries::user_github_info::{
    UserGithubInfoResult,
};

use super::ServerApi;

#[cfg(not(target_family = "wasm"))]
pub trait IntegrationsClientBounds: Send + Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T: 'static + Send + Sync> IntegrationsClientBounds for T {}

#[cfg(target_family = "wasm")]
pub trait IntegrationsClientBounds {}

#[cfg(target_family = "wasm")]
impl<T: 'static> IntegrationsClientBounds for T {}

#[cfg_attr(test, automock)]
#[cfg_attr(target_family = "wasm", allow(dead_code))]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
pub trait IntegrationsClient: 'static + IntegrationsClientBounds {
    /// Checks the user's GitHub authorization status for the given repositories.
    async fn check_user_repo_auth_status(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<UserRepoAuthStatusOutput>;

    /// Creates or updates a simple integration on the server.
    #[allow(clippy::too_many_arguments)]
    async fn create_or_update_simple_integration(
        &self,
        integration_type: String,
        is_update: bool,
        environment_uid: Option<String>,
        base_prompt: Option<String>,
        model_id: Option<String>,
        mcp_servers_json: Option<String>,
        remove_mcp_server_names: Option<Vec<String>>,
        worker_host: Option<String>,
        enabled: bool,
    ) -> Result<CreateSimpleIntegrationOutput>;

    /// Lists simple integrations for a fixed set of provider slugs.
    async fn list_simple_integrations(
        &self,
        providers: Vec<String>,
    ) -> Result<SimpleIntegrationsOutput>;

    /// Polls the status of an OAuth connect transaction.
    async fn poll_oauth_connect_status(&self, tx_id: String) -> Result<OauthConnectTxStatus>;

    /// Gets the list of integration provider names that are using the specified environment.
    async fn get_integrations_using_environment(
        &self,
        environment_id: String,
    ) -> Result<GetIntegrationsUsingEnvironmentOutput>;

    /// Gets the user's GitHub connection info, including accessible repos.
    async fn get_user_github_info(&self) -> Result<UserGithubInfoResult>;

    /// Suggests a Docker image for a cloud environment based on the provided repos.
    async fn suggest_cloud_environment_image(
        &self,
        repos: Vec<(String, String)>,
    ) -> Result<SuggestCloudEnvironmentImageResult>;
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl IntegrationsClient for ServerApi {
    async fn check_user_repo_auth_status(
        &self,
        _repos: Vec<(String, String)>,
    ) -> Result<UserRepoAuthStatusOutput> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_or_update_simple_integration(
        &self,
        _integration_type: String,
        _is_update: bool,
        _environment_uid: Option<String>,
        _base_prompt: Option<String>,
        _model_id: Option<String>,
        _mcp_servers_json: Option<String>,
        _remove_mcp_server_names: Option<Vec<String>>,
        _worker_host: Option<String>,
        _enabled: bool,
    ) -> Result<CreateSimpleIntegrationOutput> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    async fn get_integrations_using_environment(
        &self,
        _environment_id: String,
    ) -> Result<GetIntegrationsUsingEnvironmentOutput> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    async fn list_simple_integrations(
        &self,
        _providers: Vec<String>,
    ) -> Result<SimpleIntegrationsOutput> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    async fn poll_oauth_connect_status(&self, _tx_id: String) -> Result<OauthConnectTxStatus> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    async fn get_user_github_info(&self) -> Result<UserGithubInfoResult> {
        Err(anyhow!("Integration operations not supported in local version"))
    }

    async fn suggest_cloud_environment_image(
        &self,
        _repos: Vec<(String, String)>,
    ) -> Result<SuggestCloudEnvironmentImageResult> {
        Err(anyhow!("Integration operations not supported in local version"))
    }
}

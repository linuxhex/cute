use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use cute_graphql::managed_secrets::{ManagedSecret, ManagedSecretType};
pub use cute_managed_secrets::client::{ManagedSecretConfigs, ManagedSecretsClient};
use cute_managed_secrets::client::{SecretOwner, TaskIdentityToken};

use super::ServerApi;

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl ManagedSecretsClient for ServerApi {
    async fn get_managed_secret_configs(&self) -> Result<ManagedSecretConfigs> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }

    async fn create_managed_secret(
        &self,
        _owner: SecretOwner,
        _name: String,
        _secret_type: ManagedSecretType,
        _encrypted_value: String,
        _description: Option<String>,
    ) -> Result<ManagedSecret> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }

    async fn delete_managed_secret(&self, _owner: SecretOwner, _name: String) -> Result<()> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }

    async fn update_managed_secret(
        &self,
        _owner: SecretOwner,
        _name: String,
        _encrypted_value: Option<String>,
        _description: Option<String>,
    ) -> Result<ManagedSecret> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }

    async fn list_secrets(&self) -> Result<Vec<ManagedSecret>> {
        Ok(vec![])
    }

    async fn list_harness_auth_secrets(
        &self,
        _harness: cute_graphql::ai::AgentHarness,
    ) -> Result<Vec<ManagedSecret>> {
        Ok(vec![])
    }

    async fn get_task_secrets(
        &self,
        _task_id: String,
        _workload_token: String,
    ) -> Result<HashMap<String, cute_graphql::queries::task_secrets::ManagedSecretValue>> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }

    async fn issue_task_identity_token(
        &self,
        _options: cute_managed_secrets::client::IdentityTokenOptions,
    ) -> Result<TaskIdentityToken> {
        Err(anyhow!("Managed secrets not supported in local version"))
    }
}

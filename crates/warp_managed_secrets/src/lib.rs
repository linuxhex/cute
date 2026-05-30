//! Stub implementation for warp_managed_secrets crate
//! This provides minimal types to satisfy compilation without actual secrets management functionality

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedSecret {
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretType {
    Environment,
    File,
}

pub fn get_secrets() -> Vec<ManagedSecret> {
    vec![]
}

pub mod client {
    use super::*;
    use async_trait::async_trait;
    use anyhow::Result;

    #[derive(Debug, Clone)]
    pub struct SecretsClient {
        pub id: String,
    }

    impl SecretsClient {
        pub fn new() -> Self {
            Self { id: String::new() }
        }
    }

    #[cfg_attr(not(target_family = "wasm"), async_trait)]
    #[cfg_attr(target_family = "wasm", async_trait(?Send))]
    pub trait ManagedSecretsClient: Send + Sync {
        async fn get_managed_secret_configs(&self) -> Result<ManagedSecretConfigs>;
        async fn list_managed_secrets(&self, _owner: SecretOwner) -> Result<Vec<ManagedSecret>> { Ok(vec![]) }
        async fn create_managed_secret(&self, _secret: ManagedSecret) -> Result<ManagedSecret> { Err(anyhow::anyhow!("Not implemented")) }
        async fn update_managed_secret(&self, _secret: ManagedSecret) -> Result<ManagedSecret> { Err(anyhow::anyhow!("Not implemented")) }
        async fn delete_managed_secret(&self, _id: &str) -> Result<()> { Err(anyhow::anyhow!("Not implemented")) }
        async fn list_harness_auth_secrets(&self) -> Result<Vec<ManagedSecret>> { Ok(vec![]) }
        async fn list_secrets(&self) -> Result<Vec<ManagedSecret>> { Ok(vec![]) }
        async fn get_task_secrets(&self, _task_id: String, _workload_token: String) -> Result<Vec<ManagedSecret>> { Ok(vec![]) }
        async fn issue_task_identity_token(&self, _options: IdentityTokenOptions) -> Result<TaskIdentityToken> { Err(anyhow::anyhow!("Not implemented")) }
    }

    #[derive(Debug, Clone)]
    pub enum SecretsClientEvent {
        None,
    }

    #[derive(Debug, Clone)]
    pub struct ManagedSecretConfigs {
        pub id: String,
    }

    #[derive(Debug, Clone)]
    pub struct SecretOwner {
        pub id: String,
    }

    #[derive(Debug, Clone)]
    pub struct TaskIdentityToken {
        pub token: String,
    }

    #[derive(Debug, Clone, Default)]
    pub struct IdentityTokenOptions {
        pub id: String,
    }
}

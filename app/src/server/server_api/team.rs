use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::ServerApi;
use crate::workspaces::user_workspaces::WorkspacesMetadataWithPricing;

/// Minimal TeamClient trait for local version.
/// In the cloud version, this provides many team-related API operations.
/// In the local version, we only need the workspace metadata operation.
#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait TeamClient: 'static + Send + Sync {
    async fn workspaces_metadata(&self) -> Result<WorkspacesMetadataWithPricing>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl TeamClient for ServerApi {
    async fn workspaces_metadata(&self) -> Result<WorkspacesMetadataWithPricing> {
        // Return empty metadata for local version
        Ok(WorkspacesMetadataWithPricing {
            metadata: crate::workspaces::user_workspaces::WorkspacesMetadataResponse {
                workspaces: vec![],
                joinable_teams: vec![],
                feature_model_choices: None,
            },
            pricing_info: None,
        })
    }
}

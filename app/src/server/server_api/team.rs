use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::ServerApi;
use crate::cloud_stub_types::CloudObjectEventEntrypoint;
use crate::server::ids::ServerId;

/// Minimal TeamClient trait for local version.
/// In the cloud version, this provides many team-related API operations.
/// In the local version, we only need the workspace metadata operation.
#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait TeamClient: 'static + Send + Sync {
    async fn workspaces_metadata(&self) -> Result<WorkspacesMetadataWithPricing>;

    // Team operations - not supported in local version
    
    // COMMENTED: Team invitation operations
    // async fn send_team_invite_email(&self, _team_uid: ServerId, _email: String) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
    
    // COMMENTED: Team invitation link operations
    // async fn add_invite_link_domain_restriction(&self, _team_uid: ServerId, _domain: String) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
    // 
    // async fn delete_invite_link_domain_restriction(&self, _team_uid: ServerId, _domain_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
    // 
    // async fn set_is_invite_link_enabled(&self, _team_uid: ServerId, _enabled: bool) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
    // 
    // async fn reset_invite_links(&self, _team_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
    // 
    // async fn delete_team_invite(&self, _team_uid: ServerId, _invitee_email: String) -> Result<WorkspacesMetadataWithPricing> {
    //     Err(anyhow!("Team operations not supported in local version"))
    // }
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

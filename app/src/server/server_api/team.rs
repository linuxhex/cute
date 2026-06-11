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

    // Team operations - not supported in local version
    async fn create_team(&self, _name: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn leave_team(&self) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn rename_team(&self, _name: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn remove_user_from_team(&self, _user_email: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn add_invite_link_domain_restriction(&self, _domain: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn delete_invite_link_domain_restriction(&self, _domain: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn send_team_invite_email(&self, _email: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_is_invite_link_enabled(&self, _enabled: bool) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn reset_invite_links(&self) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_team_discoverability(&self, _discoverable: bool) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn join_team_with_team_discovery(&self, _team_uid: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn get_discoverable_teams(&self) -> Result<Vec<crate::workspaces::DiscoverableTeam>> {
        Ok(vec![])
    }

    async fn transfer_team_ownership(&self, _new_owner_email: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_team_member_role(&self, _user_email: &str, _role: crate::workspaces::MembershipRole) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn delete_team_invite(&self, _invite_id: &str) -> Result<()> {
        Err(anyhow!("Team operations not supported in local version"))
    }
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

use anyhow::{anyhow, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

use super::ServerApi;
use crate::cloud_object::CloudObjectEventEntrypoint;
use crate::server::ids::ServerId;
use crate::workspaces::user_workspaces::{CreateTeamResponse, WorkspacesMetadataWithPricing};

/// Minimal TeamClient trait for local version.
/// In the cloud version, this provides many team-related API operations.
/// In the local version, we only need the workspace metadata operation.
#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait TeamClient: 'static + Send + Sync {
    async fn workspaces_metadata(&self) -> Result<WorkspacesMetadataWithPricing>;

    // Team operations - not supported in local version
    async fn create_team(&self, _name: String, _entrypoint: CloudObjectEventEntrypoint, _discoverable: Option<bool>) -> Result<CreateTeamResponse> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn leave_team(&self, _user_uid: crate::auth::UserUid, _team_uid: ServerId, _entrypoint: CloudObjectEventEntrypoint) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn rename_team(&self, _name: String, _team_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn remove_user_from_team(&self, _user_uid: crate::auth::UserUid, _team_uid: ServerId, _entrypoint: CloudObjectEventEntrypoint) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn add_invite_link_domain_restriction(&self, _team_uid: ServerId, _domain: String) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn delete_invite_link_domain_restriction(&self, _team_uid: ServerId, _domain_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn send_team_invite_email(&self, _team_uid: ServerId, _email: String) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_is_invite_link_enabled(&self, _team_uid: ServerId, _enabled: bool) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn reset_invite_links(&self, _team_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_team_discoverability(&self, _team_uid: ServerId, _discoverable: bool) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn join_team_with_team_discovery(&self, _team_uid: ServerId) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn get_discoverable_teams(&self) -> Result<Vec<crate::workspaces::DiscoverableTeam>> {
        Ok(vec![])
    }

    async fn transfer_team_ownership(&self, _new_owner_email: String) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn set_team_member_role(&self, _user_uid: crate::auth::UserUid, _team_uid: ServerId, _role: crate::workspaces::MembershipRole) -> Result<WorkspacesMetadataWithPricing> {
        Err(anyhow!("Team operations not supported in local version"))
    }

    async fn delete_team_invite(&self, _team_uid: ServerId, _invitee_email: String) -> Result<WorkspacesMetadataWithPricing> {
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

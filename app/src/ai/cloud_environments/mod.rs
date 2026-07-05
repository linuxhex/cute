// Note: Cloud-specific logic has been removed.

use cuteui::{AppContext, SingletonEntity as _};

use crate::auth::AuthStateProvider;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::cloud_object::Owner;

// Re-export types from cloud_object::models::cloud_environment
pub use crate::cloud_object::models::cloud_environment::{
    AmbientAgentEnvironment, AwsProviderConfig, BaseImage, CloudAmbientAgentEnvironment,
    CloudAmbientAgentEnvironmentModel, GcpProviderConfig, GithubRepo, ProvidersConfig,
};

/// Stub function: Get the owner for a new environment (team preference).
/// Returns None if the user is not logged in or has no team.
pub fn owner_for_new_environment(ctx: &AppContext) -> Option<Owner> {
    // TODO: Implement proper owner resolution logic
    log::debug!("owner_for_new_environment called (stub)");
    
    // Try to get team UID first
    if let Some(team_uid) = UserWorkspaces::as_ref(ctx).current_team_uid() {
        return Some(Owner::Team { team_uid });
    }
    
    // Fall back to personal environment
    owner_for_new_personal_environment(ctx)
}

/// Stub function: Get the owner for a new personal environment.
/// Returns None if the user is not logged in.
pub fn owner_for_new_personal_environment(ctx: &AppContext) -> Option<Owner> {
    // TODO: Implement proper owner resolution logic
    log::debug!("owner_for_new_personal_environment called (stub)");
    
    AuthStateProvider::as_ref(ctx)
        .get()
        .user_id()
        .map(|user_uid| Owner::User { user_uid })
}

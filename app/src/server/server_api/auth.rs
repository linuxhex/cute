use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;

use super::ServerApi;
use cute_core::errors::AnyhowErrorExt as _;

/// OAuth2 client type alias for device authentication flow.
/// This type has auth_url, device_auth_url, and token_url endpoints set.
pub type OAuth2Client = oauth2::basic::BasicClient<
    oauth2::EndpointSet,       // HasAuthUrl
    oauth2::EndpointSet,       // HasDeviceAuthUrl
    oauth2::EndpointNotSet,    // HasIntrospectionUrl
    oauth2::EndpointNotSet,    // HasRevocationUrl
    oauth2::EndpointSet,       // HasTokenUrl
>;
use crate::auth::credentials::{AuthToken, Credentials};
use crate::auth::user::User;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct AgentIdentity {
    pub uid: String,
    pub name: String,
    pub available: bool,
}

pub const AMBIENT_WORKLOAD_TOKEN_HEADER: &str = "X-Warp-Ambient-Workload-Token";
pub const CLOUD_AGENT_ID_HEADER: &str = "X-Warp-Cloud-Agent-ID";
const AMBIENT_WORKLOAD_TOKEN_DURATION: instant::Duration = instant::Duration::from_secs(3 * 60 * 60);

pub struct SyncedUserSettings {
    pub is_cloud_conversation_storage_enabled: bool,
    pub is_crash_reporting_enabled: bool,
    pub is_telemetry_enabled: bool,
}

impl Default for SyncedUserSettings {
    fn default() -> Self {
        Self {
            is_cloud_conversation_storage_enabled: false,
            is_crash_reporting_enabled: false,
            is_telemetry_enabled: false,
        }
    }
}

pub struct FetchUserResult {
    pub user: User,
    pub credentials: Credentials,
    pub from_refresh: bool,
    pub llms: crate::ai::llms::ModelsByFeature,
}

#[cfg_attr(test, mockall::automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AuthClient: 'static + Send + Sync {
    async fn get_or_refresh_access_token(&self) -> Result<AuthToken>;
    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>>;
    async fn set_is_telemetry_enabled(&self, value: bool) -> Result<()>;
    async fn set_is_crash_reporting_enabled(&self, value: bool) -> Result<()>;
    async fn set_is_cloud_conversation_storage_enabled(&self, value: bool) -> Result<()>;
    async fn update_user_settings(&self, settings_snapshot: crate::settings::PrivacySettingsSnapshot) -> Result<()>;
    async fn set_user_is_onboarded(&self) -> Result<bool>;
    async fn list_api_keys(&self) -> Result<Vec<cute_graphql::queries::api_keys::ApiKeyProperties>>;
    async fn create_api_key(
        &self,
        name: String,
        team_id: Option<cynic::Id>,
        agent_uid: Option<cynic::Id>,
        expires_at: Option<cute_graphql::scalars::Time>,
    ) -> Result<cute_graphql::mutations::generate_api_key::GenerateApiKeyResult>;
    async fn expire_api_key(&self, key_uid: &crate::server::ids::ApiKeyUid) -> Result<cute_graphql::mutations::expire_api_key::ExpireApiKeyResult>;
    async fn list_agent_identities(&self) -> Result<Vec<AgentIdentity>>;
    async fn get_or_create_ambient_workload_token(&self) -> Result<Option<String>>;
}

impl ServerApi {
    pub(super) async fn access_token(&self) -> Result<AuthToken> {
        if cfg!(feature = "skip_login") {
            bail!("skip_login enabled; failing all authenticated requests");
        }

        let Some(credentials) = self.auth_state.credentials() else {
            bail!("missing authentication credentials");
        };

        match credentials {
            Credentials::ApiKey { key, .. } => Ok(AuthToken::ApiKey(key)),
            Credentials::Bearer(token) => Ok(AuthToken::Bearer(token)),
            Credentials::SessionCookie => Ok(AuthToken::NoAuth),
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Credentials::Test => Ok(AuthToken::NoAuth),
        }
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl AuthClient for ServerApi {
    async fn get_or_refresh_access_token(&self) -> Result<AuthToken> {
        self.access_token().await
    }

    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>> {
        Ok(Some(SyncedUserSettings::default()))
    }

    async fn set_is_telemetry_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn set_is_crash_reporting_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn set_is_cloud_conversation_storage_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn update_user_settings(&self, _settings_snapshot: crate::settings::PrivacySettingsSnapshot) -> Result<()> {
        Ok(())
    }

    async fn set_user_is_onboarded(&self) -> Result<bool> {
        Ok(true)
    }

    async fn list_api_keys(&self) -> Result<Vec<cute_graphql::queries::api_keys::ApiKeyProperties>> {
        Ok(vec![])
    }

    async fn create_api_key(
        &self,
        _name: String,
        _team_id: Option<cynic::Id>,
        _agent_uid: Option<cynic::Id>,
        _expires_at: Option<cute_graphql::scalars::Time>,
    ) -> Result<cute_graphql::mutations::generate_api_key::GenerateApiKeyResult> {
        Err(anyhow!("API keys not supported in local version"))
    }

    async fn expire_api_key(&self, _key_uid: &crate::server::ids::ApiKeyUid) -> Result<cute_graphql::mutations::expire_api_key::ExpireApiKeyResult> {
        Err(anyhow!("API keys not supported in local version"))
    }

    async fn list_agent_identities(&self) -> Result<Vec<AgentIdentity>> {
        Ok(vec![])
    }

    async fn get_or_create_ambient_workload_token(&self) -> Result<Option<String>> {
        if cfg!(target_family = "wasm") {
            return Ok(None);
        }

        {
            let cached = self.ambient_workload_token.lock();
            if let Some(ref token) = *cached {
                let is_valid = token.expires_at.is_none_or(|expires_at| {
                    chrono::Utc::now() + chrono::Duration::minutes(5) < expires_at
                });
                if is_valid {
                    return Ok(Some(token.token.clone()));
                }
            }
        }

        let workload_token = match cute_isolation_platform::issue_workload_token(Some(
            AMBIENT_WORKLOAD_TOKEN_DURATION,
        ))
        .await
        {
            Ok(token) => token,
            Err(cute_isolation_platform::IsolationPlatformError::NoIsolationPlatformDetected) => {
                return Ok(None);
            }
            Err(e) => return Err(e.into()),
        };

        let token_str = workload_token.token.clone();

        {
            let mut cached = self.ambient_workload_token.lock();
            *cached = Some(workload_token);
        }

        Ok(Some(token_str))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UserAuthenticationError {
    #[error("unexpected error occurred when fetching an ID token: {0:#}")]
    Unexpected(#[from] anyhow::Error),
}

impl cute_core::errors::ErrorExt for UserAuthenticationError {
    fn is_actionable(&self) -> bool {
        match self {
            UserAuthenticationError::Unexpected(err) => err.is_actionable(),
        }
    }
}

super::register_error!(UserAuthenticationError);

use std::result::Result as StdResult;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context as _, Result};
use async_trait::async_trait;
use firebase::{FetchAccessTokenResponse, FirebaseError};
use instant::Duration;
#[cfg(test)]
use mockall::{automock, predicate::*};
use oauth2::TokenResponse;
use thiserror::Error;
use warp_core::errors::{AnyhowErrorExt, ErrorExt};

use super::ServerApi;
use crate::auth::credentials::{AuthToken, Credentials, FirebaseToken, LoginToken, RefreshToken};
use crate::auth::user::{FirebaseAuthTokens, User};
use crate::auth::UserUid;
use crate::channel::ChannelState;
use crate::server::server_api::register_error;

/// A named agent identity from the public API.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct AgentIdentity {
    pub uid: String,
    pub name: String,
    pub available: bool,
}

/// Error messages returned from the Firebase REST API when attempting to convert a refresh token
/// into an access token that indicate the user's token is in an errored state.
static FETCH_ACCESS_TOKEN_SOFT_ERROR_MESSAGES: &[&str] = &[
    "TOKEN_EXPIRED",
    "INVALID_REFRESH_TOKEN",
    "MISSING_REFRESH_TOKEN",
];

/// Error messages returned from the Firebase REST API when attempting to convert a refresh token
/// into an access token that indicate the user's account is in an errored state.
static FETCH_ACCESS_TOKEN_HARD_ERROR_MESSAGES: &[&str] = &["USER_DISABLED", "USER_NOT_FOUND"];

const FETCH_ACCESS_TOKEN_TIMEOUT: Duration = Duration::from_secs(5);

/// Header key for the ambient workload token attached to multi-agent requests.
pub const AMBIENT_WORKLOAD_TOKEN_HEADER: &str = "X-Warp-Ambient-Workload-Token";

/// Header key for the cloud agent task ID attached to requests from ambient agents.
pub const CLOUD_AGENT_ID_HEADER: &str = "X-Warp-Cloud-Agent-ID";

/// Duration for which the ambient workload token is valid (3 hours).
const AMBIENT_WORKLOAD_TOKEN_DURATION: Duration = Duration::from_secs(3 * 60 * 60);

/// User settings that are currently 'synced' (e.g. stored server-side) on a per-user basis.
#[derive(Copy, Clone, Debug, Default)]
pub struct SyncedUserSettings {
    pub is_cloud_conversation_storage_enabled: bool,
    pub is_crash_reporting_enabled: bool,
    pub is_telemetry_enabled: bool,
}

/// Results of an attempt to fetch the current user.
pub struct FetchUserResult {
    pub user: User,
    pub credentials: Credentials,
    pub from_refresh: bool,
    pub llms: crate::ai::llms::ModelsByFeature,
}

#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AuthClient: 'static + Send + Sync {
    async fn create_anonymous_user(
        &self,
        anonymous_user_type: warp_graphql::mutations::create_anonymous_user::AnonymousUserType,
    ) -> Result<warp_graphql::mutations::create_anonymous_user::CreateAnonymousUserResult>;

    async fn get_or_refresh_access_token(&self) -> Result<AuthToken>;

    async fn fetch_user(
        &self,
        token: LoginToken,
        for_refresh: bool,
    ) -> StdResult<FetchUserResult, UserAuthenticationError>;

    async fn fetch_new_custom_token(&self) -> Result<warp_graphql::mutations::mint_custom_token::MintCustomTokenResult>;

    fn on_custom_token_fetched(
        &self,
        response: Result<warp_graphql::mutations::mint_custom_token::MintCustomTokenResult>,
    ) -> Result<String, MintCustomTokenError>;

    async fn fetch_user_properties<'a>(&self, auth_token: Option<&'a str>)
        -> Result<warp_graphql::queries::get_user::UserOutput>;

    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>>;

    #[allow(dead_code)]
    async fn get_conversation_usage_history(
        &self,
        days: Option<i32>,
        limit: Option<i32>,
        last_updated_end_timestamp: Option<warp_graphql::scalars::Time>,
    ) -> Result<Vec<warp_graphql::queries::get_conversation_usage::ConversationUsage>>;

    async fn set_is_telemetry_enabled(&self, value: bool) -> Result<()>;

    async fn set_is_crash_reporting_enabled(&self, value: bool) -> Result<()>;

    async fn set_is_cloud_conversation_storage_enabled(&self, value: bool) -> Result<()>;

    async fn update_user_settings(&self, settings_snapshot: crate::settings::PrivacySettingsSnapshot) -> Result<()>;

    async fn set_user_is_onboarded(&self) -> Result<bool>;

    async fn request_device_code(
        &self,
    ) -> StdResult<oauth2::StandardDeviceAuthorizationResponse, UserAuthenticationError>;

    async fn exchange_device_access_token(
        &self,
        details: &oauth2::StandardDeviceAuthorizationResponse,
        timeout: Duration,
    ) -> StdResult<FirebaseToken, UserAuthenticationError>;

    async fn list_api_keys(&self) -> Result<Vec<warp_graphql::queries::api_keys::ApiKeyProperties>>;

    async fn create_api_key(
        &self,
        name: String,
        team_id: Option<cynic::Id>,
        agent_uid: Option<cynic::Id>,
        expires_at: Option<warp_graphql::scalars::Time>,
    ) -> Result<warp_graphql::mutations::generate_api_key::GenerateApiKeyResult>;

    async fn expire_api_key(&self, key_uid: &crate::server::ids::ApiKeyUid) -> Result<warp_graphql::mutations::expire_api_key::ExpireApiKeyResult>;

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
            Credentials::Firebase(auth_tokens) => {
                let expiration_time = auth_tokens.expiration_time;

                if chrono::Utc::now() + chrono::Duration::minutes(5) >= expiration_time {
                    let refresh_token = auth_tokens.refresh_token.clone();
                    let firebase_token = FirebaseToken::Refresh(RefreshToken::new(refresh_token));

                    let result = fetch_auth_tokens(self.client.clone(), firebase_token).await;

                    if let Err(UserAuthenticationError::DeniedAccessToken(_)) = result {
                        let _ = self.event_sender.send(super::ServerApiEvent::NeedsReauth).await;
                    }
                    let new_firebase_token_info = result?;
                    self.auth_state
                        .update_firebase_tokens(new_firebase_token_info.clone());
                    let _ = self
                        .event_sender
                        .send(super::ServerApiEvent::AccessTokenRefreshed {
                            token: new_firebase_token_info.id_token.clone(),
                        })
                        .await;
                    return Ok(AuthToken::Firebase(new_firebase_token_info.id_token));
                }

                Ok(AuthToken::Firebase(auth_tokens.id_token))
            }
            Credentials::SessionCookie => Ok(AuthToken::NoAuth),
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Credentials::Test => Ok(AuthToken::NoAuth),
        }
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl AuthClient for ServerApi {
    async fn create_anonymous_user(
        &self,
        _anonymous_user_type: warp_graphql::mutations::create_anonymous_user::AnonymousUserType,
    ) -> Result<warp_graphql::mutations::create_anonymous_user::CreateAnonymousUserResult> {
        Err(anyhow!("Authentication not supported in local version"))
    }

    async fn get_or_refresh_access_token(&self) -> Result<AuthToken> {
        self.access_token().await
    }

    async fn fetch_user(
        &self,
        token: LoginToken,
        for_refresh: bool,
    ) -> StdResult<FetchUserResult, UserAuthenticationError> {
        let new_credentials = exchange_credentials(self.client.clone(), token).await?;
        // Return a minimal user for local version
        let user = User {
            is_onboarded: true,
            local_id: UserUid::new("local"),
            metadata: Default::default(),
            needs_sso_link: false,
            anonymous_user_type: None,
            is_on_work_domain: false,
            linked_at: None,
            personal_object_limits: None,
            principal_type: Default::default(),
            global_skills: vec![],
        };
        Ok(FetchUserResult {
            user,
            credentials: new_credentials,
            from_refresh: for_refresh,
            llms: Default::default(),
        })
    }

    async fn fetch_new_custom_token(&self) -> Result<warp_graphql::mutations::mint_custom_token::MintCustomTokenResult> {
        Err(anyhow!("Authentication not supported in local version"))
    }

    fn on_custom_token_fetched(
        &self,
        response: Result<warp_graphql::mutations::mint_custom_token::MintCustomTokenResult>,
    ) -> Result<String, MintCustomTokenError> {
        match response {
            Ok(response_data) => match response_data {
                warp_graphql::mutations::mint_custom_token::MintCustomTokenResult::MintCustomTokenOutput(output) => Ok(output.custom_token),
                warp_graphql::mutations::mint_custom_token::MintCustomTokenResult::UserFacingError(_) => {
                    Err(MintCustomTokenError::UserFacingError("Error".to_string()))
                }
                warp_graphql::mutations::mint_custom_token::MintCustomTokenResult::Unknown => Err(MintCustomTokenError::Unknown),
            },
            Err(_) => Err(MintCustomTokenError::Unknown),
        }
    }

    async fn fetch_user_properties<'a>(
        &self,
        _auth_token: Option<&'a str>,
    ) -> Result<warp_graphql::queries::get_user::UserOutput> {
        Err(anyhow!("User properties not supported in local version"))
    }

    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>> {
        Ok(Some(SyncedUserSettings::default()))
    }

    async fn get_conversation_usage_history(
        &self,
        _days: Option<i32>,
        _limit: Option<i32>,
        _last_updated_end_timestamp: Option<warp_graphql::scalars::Time>,
    ) -> Result<Vec<warp_graphql::queries::get_conversation_usage::ConversationUsage>> {
        Ok(vec![])
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

    async fn request_device_code(
        &self,
    ) -> StdResult<oauth2::StandardDeviceAuthorizationResponse, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(anyhow!("Device auth not supported in local version")))
    }

    async fn exchange_device_access_token(
        &self,
        _details: &oauth2::StandardDeviceAuthorizationResponse,
        _timeout: Duration,
    ) -> StdResult<FirebaseToken, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(anyhow!("Device auth not supported in local version")))
    }

    async fn list_api_keys(&self) -> Result<Vec<warp_graphql::queries::api_keys::ApiKeyProperties>> {
        Ok(vec![])
    }

    async fn create_api_key(
        &self,
        _name: String,
        _team_id: Option<cynic::Id>,
        _agent_uid: Option<cynic::Id>,
        _expires_at: Option<warp_graphql::scalars::Time>,
    ) -> Result<warp_graphql::mutations::generate_api_key::GenerateApiKeyResult> {
        Err(anyhow!("API keys not supported in local version"))
    }

    async fn expire_api_key(&self, _key_uid: &crate::server::ids::ApiKeyUid) -> Result<warp_graphql::mutations::expire_api_key::ExpireApiKeyResult> {
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

        let workload_token = match warp_isolation_platform::issue_workload_token(Some(
            AMBIENT_WORKLOAD_TOKEN_DURATION,
        ))
        .await
        {
            Ok(token) => token,
            Err(warp_isolation_platform::IsolationPlatformError::NoIsolationPlatformDetected) => {
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

async fn exchange_credentials(
    client: Arc<http_client::Client>,
    token: LoginToken,
) -> StdResult<Credentials, UserAuthenticationError> {
    match token {
        LoginToken::Firebase(firebase_token) => {
            let tokens = fetch_auth_tokens(client, firebase_token).await?;
            Ok(Credentials::Firebase(tokens))
        }
        LoginToken::ApiKey(key) => Ok(Credentials::ApiKey {
            key,
            owner_type: None,
        }),
        LoginToken::SessionCookie => Ok(Credentials::SessionCookie),
    }
}

fn fetch_auth_tokens(
    client: Arc<http_client::Client>,
    token: FirebaseToken,
) -> warpui::r#async::BoxFuture<'static, StdResult<FirebaseAuthTokens, UserAuthenticationError>> {
    Box::pin(async move {
        let firebase_api_key = ChannelState::firebase_api_key();
        let url = token.access_token_url(&firebase_api_key);
        let request_body = token.access_token_request_body();
        let proxy_url = token.proxy_url(&ChannelState::server_root_url(), &firebase_api_key);
        let response = match client
            .post(&url)
            .form(&request_body)
            .timeout(FETCH_ACCESS_TOKEN_TIMEOUT)
            .send()
            .await
        {
            Ok(response) => match response.error_for_status_ref() {
                Ok(_) => Ok(response),
                Err(error) => {
                    log::warn!("Request to firebase failed: {error:?}");
                    fetch_access_token_via_proxy(client, &request_body, proxy_url).await
                }
            },
            Err(error) => {
                log::warn!("Failed to make request to firebase: {error:?}");
                fetch_access_token_via_proxy(client, &request_body, proxy_url).await
            }
        }?;

        let response = response
            .json::<FetchAccessTokenResponse>()
            .await
            .map_err(anyhow::Error::from)?;
        match response {
            FetchAccessTokenResponse::Success {
                id_token,
                expires_in,
                refresh_token,
            } => Ok(FirebaseAuthTokens::from_response(
                id_token,
                refresh_token,
                expires_in,
            )?),
            FetchAccessTokenResponse::Error { error } => Err(error.into()),
        }
    })
}

fn fetch_access_token_via_proxy<'a>(
    client: Arc<http_client::Client>,
    request_body: &'a [(&'a str, &'a str)],
    proxy_url: String,
) -> warpui::r#async::BoxFuture<'a, Result<http_client::Response>> {
    Box::pin(async move {
        client
            .post(&proxy_url)
            .form(request_body)
            .send()
            .await
            .map_err(anyhow::Error::from)
    })
}

pub type OAuth2Client = oauth2::basic::BasicClient<
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

#[derive(Error, Debug)]
pub enum UserAuthenticationError {
    #[error("Firebase returned a token error when fetching an ID token")]
    DeniedAccessToken(FirebaseError),
    #[error("Firebase returned a user error when fetching an ID token")]
    UserAccountDisabled(FirebaseError),
    #[error("Invalid state parameter in auth redirect")]
    InvalidStateParameter,
    #[error("Missing state parameter in auth redirect")]
    MissingStateParameter,
    #[error("unexpected error occurred when fetching an ID token: {0:#}")]
    Unexpected(#[from] anyhow::Error),
}

impl ErrorExt for UserAuthenticationError {
    fn is_actionable(&self) -> bool {
        match self {
            UserAuthenticationError::DeniedAccessToken(err) => {
                log::info!("ignoring denied access token error: {err:#}");
                false
            }
            UserAuthenticationError::UserAccountDisabled(err) => {
                log::info!("ignoring user account disabled error: {err:#}");
                false
            }
            UserAuthenticationError::Unexpected(err) => err.is_actionable(),
            UserAuthenticationError::InvalidStateParameter
            | UserAuthenticationError::MissingStateParameter => true,
        }
    }
}
register_error!(UserAuthenticationError);

impl From<FirebaseError> for UserAuthenticationError {
    fn from(error: FirebaseError) -> Self {
        if FETCH_ACCESS_TOKEN_SOFT_ERROR_MESSAGES.contains(&error.message.as_str()) {
            UserAuthenticationError::DeniedAccessToken(error)
        } else if FETCH_ACCESS_TOKEN_HARD_ERROR_MESSAGES.contains(&error.message.as_str()) {
            UserAuthenticationError::UserAccountDisabled(error)
        } else {
            UserAuthenticationError::Unexpected(
                anyhow::Error::from(error)
                    .context("Failed to exchange refresh token with access token."),
            )
        }
    }
}

#[derive(Error, Debug)]
pub enum AnonymousUserCreationError {
    #[error("The network request to create the anonymous user failed")]
    CreationFailed,
    #[error("Received a user facing error: {0}")]
    UserFacingError(String),
    #[error("The user was created, but the ID token could not be fetched")]
    UserAuthenticationFailed(#[from] UserAuthenticationError),
    #[error("Failed to create anonymous user with unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum MintCustomTokenError {
    #[error("Received a user facing error: {0}")]
    UserFacingError(String),
    #[error("Failed to create new custom token with unknown error")]
    Unknown,
}

pub const EXPERIMENT_ID_HEADER: &str = "X-Warp-Experiment-Id";

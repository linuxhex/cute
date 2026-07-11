use std::collections::HashMap;

use anyhow::{anyhow, bail};
use oauth2::{RefreshToken, TokenResponse as _};
use rmcp::transport::auth::{
    AuthClient, AuthorizationManager, CredentialStore, InMemoryCredentialStore, OAuthClientConfig,
    OAuthState, StoredCredentials,
};
use rmcp::transport::{AuthError, AuthorizationSession};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use cute_core::channel::ChannelState;
use cuteui::{ModelSpawner, SingletonEntity};
use cuteui_extras::secure_storage::AppContextExt as _;

use super::{MCPServerState, TemplatableMCPServerManager};
use crate::ai::mcp::FileBasedMCPManager;

pub(crate) const TEMPLATABLE_MCP_CREDENTIALS_KEY: &str = "TemplatableMcpCredentials";
pub(crate) const FILE_BASED_MCP_CREDENTIALS_KEY: &str = "FileBasedMcpCredentials";

/// The issuer URL for GitHub's OAuth provider.
const GITHUB_ISSUER: &str = "https://github.com/login/oauth";

static GITHUB_OAUTH_SCOPES: [&str; 7] = [
    "repo",
    "read:org",
    "gist",
    "notifications",
    "user",
    "project",
    "workflow",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedCredentials {
    /// The credential information that `rmcp` wants us to store and retrieve.
    #[serde(flatten)]
    credentials: StoredCredentials,
    /// The client secret for the OAuth application.
    ///
    /// This is needed to properly refresh tokens when using DCR (Dynamic Client Registration),
    /// as the server expects the client to provide the secret when refreshing.
    client_secret: Option<String>,
}

/// Maps cloud MCP installation UUID to its OAuth credentials in secure storage.
pub type PersistedCredentialsMap = HashMap<Uuid, PersistedCredentials>;

// Maps a consistent hash of the installation to its persisted credentials
pub type FileBasedPersistedCredentialsMap = HashMap<u64, PersistedCredentials>;

/// A credential store that wraps [`InMemoryCredentialStore`] and persists token
/// updates to Warp's secure storage via a channel.
///
/// When rmcp auto-refreshes an expired access token at runtime, the rotated
/// tokens are only saved to the in-memory store by default. This wrapper
/// ensures they also get written back to secure storage so they survive app
/// restarts.
struct PersistingCredentialStore {
    inner: InMemoryCredentialStore,
    client_secret: Option<String>,
    persist_tx: async_channel::Sender<PersistedCredentials>,
}

impl PersistingCredentialStore {
    /// Per RFC 6749 §6, the authorization server MAY issue a new refresh token on
    /// refresh, but is not required to. Many OAuth providers (e.g. Figma) only
    /// issue a refresh token on the initial authorization grant and omit it from
    /// subsequent refresh responses. If we blindly persist the new token response,
    /// the refresh token is lost and the next session (or next in-process refresh)
    /// requires a full re-auth.
    ///
    /// When the new response omits a refresh token, carry forward the one already
    /// in the store. See: <https://datatracker.ietf.org/doc/html/rfc6749#section-6>
    async fn apply_refresh_token_carry_forward(&self, credentials: &mut StoredCredentials) {
        if credentials
            .token_response
            .as_ref()
            .is_none_or(|tr| tr.refresh_token().is_some())
        {
            return;
        }

        if let Some(prev_rt) = self
            .inner
            .load()
            .await
            .ok()
            .and_then(|opt| opt)
            .and_then(|prev| prev.token_response)
            .and_then(|prev_tr| prev_tr.refresh_token().cloned())
        {
            if let Some(tr) = credentials.token_response.as_mut() {
                // Carry forward the existing/previous refresh token, constructing new if needed
                tr.set_refresh_token(Some(RefreshToken::new(prev_rt.secret().to_string())));
            }
        }
    }
}

#[async_trait::async_trait]
impl CredentialStore for PersistingCredentialStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        self.inner.load().await
    }

    async fn save(&self, mut credentials: StoredCredentials) -> Result<(), AuthError> {
        self.apply_refresh_token_carry_forward(&mut credentials)
            .await;

        self.inner.save(credentials.clone()).await?;

        // Only persist credentials if we actually have any.
        if credentials.token_response.is_some() {
            let _ = self.persist_tx.try_send(PersistedCredentials {
                credentials,
                client_secret: self.client_secret.clone(),
            });
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        self.inner.clear().await
    }
}

/// Installs a [`PersistingCredentialStore`] on the given auth manager so that
/// runtime token auto-refreshes are written back to Warp's secure storage.
///
/// A background tokio task is spawned to receive credential updates and persist
/// them via the [`ModelSpawner`]. The task terminates when the auth manager (and
/// thus the credential store's sender) is dropped.
///
/// Note: this store is not responsible for the initial population of credentials.
/// Instead, the caller seeds the inner store with any existing credentials prior
/// to installation (see [`install_persisting_credential_store`]). This store's
/// sole role is to write token updates back to secure storage as they occur.
async fn install_persisting_credential_store(
    auth_manager: &mut AuthorizationManager,
    persisted_credentials: Option<PersistedCredentials>,
    spawner: ModelSpawner<TemplatableMCPServerManager>,
    installation_uuid: Uuid,
) {
    let client_secret = persisted_credentials
        .as_ref()
        .and_then(|c| c.client_secret.clone());
    let in_memory_store = InMemoryCredentialStore::new();

    // If we have persisted credentials, populate the backing in-memory store with them.
    if let Some(credentials) = persisted_credentials {
        let _ = in_memory_store.save(credentials.credentials).await;
    }

    let (persist_tx, persist_rx) = async_channel::unbounded();
    let store = PersistingCredentialStore {
        inner: in_memory_store,
        client_secret,
        persist_tx,
    };

    auth_manager.set_credential_store(store);

    tokio::spawn(async move {
        while let Ok(credentials) = persist_rx.recv().await {
            if let Err(e) = spawner
                .spawn(move |manager, ctx| {
                    manager.save_credentials_to_secure_storage(ctx, installation_uuid, credentials);
                })
                .await
            {
                log::warn!("Failed to persist auto-refreshed MCP credentials: {e:?}");
            }
        }
    });
}

/// Context for OAuth authentication flows.
pub struct AuthContext {
    pub oauth_result_rx: async_channel::Receiver<CallbackResult>,
    pub spawner: ModelSpawner<TemplatableMCPServerManager>,
    pub uuid: Uuid,
    pub persisted_credentials: Option<PersistedCredentials>,
    /// Whether the client is running in headless/CLI mode.
    pub is_headless: bool,
    /// Whether this server was auto-discovered from a repo MCP configuration file.
    pub is_file_based: bool,
}

/// Result of OAuth callback.
#[derive(Debug, Clone)]
pub enum CallbackResult {
    Success { code: String, csrf_token: String },
    Error { error: Option<String> },
}

// 注释掉 make_authenticated_client 函数的实现 - 本地版本不支持 OAuth
// /// Makes an authenticated client for the given authorization server.
// ///
// /// This takes in the URL of the resource to authenticate for, and uses that
// /// to determine the authorization server.
// ///
// /// Upon success, returns the client and a boolean indicating whether the user was required to
// /// re-authenticate (e.g. re-log in).
// pub async fn make_authenticated_client(
//     resource_url: &str,
//     auth_context: AuthContext,
// ) -> Result<(AuthClient<reqwest::Client>, bool), AuthError> {
//     ... (详细实现被注释掉)
// }

// 本地版本的简化实现：不支持 OAuth 认证
pub async fn make_authenticated_client(
    _resource_url: &str,
    auth_context: AuthContext,
) -> Result<(AuthClient<reqwest::Client>, bool), AuthError> {
    // 如果是 headless 模式，直接返回错误
    if auth_context.is_headless {
        if auth_context.is_file_based {
            log::warn!(
                "File-based MCP server {} requires OAuth authentication; \
                 skipping in headless mode. To use this server, authenticate it \
                 in the Warp desktop app first.",
                auth_context.uuid
            );
        }
        return Err(AuthError::AuthorizationFailed(
            "MCP server requires OAuth authentication. Please authenticate this server in the \
             Warp desktop app first, then try again."
                .to_string(),
        ));
    }
    
    // 本地版本不支持 OAuth 认证
    Err(AuthError::AuthorizationFailed(
        "OAuth authentication is not supported in local version. Please use MCP servers without OAuth requirements."
            .to_string(),
    ))
}

impl TemplatableMCPServerManager {
    // 注释掉 OAuth 回调处理 - 本地版本不支持
    // /// Handles an incoming OAuth callback URL.
    // ///
    // /// Routes the callback to the correct in-flight OAuth flow using the `state` query
    // /// parameter (the CSRF token that rmcp embedded in the authorization URL). This avoids
    // /// encoding routing data in the redirect URI, keeping it RFC 6749 §3.1.2.2 compliant.
    // pub fn handle_oauth_callback(&mut self, url: &Url) -> anyhow::Result<()> {
    //     ... (详细实现被注释掉)
    // }
    
    // 本地版本的简化实现：不支持 OAuth 回调处理
    pub fn handle_oauth_callback(&mut self, _url: &Url) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("OAuth callback handling is not supported in local version"))
    }

    pub fn save_credentials_to_secure_storage(
        &mut self,
        app: &mut cuteui::AppContext,
        installation_uuid: Uuid,
        credentials: PersistedCredentials,
    ) {
        if let Some(hash) = FileBasedMCPManager::as_ref(app).get_hash_by_uuid(installation_uuid) {
            self.file_based_server_credentials.insert(hash, credentials);
            write_to_secure_storage(
                app,
                FILE_BASED_MCP_CREDENTIALS_KEY,
                &self.file_based_server_credentials,
            );
            return;
        }

        if let Some(template_uuid) = self.get_template_uuid(installation_uuid) {
            self.server_credentials.insert(template_uuid, credentials);
            write_to_secure_storage(
                app,
                TEMPLATABLE_MCP_CREDENTIALS_KEY,
                &self.server_credentials,
            );
        } else {
            log::error!(
                "Corresponding file or cloud-based server not found for installation UUID {installation_uuid}"
            );
        }
    }

    pub fn delete_credentials_from_secure_storage(
        &mut self,
        installation_uuid: Uuid,
        app: &mut cuteui::AppContext,
    ) {
        if let Some(template_uuid) = self.get_template_uuid(installation_uuid) {
            self.server_credentials.remove(&template_uuid);
            write_to_secure_storage(
                app,
                TEMPLATABLE_MCP_CREDENTIALS_KEY,
                &self.server_credentials,
            );
        } else {
            log::error!("No template UUID found for installation UUID {installation_uuid}");
        }
    }
}

/// Loads credentials from secure storage at the provided key.
pub(crate) fn load_credentials_from_secure_storage<T: DeserializeOwned + Default>(
    app: &mut cuteui::AppContext,
    key: &str,
) -> T {
    app.secure_storage()
        .read_value(key)
        .inspect_err(|err| {
            if !matches!(err, cuteui_extras::secure_storage::Error::NotFound) {
                log::warn!("Failed to read MCP credentials from secure storage: {err:#}");
            }
        })
        .ok()
        .and_then(|value| serde_json::from_str(&value).ok())
        .unwrap_or_default()
}

/// Writes credentials to secure storage at the provided key.
pub(crate) fn write_to_secure_storage<T: Serialize>(
    app: &mut cuteui::AppContext,
    key: &str,
    credentials: &T,
) {
    match serde_json::to_string(credentials) {
        Ok(json) => {
            app.secure_storage()
                .write_value(key, &json)
                .inspect_err(|err| {
                    log::error!("Failed to write MCP credentials to secure storage: {err:#}")
                })
                .ok();
        }
        Err(err) => {
            log::error!("Failed to serialize MCP credentials for secure storage: {err:#}");
        }
    }
}


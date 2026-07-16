#![allow(dead_code)]

use std::sync::Arc;

use cuteui::{Entity, ModelContext, SingletonEntity};

use super::auth_state::AuthState;
use super::AuthStateProvider;

pub struct AuthManager {
    auth_state: Arc<AuthState>,
}

impl AuthManager {
    pub fn new(
        _server_api: Arc<crate::server::server_api::ServerApi>,
        _auth_client: Arc<dyn crate::server::server_api::auth::AuthClient>,
        ctx: &mut ModelContext<Self>,
    ) -> Self {
        let auth_state = AuthStateProvider::as_ref(ctx).get().clone();

        Self {
            auth_state,
        }
    }

    #[cfg(test)]
    pub fn new_for_test(ctx: &mut ModelContext<Self>) -> Self {
        let auth_state = AuthStateProvider::as_ref(ctx).get().clone();

        Self {
            auth_state,
        }
    }

    pub fn set_needs_reauth(&self, needs_reauth: bool, _ctx: &mut ModelContext<Self>) {
        let became_true = self.auth_state.set_needs_reauth(needs_reauth);

        if became_true {
        }
    }

    pub fn set_user_onboarded(&self, _ctx: &mut ModelContext<Self>) {
        self.auth_state.set_is_onboarded(true);
    }

    pub(super) fn log_out(&mut self, _ctx: &mut ModelContext<Self>) {
        self.auth_state.set_user(None);
        self.auth_state.set_credentials(None);
    }

    pub fn initiate_anonymous_user_linking(&mut self, _entrypoint: crate::server::telemetry::AnonymousUserSignupEntrypoint, _ctx: &mut ModelContext<Self>) {}

    pub fn sign_up_url(&self) -> String {
        String::new()
    }

    pub fn sign_in_url(&self) -> String {
        String::new()
    }

    pub fn attempt_login_gated_feature(
        &mut self,
        _action: crate::workspace::WorkspaceAction,
        _auth_view_variant: super::AuthViewVariant,
        _ctx: &mut ModelContext<Self>,
    ) {
    }

    pub fn authorize_device(&mut self, ctx: &mut ModelContext<Self>) {
        // Cute: In skip_login mode, there is no device authorization flow.
        // Emit AuthComplete immediately so subscribers (launch_command, login, etc.)
        // can proceed without waiting for a server response that will never come.
        ctx.emit(AuthManagerEvent::AuthComplete);
    }

    pub fn refresh_user(&mut self, ctx: &mut ModelContext<Self>) {
        // Cute: In skip_login mode, there is no server to refresh from.
        // Emit AuthComplete immediately so subscribers (launch_command, etc.)
        // can proceed without waiting for a server response that will never come.
        ctx.emit(AuthManagerEvent::AuthComplete);
    }

    pub fn open_url_maybe_with_anonymous_token(
        &mut self,
        ctx: &mut ModelContext<Self>,
        url_factory: Box<dyn Fn(Option<&str>) -> String>,
    ) {
        let url = url_factory(None);
        ctx.open_url(&url);
    }
}

#[derive(Clone, Debug)]
pub struct PersistedCurrentUserInformation {
    pub email: String,
}

impl Entity for AuthManager {
    type Event = AuthManagerEvent;
}

impl SingletonEntity for AuthManager {}

#[derive(Debug, Clone)]
pub enum AuthManagerEvent {
    AuthComplete,
    SkippedLogin,
    AuthFailed(String),
    NeedsReauth,
    ReceivedDeviceAuthorizationCode {
        user_code: String,
        verification_uri: String,
    },
    AttemptedLoginGatedFeature {
        auth_view_variant: super::AuthViewVariant,
    },
    LoginOverrideDetected(super::AuthRedirectPayload),
}

#[derive(Debug, Clone, Default)]
pub struct LoginGatedFeature;

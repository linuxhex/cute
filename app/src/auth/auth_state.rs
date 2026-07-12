use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;
use uuid::Uuid;
use cuteui::{AppContext, Entity, SingletonEntity};

use super::anonymous_id::get_or_create_anonymous_id;
use super::credentials::Credentials;
use super::user::User;
use super::UserUid;

/// AuthState holds information about the currently-logged in user.
/// If you need to access AuthState, you can use the AuthStateProvider singleton model.
pub struct AuthState {
    user: RwLock<Option<User>>,
    anonymous_id: Uuid,
    needs_reauth: AtomicBool,
    credentials: RwLock<Option<Credentials>>,
}

impl AuthState {
    fn new(ctx: &AppContext) -> Self {
        Self {
            user: RwLock::new(None),
            anonymous_id: get_or_create_anonymous_id(ctx),
            needs_reauth: AtomicBool::new(false),
            credentials: RwLock::new(None),
        }
    }

    #[cfg(any(test, feature = "integration_tests"))]
    pub fn new_for_test() -> Self {
        Self {
            user: RwLock::new(Some(User::test())),
            anonymous_id: Uuid::new_v4(),
            needs_reauth: AtomicBool::new(false),
            credentials: RwLock::new(Some(Credentials::Test)),
        }
    }

    #[cfg(test)]
    pub fn new_logged_out_for_test() -> Self {
        Self {
            user: RwLock::new(None),
            anonymous_id: Uuid::new_v4(),
            needs_reauth: AtomicBool::new(false),
            credentials: RwLock::new(None),
        }
    }

    pub fn initialize(ctx: &AppContext, api_key: Option<String>) -> Self {
        let state = Self::new(ctx);

        if Self::should_use_test_user() {
            state.set_user(Some(User::test()));
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            state.set_credentials(Some(Credentials::Test));
            return state;
        }

        if let Some(api_key_value) = api_key {
            log::info!("Authenticating via API key");
            state.set_credentials(Some(Credentials::ApiKey {
                key: api_key_value,
                owner: None,
            }));
            return state;
        }

        state
    }

    fn should_use_test_user() -> bool {
        cfg!(any(test, feature = "skip_login"))
    }

    pub(super) fn set_user(&self, user: Option<User>) {
        *self.user.write() = user;
    }

    pub fn credentials(&self) -> Option<Credentials> {
        self.credentials.read().clone()
    }

    pub(super) fn set_credentials(&self, credentials: Option<Credentials>) {
        *self.credentials.write() = credentials;
    }

    // 注释掉云端认证相关方法 - 本地版本不需要
    // #[cfg(any(not(target_family = "wasm"), test))]
    // pub(crate) fn apply_remote_server_auth_context(
    //     &self,
    //     auth_token: String,
    //     user_id: String,
    //     user_email: String,
    // ) {
    //     self.set_remote_server_bearer_token(auth_token);
    //     self.set_remote_server_user(user_id, user_email);
    // }

    // #[cfg(any(not(target_family = "wasm"), test))]
    // pub(crate) fn set_remote_server_bearer_token(&self, auth_token: String) {
    //     if auth_token.is_empty() {
    //         self.set_credentials(None);
    //         return;
    //     }
    //     self.set_credentials(Some(Credentials::Bearer(auth_token)));
    // }

    // #[cfg(any(not(target_family = "wasm"), test))]
    // fn set_remote_server_user(&self, user_id: String, user_email: String) {
    //     let mut user = self.user.write();
    //     if user_id.is_empty() {
    //         *user = None;
    //         return;
    //     }

    //     match user.as_mut() {
    //         Some(user) => {
    //             user.local_id = UserUid::new(&user_id);
    //             user.metadata.email = user_email;
    //         }
    //         None => {
    //             *user = Some(User {
    //                 local_id: UserUid::new(&user_id),
    //                 metadata: Default::default(),
    //                 is_onboarded: false,
    //                 needs_sso_link: false,
    //                 anonymous_user_type: None,
    //                 is_on_work_domain: false,
    //                 linked_at: None,
    //                 personal_object_limits: None,
    //                 principal_type: Default::default(),
    //                 global_skills: Vec::new(),
    //             });
    //             user.as_mut().unwrap().metadata.email = user_email;
    //         }
    //     }
    // }

    // Cute: 本地版本始终处于已登录状态
    pub fn is_logged_in(&self) -> bool {
        true
    }

    // Cute: 本地版本始终不为匿名或未登录状态
    pub fn is_anonymous_or_logged_out(&self) -> bool {
        false
    }

    pub fn get_access_token_ignoring_validity(&self) -> Option<String> {
        let credentials = self.credentials.read();
        credentials.as_ref()?.bearer_token().bearer_token()
    }

    pub fn username_for_display(&self) -> Option<String> {
        Some(self.user.read().as_ref()?.username_for_display().to_owned())
    }

    pub fn display_name(&self) -> Option<String> {
        self.user
            .read()
            .as_ref()
            .and_then(|user| user.display_name().to_owned())
    }

    pub fn user_email(&self) -> Option<String> {
        self.user
            .read()
            .as_ref()
            .map(|user| user.metadata.email.clone())
    }

    pub fn is_onboarded(&self) -> Option<bool> {
        self.user.read().as_ref().map(|user| user.is_onboarded)
    }

    pub fn user_email_domain(&self) -> Option<String> {
        self.user.read().as_ref().map(|user| {
            user.metadata
                .email
                .clone()
                .split('@')
                .nth(1)
                .unwrap_or("")
                .to_string()
        })
    }

    pub fn is_user_anonymous(&self) -> Option<bool> {
        self.user
            .read()
            .as_ref()
            .map(|user| user.is_user_anonymous())
    }

    pub fn is_user_web_anonymous_user(&self) -> Option<bool> {
        self.user
            .read()
            .as_ref()
            .map(|user| {
                user.anonymous_user_type
                    .map(|t| t == super::user::AnonymousUserType::WebClientAnonymousUser)
                    .unwrap_or(false)
            })
    }

    pub fn is_anonymous_user_feature_gated(&self) -> Option<bool> {
        self.user
            .read()
            .as_ref()
            .and_then(|user| user.anonymous_user_type.map(|_| true))
    }

    pub fn user_photo_url(&self) -> Option<String> {
        self.user
            .read()
            .as_ref()
            .and_then(|user| user.metadata.photo_url.clone())
    }

    pub fn needs_sso_link(&self) -> Option<bool> {
        self.user.read().as_ref().map(|user| user.needs_sso_link)
    }

    pub fn personal_object_limits(&self) -> Option<crate::auth::user::PersonalObjectLimits> {
        self.user
            .read()
            .as_ref()
            .and_then(|user| user.personal_object_limits())
    }

    pub fn set_is_onboarded(&self, is_onboarded: bool) {
        if let Some(user) = self.user.write().as_mut() {
            user.is_onboarded = is_onboarded;
        }
    }

    pub fn user_id(&self) -> Option<UserUid> {
        self.user.read().as_ref().map(|user| user.local_id)
    }

    pub fn anonymous_id(&self) -> String {
        self.anonymous_id.to_string()
    }

    pub fn needs_reauth(&self) -> bool {
        self.needs_reauth.load(Ordering::Relaxed)
    }

    pub(super) fn set_needs_reauth(&self, new_needs_reauth: bool) -> bool {
        let prev_needs_reauth = self.needs_reauth.swap(new_needs_reauth, Ordering::Relaxed);
        !prev_needs_reauth && new_needs_reauth
    }

    pub fn is_on_work_domain(&self) -> Option<bool> {
        self.user.read().as_ref().map(|user| user.is_on_work_domain)
    }

    pub fn is_api_key_authenticated(&self) -> bool {
        matches!(
            self.credentials.read().as_ref(),
            Some(Credentials::ApiKey { .. })
        )
    }

    pub fn api_key(&self) -> Option<String> {
        let credentials = self.credentials.read();
        credentials.as_ref()?.as_api_key().map(|s| s.to_owned())
    }

    pub fn principal_type(&self) -> Option<crate::auth::user::PrincipalType> {
        self.user.read().as_ref().map(|user| user.principal_type)
    }

    pub fn is_service_account(&self) -> bool {
        matches!(self.principal_type(), Some(crate::auth::user::PrincipalType::ServiceAccount))
    }

    pub fn global_skills(&self) -> Vec<String> {
        self.user
            .read()
            .as_ref()
            .map(|user| user.global_skills.clone())
            .unwrap_or_default()
    }

    pub fn api_key_owner_type(&self) -> Option<cute_graphql::object_permissions::OwnerType> {
        self.credentials.read().as_ref()?.api_key_owner_type()
    }
}

impl cute_managed_secrets::ActorProvider for AuthState {
    fn actor_uid(&self) -> Option<String> {
        self.user_id().map(|uid| uid.as_string())
    }
}

pub struct AuthStateProvider {
    auth_state: Arc<AuthState>,
}

impl AuthStateProvider {
    pub fn new(auth_state: Arc<AuthState>) -> Self {
        Self { auth_state }
    }

    #[cfg(test)]
    pub fn new_for_test() -> Self {
        Self {
            auth_state: Arc::new(AuthState::new_for_test()),
        }
    }

    #[cfg(test)]
    pub fn new_logged_out_for_test() -> Self {
        Self {
            auth_state: Arc::new(AuthState::new_logged_out_for_test()),
        }
    }

    pub fn get(&self) -> &Arc<AuthState> {
        &self.auth_state
    }
}

impl Entity for AuthStateProvider {
    type Event = ();
}

impl SingletonEntity for AuthStateProvider {}

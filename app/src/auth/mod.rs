pub mod anonymous_id;
pub mod auth_manager;
pub mod auth_state;
pub mod credentials;
pub mod user;
pub mod user_uid;

pub use auth_manager::AuthManager;
pub use auth_state::AuthStateProvider;
pub use user_uid::UserUid;

// Cute: 本地版始终已登录，不需要登录/注册UI。
// 以下为原 auth_view_modal / auth_override_warning_modal 中的类型 stub，
// 仅保留被 auth_manager 等模块引用的类型定义。

#[derive(Debug, Clone, Default)]
pub enum AuthViewVariant {
    #[default]
    RequireLogin,
    #[allow(dead_code)]
    RequireLoginCloseable,
    #[allow(dead_code)]
    ShareRequirementCloseable,
}

#[derive(Debug, Clone, Default)]
pub struct AuthRedirectPayload;

use cuteui::{AppContext, SingletonEntity};

pub fn init(app: &mut AppContext) {
    // Cute: In skip_login mode, emit SkippedLogin AND AuthComplete so all
    // subscribers initialize correctly. Some subscribers (root_view onboarding)
    // listen for SkippedLogin; others (LLMPreferences, HarnessAvailabilityModel,
    // TeamUpdateManager) listen for AuthComplete to trigger model refreshes.
    // Without AuthComplete, AI/agent features including CLI agent integration
    // never load available models or harness secrets.
    AuthManager::handle(app).update(app, |_auth_manager, ctx| {
        ctx.emit(auth_manager::AuthManagerEvent::SkippedLogin);
        ctx.emit(auth_manager::AuthManagerEvent::AuthComplete);
    });
}

// Cute: 本地版始终已登录，不需要登出功能。
// maybe_log_out / log_out / remove_cloud_persisted_settings 已删除。

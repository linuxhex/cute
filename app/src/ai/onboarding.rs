//! Onboarding-specific AI types and conversions.

use ai::LLMId;
use onboarding::slides::OnboardingModelInfo;
use onboarding::OnboardingAuthState;
use cute_core::ui::icons::Icon;
use cuteui::{AppContext, SingletonEntity};

use super::llms::{DisableReason, LLMInfo, LLMPreferences};
use crate::auth::AuthStateProvider;

impl From<&LLMInfo> for OnboardingModelInfo {
    fn from(llm: &LLMInfo) -> Self {
        Self {
            id: llm.id.clone(),
            title: llm.display_name.clone(),
            icon: llm.provider.icon().unwrap_or(Icon::Oz),
            requires_upgrade: matches!(llm.disable_reason, Some(DisableReason::RequiresUpgrade)),
            is_default: false,
        }
    }
}

pub fn build_onboarding_models(
    prefs: &LLMPreferences,
    app: &AppContext,
) -> (Vec<OnboardingModelInfo>, LLMId) {
    let default_id = prefs.get_default_base_model().id.clone();
    let models: Vec<OnboardingModelInfo> = prefs
        .get_base_llm_choices_for_agent_mode(app)
        .map(|llm| {
            let mut info = OnboardingModelInfo::from(llm);
            info.is_default = info.id == default_id;
            info
        })
        .collect();
    (models, default_id)
}

pub fn current_onboarding_auth_state(_ctx: &AppContext) -> OnboardingAuthState {
    // Cute: 本地版本始终为付费用户状态（不需要云端认证）
    OnboardingAuthState::PayingUser
}

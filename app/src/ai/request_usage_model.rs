use std::sync::Arc;

use ai::api_keys::ApiKeyManager;
use chrono::{DateTime, Local, Utc};
use instant::Instant;
use serde::{Deserialize, Serialize};
use cute_core::user_preferences::GetUserPreferences as _;
pub use cute_graphql::billing::BonusGrantType;
use cute_graphql::scalars::time::ServerTimestamp;
use cuteui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::agent::AIAgentExchangeId;
// COMMENTED: Auth import disabled for local version
// use crate::auth::AuthStateProvider;
use crate::server::server_api::ai::AIClient;
use crate::settings::AISettings;
use crate::WorkspaceUid;
use crate::BlocklistAIHistoryModel;

/// Threshold of ambient-only credits at which we surface upgrade/CTA UI.
pub const AMBIENT_AGENT_TRIAL_CREDIT_THRESHOLD: i32 = 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BonusGrantScope {
    User,
    Workspace(WorkspaceUid),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Default)]
pub enum BuyCreditsBannerDisplayState {
    #[default]
    Hidden,
    OutOfCredits,
    MonthlyLimitReached,
}

#[derive(Clone, Debug)]
pub struct BonusGrant {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cost_cents: i32,
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub grant_type: BonusGrantType,
    pub reason: String,
    pub user_facing_message: Option<String>,
    pub request_credits_granted: i32,
    pub request_credits_remaining: i32,
    pub scope: BonusGrantScope,
}

/// The key for the corresponding entry in UserDefaults.
const REQUEST_LIMIT_INFO_CACHE_KEY: &str = "AIRequestLimitInfo";

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum RequestLimitRefreshDuration {
    Weekly,
    Monthly,
    EveryTwoWeeks,
}

/// The current rate limit info for the user.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct RequestLimitInfo {
    pub limit: usize,
    pub num_requests_used_since_refresh: usize,
    pub next_refresh_time: ServerTimestamp,
    pub is_unlimited: bool,
    pub request_limit_refresh_duration: RequestLimitRefreshDuration,
    pub is_unlimited_voice: bool,
    #[serde(default)]
    pub voice_request_limit: usize,
    #[serde(default)]
    pub voice_requests_used_since_last_refresh: usize,
    #[serde(default)]
    pub is_unlimited_codebase_indices: bool,
    #[serde(default)]
    pub max_codebase_indices: usize,
    #[serde(default)]
    pub max_files_per_repo: usize,
    #[serde(default)]
    pub embedding_generation_batch_size: usize,
}

fn default_voice_requests_limit() -> usize {
    10000
}

impl Default for RequestLimitInfo {
    /// This is the default rate limit for the free tier imposed by the server as of 02/10/25.
    fn default() -> Self {
        Self {
            limit: 150,
            num_requests_used_since_refresh: 0,
            next_refresh_time: ServerTimestamp::new(Utc::now() + chrono::Duration::days(30)),
            is_unlimited: false,
            request_limit_refresh_duration: RequestLimitRefreshDuration::Monthly,
            is_unlimited_voice: false,
            voice_request_limit: default_voice_requests_limit(),
            voice_requests_used_since_last_refresh: 0,
            is_unlimited_codebase_indices: false,
            max_codebase_indices: 3,
            max_files_per_repo: 5000,
            embedding_generation_batch_size: 100,
        }
    }
}

#[cfg(test)]
impl RequestLimitInfo {
    pub fn new_for_test(limit: usize, num_requests_used_since_refresh: usize) -> Self {
        Self {
            limit,
            num_requests_used_since_refresh,
            ..Self::default()
        }
    }
}

pub struct CodebaseContextUsageLimit {
    pub max_files_per_repo: usize,
    pub max_indices_allowed: Option<usize>,
    pub embedding_generation_batch_size: usize,
}

/// Contains all usage-related information fetched from the server.
pub struct RequestUsageInfo {
    pub request_limit_info: RequestLimitInfo,
    pub bonus_grants: Vec<BonusGrant>,
}

#[cfg(feature = "agent_mode_evals")]
impl RequestLimitInfo {
    pub fn new_for_evals() -> Self {
        Self {
            limit: 999999,
            num_requests_used_since_refresh: 0,
            next_refresh_time: ServerTimestamp::new(Utc::now() + chrono::Duration::days(30)),
            is_unlimited: true,
            request_limit_refresh_duration: RequestLimitRefreshDuration::Monthly,
            is_unlimited_voice: true,
            voice_request_limit: 999999,
            voice_requests_used_since_last_refresh: 0,
            is_unlimited_codebase_indices: false,
            max_codebase_indices: 40,
            max_files_per_repo: 10000,
            embedding_generation_batch_size: 100,
        }
    }
}

fn cache_request_limit_info(request_limit_info: RequestLimitInfo, app_mut: &mut AppContext) {
    if let Ok(serialized) = serde_json::to_string(&request_limit_info) {
        let _ = app_mut
            .private_user_preferences()
            .write_value(REQUEST_LIMIT_INFO_CACHE_KEY, serialized);
    }
}

fn get_cached_request_limit_info(app_mut: &mut AppContext) -> Option<RequestLimitInfo> {
    app_mut
        .private_user_preferences()
        .read_value(REQUEST_LIMIT_INFO_CACHE_KEY)
        .unwrap_or_default()
        .and_then(|serialized| serde_json::from_str(serialized.as_str()).ok())
}

pub struct AIRequestUsageModel {
    ai_client: Arc<dyn AIClient>,

    /// The last time at which `request_limit_info` was updated.
    last_update_time: Option<Instant>,

    request_limit_info: RequestLimitInfo,

    bonus_grants: Vec<BonusGrant>,

    /// Whether the buy credits banner has been dismissed by the user.
    buy_addon_credits_banner_dismissed: bool,
}

impl Entity for AIRequestUsageModel {
    type Event = AIRequestUsageModelEvent;
}

pub enum AIRequestUsageModelEvent {
    RequestUsageUpdated,
    RequestBonusRefunded {
        requests_refunded: i32,
        server_conversation_id: String,
        request_id: String,
    },
}

impl AIRequestUsageModel {
    pub fn new(ai_client: Arc<dyn AIClient>, ctx: &mut ModelContext<Self>) -> Self {
        // Check if the user has cached request limit info from before.
        // This is only used to show the latest known value before we finish refreshing from the server below.
        let cached_request_limit_info = get_cached_request_limit_info(ctx);
        let request_limit_info = cached_request_limit_info.unwrap_or_default();

        Self {
            ai_client,
            request_limit_info,
            last_update_time: None,
            bonus_grants: vec![],
            buy_addon_credits_banner_dismissed: false,
        }
    }

    #[cfg(test)]
    pub fn new_for_test(ai_client: Arc<dyn AIClient>, _ctx: &mut ModelContext<Self>) -> Self {
        Self {
            ai_client,
            last_update_time: None,
            request_limit_info: RequestLimitInfo::default(),
            bonus_grants: vec![],
            buy_addon_credits_banner_dismissed: false,
        }
    }

    pub fn last_update_time(&self) -> Option<Instant> {
        self.last_update_time
    }

    /// Simplified: Local version skips server refresh (no quota tracking).
    pub fn refresh_request_usage_async(&mut self, _ctx: &mut ModelContext<Self>) {
        // Skip refreshing from server - local version has no quota limits
        // The model is initialized with default values that indicate unlimited usage
    }

    pub fn update_request_limit_info(
        &mut self,
        request_limit_info: RequestLimitInfo,
        ctx: &mut ModelContext<Self>,
    ) {
        self.last_update_time = Some(Instant::now());
        self.request_limit_info = request_limit_info;
        cache_request_limit_info(request_limit_info, ctx);

        AISettings::handle(ctx).update(ctx, |ai_settings, ctx| {
            ai_settings.update_quota_info(&request_limit_info, ctx);
        });

        ctx.emit(AIRequestUsageModelEvent::RequestUsageUpdated);
    }

    pub fn provide_negative_feedback_response_for_ai_conversation(
        &mut self,
        client_conversation_id: AIConversationId,
        request_id: String,
        client_exchange_id: AIAgentExchangeId,
        ctx: &mut ModelContext<Self>,
    ) {
        let server_conversation_id = BlocklistAIHistoryModel::as_ref(ctx)
            .conversation(&client_conversation_id)
            .and_then(|conversation| conversation.server_conversation_token());

        let Some(server_conversation_id) = server_conversation_id else {
            return;
        };
        let server_conversation_id_string = server_conversation_id.as_str().to_string();
        let server_conversation_id_string_clone = server_conversation_id_string.clone();

        let request_ids = BlocklistAIHistoryModel::as_ref(ctx)
            .conversation(&client_conversation_id)
            .map(|conversation| {
                let mut request_ids = vec![];

                let target_exchange = conversation
                    .root_task_exchanges()
                    .find(|exchange| exchange.id == client_exchange_id);

                let mut found_target = false;

                for exchange in conversation.exchanges_reversed() {
                    if let Some(target_exchange) = target_exchange {
                        if exchange.id == target_exchange.id {
                            found_target = true;
                        }
                    } else {
                        break;
                    }

                    if found_target {
                        if let Some(server_output_id) = exchange.output_status.server_output_id() {
                            request_ids.push(server_output_id.to_string());
                        }

                        if exchange
                            .input
                            .iter()
                            .any(|input| input.user_query().is_some())
                        {
                            break;
                        }
                    }
                }

                request_ids
            })
            .unwrap_or_default();

        // No reason to refund if there are no request ids.
        if request_ids.is_empty() {
            return;
        }

        let ai_client = self.ai_client.clone();
        ctx.spawn(
            async move {
                ai_client
                    .provide_negative_feedback_response_for_ai_conversation(
                        server_conversation_id_string_clone,
                        request_ids,
                    )
                    .await
            },
            |_, result, ctx| match result {
                Ok(requests_refunded) => {
                    if requests_refunded > 0 {
                        ctx.emit(AIRequestUsageModelEvent::RequestBonusRefunded {
                            requests_refunded,
                            server_conversation_id: server_conversation_id_string,
                            request_id,
                        });
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to provide negative feedback response for ai conversation: {e:?}"
                    );
                }
            },
        );
    }

    /// Simplified: Local version always has requests remaining (no quota limits).
    fn requests_remaining(&self) -> usize {
        999999
    }

    /// Simplified: Local version always has requests remaining (no quota limits).
    pub fn has_requests_remaining(&self) -> bool {
        true
    }

    /// Simplified: Local version always has AI remaining (no quota limits).
    /// Always returns true to allow unlimited local AI usage.
    pub fn has_any_ai_remaining(&self, _ctx: &AppContext) -> bool {
        true
    }

    pub fn requests_used(&self) -> usize {
        if self.next_refresh_time() <= Utc::now() {
            return 0;
        }
        self.request_limit_info.num_requests_used_since_refresh
    }

    pub fn request_percentage_used(&self) -> f32 {
        self.requests_used() as f32 / self.request_limit() as f32
    }

    pub fn request_limit(&self) -> usize {
        self.request_limit_info.limit
    }

    /// Returns the number of indices the user's tier allows them to create and the number of files
    /// the user's tier allows them to index. If the user is allowed unlimited indices, then the
    /// max_indices_allowed is None.
    pub fn codebase_context_limits(&self) -> CodebaseContextUsageLimit {
        CodebaseContextUsageLimit {
            max_files_per_repo: self.request_limit_info.max_files_per_repo,
            max_indices_allowed: if self.request_limit_info.is_unlimited_codebase_indices {
                None
            } else {
                Some(self.request_limit_info.max_codebase_indices)
            },
            embedding_generation_batch_size: self
                .request_limit_info
                .embedding_generation_batch_size,
        }
    }

    /// Returns whether the user has hit their maximum codebase allowance.
    /// (If the user is allowed unlimited indices, this is vacuously false.)
    pub fn hit_codebase_index_limit(&self, current_indices: usize) -> bool {
        self.codebase_context_limits()
            .max_indices_allowed
            .map(|lim| current_indices >= lim)
            .unwrap_or(false)
    }

    pub fn next_refresh_time(&self) -> DateTime<Utc> {
        self.request_limit_info.next_refresh_time.utc()
    }

    /// Simplified: local version has no refresh time display
    #[allow(dead_code)]
    pub fn next_refresh_time_local(&self) -> DateTime<Local> {
        Local::now()
    }

    pub fn is_unlimited(&self) -> bool {
        self.request_limit_info.is_unlimited
    }

    pub fn refresh_duration_to_string(&self) -> String {
        match self.request_limit_info.request_limit_refresh_duration {
            RequestLimitRefreshDuration::Weekly => "weekly".to_string(),
            RequestLimitRefreshDuration::Monthly => "monthly".to_string(),
            RequestLimitRefreshDuration::EveryTwoWeeks => "biweekly".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn bonus_grants(&self) -> &[BonusGrant] {
        &self.bonus_grants
    }

    /// Returns the total remaining ambient-only credits for the user.
    /// Returns None if the user has never received any ambient-only grants.
    pub fn ambient_only_credits_remaining(&self) -> Option<i32> {
        let ambient_grants: Vec<_> = self
            .bonus_grants
            .iter()
            .filter(|g| g.grant_type == BonusGrantType::AmbientOnly)
            .collect();
        if ambient_grants.is_empty() {
            None
        } else {
            Some(
                ambient_grants
                    .iter()
                    .map(|g| g.request_credits_remaining)
                    .sum(),
            )
        }
    }

    pub fn total_workspace_bonus_credits_remaining(&self, uid: WorkspaceUid) -> i32 {
        let now = Utc::now();
        let uid_ref = &uid;  // Use reference instead of moving uid
        self.bonus_grants
            .iter()
            .filter(|grant| grant.scope == BonusGrantScope::Workspace(uid_ref.clone()))
            .filter(|grant| grant.expiration.is_none_or(|exp| now < exp))
            .map(|grant| grant.request_credits_remaining)
            .sum()
    }

    #[allow(dead_code)]
    pub fn total_current_workspace_bonus_credits_remaining(&self, ctx: &AppContext) -> i32 {
        // COMMENTED: UserWorkspaces disabled in local version - 注释掉云端工作空间/团队功能 - 本地版本不支持
        0 // UserWorkspaces::as_ref(ctx)
        //     .current_workspace()
        //     .map(|workspace| self.total_workspace_bonus_credits_remaining(workspace.uid))
        //     .unwrap_or(0)
    }

    pub fn total_user_interactive_bonus_credits_remaining(&self) -> i32 {
        let now = Utc::now();
        self.bonus_grants
            .iter()
            .filter(|grant| grant.scope == BonusGrantScope::User)
            .filter(|grant| grant.grant_type != BonusGrantType::AmbientOnly)
            .filter(|grant| grant.expiration.is_none_or(|exp| now < exp))
            .map(|grant| grant.request_credits_remaining)
            .sum()
    }

    /// Simplified: local version has no add-on credits purchase
    #[allow(dead_code)]
    pub fn compute_buy_addon_credits_banner_display_state(
        &self,
        _ctx: &AppContext,
    ) -> BuyCreditsBannerDisplayState {
        BuyCreditsBannerDisplayState::Hidden
    }

    #[allow(dead_code)]
    pub fn dismiss_buy_credits_banner(&mut self, ctx: &mut ModelContext<Self>) {
        self.buy_addon_credits_banner_dismissed = true;
        ctx.notify();
    }

    pub fn enable_buy_credits_banner(&mut self, ctx: &mut ModelContext<Self>) {
        self.buy_addon_credits_banner_dismissed = false;
        ctx.notify();
    }
}

/// Voice request usage, only available if built with voice input support.
#[cfg(feature = "voice_input")]
impl AIRequestUsageModel {
    fn voice_requests(&self) -> usize {
        0 // Simplified: local version has no voice usage tracking
    }

    fn voice_requests_limit(&self) -> usize {
        999999 // Simplified: local version has unlimited voice requests
    }

    fn is_unlimited_voice_requests(&self) -> bool {
        true // Simplified: local version always unlimited
    }

    /// Simplified: Local version always has voice requests remaining (no quota limits).
    fn voice_requests_remaining(&self) -> usize {
        999999
    }

    /// Simplified: Local version always has voice requests remaining (no quota limits).
    fn has_voice_requests_remaining(&self) -> bool {
        true
    }

    /// Simplified: Local version can always request voice (no quota limits).
    pub fn can_request_voice(&self) -> bool {
        true
    }
}

impl SingletonEntity for AIRequestUsageModel {}

#[cfg(test)]
#[path = "request_usage_model_tests.rs"]
mod tests;

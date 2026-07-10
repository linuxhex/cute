// 已注释：清理 shared_session 共享会话功能
// 原始文件已简化，保留文件以避免编译错误

use std::time::Duration;

use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, Setting, SupportedPlatforms, SyncToCloud};

// 简化的 SharedSessionSettings，功能已禁用
define_settings_group!(SharedSessionSettings, settings: [
    onboarding_block_shown: SessionSharingOnboardingBlockShown {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: true,
    },
]);

impl SharedSessionSettings {
    /// Returns time between showing the inactivity warning modal and ending the session.
    pub fn inactivity_period_between_warning_and_ending_session(&self) -> Duration {
        Duration::from_secs(0)  // 返回 0，禁用该功能
    }
}
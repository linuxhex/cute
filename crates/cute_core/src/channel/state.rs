use std::borrow::Cow;
use std::collections::HashSet;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use url::Url;

use super::Channel;
use crate::channel::config::{
    ChannelConfig, IapConfig, McpOAuthProviderConfig, OzConfig, RudderStackDestination,
    WarpServerConfig,
};
use crate::features::FeatureFlag;
use crate::AppId;

lazy_static! {
    static ref CHANNEL_STATE: Mutex<ChannelState> = Mutex::new(ChannelState::init());
}

#[cfg(feature = "test-util")]
lazy_static! {
    static ref MOCK_SERVER: mockito::ServerGuard = mockito::Server::new();
    static ref MOCK_SERVER_URL: String = MOCK_SERVER.url();
    static ref APP_VERSION: Mutex<Option<&'static str>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct ChannelState {
    channel: Channel,

    /// The set of additional features to enable (on top of default-enabled ones).
    additional_features: HashSet<FeatureFlag>,

    config: ChannelConfig,
}

impl ChannelState {
    pub fn init() -> Self {
        let channel = Channel::Oss;
        let app_id = AppId::new("dev", "cute", "Cute");
        Self {
            channel,
            additional_features: Default::default(),
            config: ChannelConfig {
                app_id,
                logfile_name: "".into(),
                server_config: WarpServerConfig::production(),
                oz_config: OzConfig::production(),
                telemetry_config: None,
                autoupdate_config: None,
                crash_reporting_config: None,
                mcp_static_config: None,
            },
        }
    }

    pub fn new(channel: Channel, mut config: ChannelConfig) -> Self {
        if let Some(app_id) = app_id_from_bundle() {
            config.app_id = app_id;
        }
        Self {
            channel,
            additional_features: Default::default(),
            config,
        }
    }

    pub fn with_additional_features(mut self, overrides: &[FeatureFlag]) -> Self {
        self.additional_features.extend(overrides);
        self
    }

    pub fn set(state: ChannelState) {
        *CHANNEL_STATE.lock() = state;
    }

    pub fn is_release_bundle() -> bool {
        cfg!(feature = "release_bundle")
    }

    pub fn enable_debug_features() -> bool {
        cfg!(debug_assertions) || matches!(Self::channel(), Channel::Local | Channel::Dev)
    }



    /// Returns the canonical identifier for the application.
    ///
    /// This should not be used for namespacing persisted data - such use cases
    /// should make use of [`Self::data_domain`] instead.
    pub fn app_id() -> AppId {
        CHANNEL_STATE.lock().config.app_id.clone()
    }

    /// Returns a profile name for isolating user data. This should be used to
    /// sandbox how user data is stored.
    ///
    /// This is a debugging tool for isolating development instances of Warp, and is not
    /// supported in release builds.
    pub fn data_profile() -> Option<String> {
        if cfg!(debug_assertions) {
            std::env::var("CUTE_DATA_PROFILE").ok()
        } else {
            None
        }
    }

    /// Returns a value that should be used for namespacing persisted data.
    ///
    /// In release builds, this is identical to the app ID; in debug builds,
    /// it optionally includes a suffix derived from the `CUTE_DATA_PROFILE`
    /// environment variable.
    pub fn data_domain() -> String {
        match Self::data_profile() {
            Some(profile) => format!("{}-{profile}", Self::app_id()),
            None => Self::app_id().to_string(),
        }
    }

    /// Returns the data domain if overridden from the default, otherwise None.
    pub fn data_domain_if_not_default() -> Option<String> {
        Self::data_profile().map(|_| Self::data_domain())
    }

    pub fn additional_features() -> HashSet<FeatureFlag> {
        CHANNEL_STATE
            .lock()
            .additional_features
            .iter()
            .cloned()
            .collect()
    }

    pub fn debug_str() -> String {
        format!("{:?}", *CHANNEL_STATE.lock())
    }

    pub fn logfile_name() -> Cow<'static, str> {
        CHANNEL_STATE.lock().config.logfile_name.clone()
    }

    pub fn telemetry_file_name() -> Cow<'static, str> {
        // Simplified: OSS version does not have telemetry
        Cow::Borrowed("")
    }

    /// Returns whether this build has a telemetry config and can therefore ship
    /// telemetry events. OSS builds ship with `telemetry_config: None`.
    pub fn is_telemetry_available() -> bool {
        false
    }

    /// Returns whether this build has a crash reporting config and can therefore
    /// ship crash reports. OSS builds ship with `crash_reporting_config: None`.
    pub fn is_crash_reporting_available() -> bool {
        false
    }

    pub fn releases_base_url() -> Cow<'static, str> {
        CHANNEL_STATE
            .lock()
            .config
            .autoupdate_config
            .as_ref()
            .map(|ac| ac.releases_base_url.clone())
            .unwrap_or_default()
    }

    pub fn iap_config() -> Option<IapConfig> {
        CHANNEL_STATE.lock().config.server_config.iap_config.clone()
    }

    /// Returns whether this build uses a staging server.
    /// For OSS builds, this always returns false.
    pub fn uses_staging_server() -> bool {
        false
    }

    /// Returns the server root URL for API calls.
    /// For OSS builds without cloud services, returns empty string.
    pub fn server_root_url() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    /// Returns the WebSocket server URL for RTC connections.
    /// For OSS builds without cloud services, returns empty string.
    pub fn ws_server_url() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    /// Returns the session sharing server URL.
    /// For OSS builds without cloud services, returns None.
    pub fn session_sharing_server_url() -> Option<Cow<'static, str>> {
        None
    }

    /// Returns the Oz dashboard URL.
    /// For OSS builds without cloud services, returns empty string.
    pub fn oz_root_url() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    /// Returns the workload audience URL.
    /// For OSS builds without cloud services, returns empty string.
    pub fn workload_audience_url() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    /// Returns the RTC HTTP URL.
    /// For OSS builds without cloud services, returns empty string.
    pub fn rtc_http_url() -> Cow<'static, str> {
        Cow::Borrowed("")
    }

    /// Override the server root URL.
    /// For OSS builds, this does nothing as URL overrides are not allowed.
    pub fn override_server_root_url(_url: impl Into<Cow<'static, str>>) -> Result<(), url::ParseError> {
        // OSS builds don't allow URL overrides, so this is a no-op stub
        Ok(())
    }

    /// Override the WebSocket server URL.
    /// For OSS builds, this does nothing as URL overrides are not allowed.
    pub fn override_ws_server_url(_url: impl Into<Cow<'static, str>>) -> Result<(), url::ParseError> {
        // OSS builds don't allow URL overrides, so this is a no-op stub
        Ok(())
    }

    /// Override the session sharing server URL.
    /// For OSS builds, this does nothing as URL overrides are not allowed.
    pub fn override_session_sharing_server_url(_url: impl Into<Cow<'static, str>>) -> Result<(), url::ParseError> {
        // OSS builds don't allow URL overrides, so this is a no-op stub
        Ok(())
    }

    /// Returns the server root domain origin.
    /// For OSS builds without cloud services, returns an opaque origin.
    pub fn server_root_domain() -> url::Origin {
        // Return an opaque origin for local/OSS builds
        Url::parse("about:blank").unwrap().origin()
    }

    /// Returns the rudderstack destination for all events that don't contain user-generated content.
    /// Simplified: OSS version does not use RudderStack.
    pub fn rudderstack_non_ugc_destination() -> RudderStackDestination {
        RudderStackDestination::default()
    }

    /// Returns the rudderstack destination for all events that contain user-generated content.
    /// Simplified: OSS version does not use RudderStack.
    pub fn rudderstack_ugc_destination() -> RudderStackDestination {
        RudderStackDestination::default()
    }

    pub fn channel() -> Channel {
        CHANNEL_STATE.lock().channel
    }

    #[cfg(feature = "test-util")]
    pub fn app_version() -> Option<&'static str> {
        let version = APP_VERSION.lock();

        version.or_else(|| option_env!("GIT_RELEASE_TAG"))
    }

    #[cfg(feature = "test-util")]
    pub fn set_app_version(version: Option<&'static str>) {
        *APP_VERSION.lock() = version;
    }

    #[cfg(not(feature = "test-util"))]
    pub fn app_version() -> Option<&'static str> {
        option_env!("GIT_RELEASE_TAG")
    }

    pub fn sentry_url() -> Cow<'static, str> {
        // Simplified: OSS version does not use Sentry for crash reporting
        Cow::Borrowed("")
    }

    pub fn show_autoupdate_menu_items() -> bool {
        CHANNEL_STATE
            .lock()
            .config
            .autoupdate_config
            .as_ref()
            .map(|ac| ac.show_autoupdate_menu_items)
            .unwrap_or_default()
    }

    /// Returns the MCP OAuth provider config matching the given client ID, if any.
    pub fn mcp_oauth_provider_by_client_id(client_id: &str) -> Option<McpOAuthProviderConfig> {
        CHANNEL_STATE
            .lock()
            .config
            .mcp_static_config
            .as_ref()
            .and_then(|c| c.providers.iter().find(|p| p.client_id == client_id))
            .cloned()
    }

    pub fn url_scheme() -> &'static str {
        match Self::channel() {
            Channel::Stable => "warp",
            Channel::Preview => "warppreview",
            Channel::Dev => "warpdev",
            // Dummy value--integration tests shouldn't support URL schemes.
            Channel::Integration => "warpintegration",
            Channel::Local => "warplocal",
            Channel::Oss => "cute",
        }
    }
}



#[cfg(all(test, not(feature = "test-util")))]
#[path = "state_tests.rs"]
mod tests;

fn app_id_from_bundle() -> Option<AppId> {
    // On macOS, attempt to determine the app ID from the containing bundle,
    // falling back to the channel-keyed "default" ID if we cannot retrieve
    // bundle information.
    //
    // We skip this for tests, as the call to `mainBundle` can take 30+ms,
    // which is a significant portion of the total test runtime.
    #[cfg(all(target_os = "macos", not(feature = "test-util")))]
    {
        use objc2_foundation::NSBundle;

        let bundle = NSBundle::mainBundle();
        if let Some(bundle_identifier) = bundle.bundleIdentifier() {
            let app_id = bundle_identifier.to_string();
            if !app_id.is_empty() {
                return Some(
                    AppId::parse(&app_id)
                        .expect("macOS bundle identifier has an unexpected format"),
                );
            }
        }
    }

    None
}

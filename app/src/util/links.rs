use crate::channel::ChannelState;

pub const USER_DOCS_URL: &str = "https://github.com/linuxhex/cute";
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub const GITHUB_ISSUES_URL: &str = "https://github.com/linuxhex/cute/issues";
pub const SLACK_URL: &str = "https://github.com/linuxhex/cute/discussions";
pub const PRIVACY_POLICY_URL: &str = "https://github.com/linuxhex/cute#privacy";

pub fn feedback_form_url() -> String {
    let mut url = url::Url::parse("https://github.com/linuxhex/cute/issues/new/choose")
        .expect("Should not fail to parse");
    if let Some(version) = ChannelState::app_version() {
        url.query_pairs_mut().append_pair("cute-version", version);
    }
    url.query_pairs_mut()
        .append_pair("os-version", &os_info::get().version().to_string());
    url.to_string()
}

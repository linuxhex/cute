//! This module should houses all horizontal/cross-cutting AI functionality throughout
//! Warp (including Agent Mode).
//!
//! The side panel Warp AI implementation lives in `super::ai_assistant`.
pub(crate) mod active_agent_views_model;
pub(crate) mod agent;
pub(crate) mod agent_conversations_model;
pub(crate) mod agent_events;
pub(crate) mod agent_management;
pub(crate) mod agent_tips;
pub(crate) mod ai_document_view;
pub mod ambient_agent_types;
pub(crate) mod artifact_download;
pub mod artifacts;
pub(crate) mod attachment_utils;
pub mod auth_secret_types;
#[cfg(not(target_family = "wasm"))]
pub mod aws_credentials;
pub(crate) mod block_context;
pub(crate) mod blocklist;
pub(crate) mod cloud_environments;
pub(crate) mod execution_profiles;
pub(crate) mod facts;
#[cfg(any(feature = "local_fs", not(target_family = "wasm")))]
pub(crate) mod codebase_auto_indexing;
pub mod control_code_parser;
pub(crate) mod conversation_details_panel;
pub(crate) mod conversation_navigation;
pub(crate) mod conversation_status_ui;
pub(crate) mod conversation_utils;
pub(crate) mod document;
pub(crate) mod get_relevant_files;
pub mod harness_availability;
pub(crate) mod harness_display;
pub(crate) mod llms;
pub(crate) mod local_harness_setup;
pub mod onboarding;
pub(crate) mod persisted_workspace;
pub(crate) mod predict;
pub mod request_usage_model;
pub(crate) mod restored_conversations;
pub(crate) mod skills;
pub(crate) mod voice;
pub use agent_tips::*;
pub use request_usage_model::*;
use cuteui::AppContext;
#[cfg(not(target_family = "wasm"))]
pub mod agent_sdk;
// pub mod cloud_agent_settings; // Cute: 已注释，清理云端 Agent 设置模块
pub mod connected_self_hosted_workers;
pub(crate) mod generate_block_title;
pub(crate) mod generate_code_review_content;
pub(crate) mod loading;
pub mod mcp;
pub mod outline;

pub(crate) use ai::paths;

pub fn init(app: &mut AppContext) {
    blocklist::keyboard_navigable_buttons::init(app);
    blocklist::block::number_shortcut_buttons::init(app);
    blocklist::toggleable_items::init(app);
    blocklist::suggested_agent_mode_workflow_modal::init(app);
    blocklist::suggested_rule_modal::init(app);
    ai_document_view::init(app);
    conversation_details_panel::init(app);
    // COMMENTED: Cloud agent management view disabled for local version
    // #[cfg(feature = "agent_management_view")]
    // agent_management::init(app);
}

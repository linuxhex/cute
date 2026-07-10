pub mod anonymous_id;
pub mod auth_manager;
pub mod auth_override_warning_modal;
pub mod auth_state;
pub mod auth_view_modal;
pub mod credentials;
pub mod user;
pub mod user_uid;

pub use auth_manager::AuthManager;
pub use auth_state::AuthStateProvider;
pub use auth_view_modal::AuthRedirectPayload;
pub use user_uid::UserUid;

use ai::index::full_source_code_embedding::manager::CodebaseIndexManager;
use itertools::Itertools;
use cuteui::{AppContext, SingletonEntity};

use crate::ai::agent_conversations_model::AgentConversationsModel;
use crate::ai::blocklist::agent_view::orchestration_pill_bar_model::OrchestrationPillBarModel;
use crate::ai::blocklist::BlocklistAIHistoryModel;
use crate::ai::execution_profiles::profiles::AIExecutionProfilesModel;
use crate::env_vars::manager::EnvVarCollectionManager;
use crate::cloud_stub_types::NotebookManager;
use crate::session_management::{RunningSessionSummary, SessionNavigationData};
use crate::settings::{
    PrivacySettings, CRASH_REPORTING_ENABLED_DEFAULTS_KEY,
    TELEMETRY_ENABLED_DEFAULTS_KEY,
};
use crate::terminal::general_settings::GeneralSettings;
use crate::workflows::manager::WorkflowManager;
use crate::{persistence, report_if_error, GlobalResourceHandlesProvider};
use cute_core::user_preferences::GetUserPreferences as _;
use settings::Setting;
use settings::ToggleableSetting;

pub fn init(app: &mut AppContext) {
    // Cute: In skip_login mode, emit SkippedLogin AND AuthComplete so all
    // subscribers initialize correctly. Some subscribers (root_view onboarding)
    // listen for SkippedLogin; others (LLMPreferences, HarnessAvailabilityModel,
    // TeamUpdateManager) listen for AuthComplete to trigger model refreshes.
    // Without AuthComplete, AI/agent features including CLI agent integration
    // never load available models or harness secrets.
    #[cfg(feature = "skip_login")]
    {
        AuthManager::handle(app).update(app, |_auth_manager, ctx| {
            ctx.emit(auth_manager::AuthManagerEvent::SkippedLogin);
            ctx.emit(auth_manager::AuthManagerEvent::AuthComplete);
        });
    }
    let _ = app; // suppress unused warning when skip_login is not enabled
}

pub fn maybe_log_out(app: &mut AppContext) {
    let sessions = SessionNavigationData::all_sessions(app).collect_vec();
    let num_long_running_commands = RunningSessionSummary::new(&sessions)
        .long_running_cmds
        .len();
    let code_editors = crate::code::editor_management::CodeEditorStatus::all_editors(app).collect_vec();
    let code_editor_summary = crate::code::editor_management::CodeEditorSummary::new(&code_editors);
    let num_unsaved_files = code_editor_summary.unsaved_changes.len();

    let show_warning_before_log_out = *GeneralSettings::as_ref(app)
        .show_warning_before_quitting
        .value();
    if show_warning_before_log_out
        && (num_long_running_commands > 0
            || num_unsaved_files > 0)
    {
        let mut button_data = vec![cuteui::modals::ModalButton::for_app("Yes, log out", |ctx| {
            log_out(ctx);
        })];

        let mut info_text_vec: Vec<String> = vec![];
        if num_long_running_commands > 0 {
            let plural = if num_long_running_commands > 1 {
                "processes"
            } else {
                "process"
            };
            info_text_vec.push(format!(
                "You have {num_long_running_commands} {plural} running."
            ));

            button_data.push(cuteui::modals::ModalButton::for_app("Show running processes", move |ctx| {
                let windowing_model = ctx.windows();
                let window_id = if let Some(active_window_id) = windowing_model.active_window() {
                    active_window_id
                } else if let Some(window_id) = ctx.window_ids().collect_vec().first() {
                    let window_id = *window_id;
                    windowing_model.show_window_and_focus_app(window_id);
                    window_id
                } else {
                    return;
                };

                if let Some(workspaces) = ctx.views_of_type::<crate::workspace::Workspace>(window_id) {
                    if let Some(handle) = workspaces.first() {
                        ctx.dispatch_typed_action_for_view(
                            window_id,
                            handle.id(),
                            &crate::workspace::WorkspaceAction::OpenPalette {
                                mode: crate::palette::PaletteMode::Navigation,
                                source: crate::server::telemetry::PaletteSource::LogOutModal,
                                query: Some("running".to_owned()),
                            },
                        );
                    }
                }
            }))
        }

        if num_unsaved_files > 0 {
            let plural = if num_unsaved_files > 1 {
                "files"
            } else {
                "file"
            };
            info_text_vec.push(format!(
                "You have {num_unsaved_files} unsaved {plural}. \
            Logging out will cause you to lose the {plural}."
            ));
        }

        button_data.push(cuteui::modals::ModalButton::for_app("Cancel", move |_ctx| {
        }));

        let alert_data = cuteui::modals::AlertDialogWithCallbacks::for_app(
            "Log out?",
            info_text_vec.join("\n"),
            button_data,
            move |ctx| {
                GeneralSettings::handle(ctx).update(ctx, |general_settings, ctx| {
                    report_if_error!(general_settings
                        .show_warning_before_quitting
                        .toggle_and_save_value(ctx));
                });
            },
        );

        if cfg!(all(not(target_family = "wasm"), target_os = "macos")) {
            app.show_native_platform_modal(alert_data);
        } else {
            let sessions = SessionNavigationData::all_sessions(app).collect_vec();
            let sessions_summary = RunningSessionSummary::new(&sessions);
            crate::focus_running_window_and_show_native_modal(sessions_summary, alert_data, app);
        }
    } else {
        log_out(app);
    }
}

pub fn log_out(app: &mut AppContext) {
    CodebaseIndexManager::handle(app).update(app, |index_manager, ctx| {
        index_manager.reset_codebase_indexing(ctx);
    });

    let global_resource_handles = GlobalResourceHandlesProvider::as_ref(app).get();

    persistence::remove(&global_resource_handles.model_event_sender);

    AuthManager::handle(app).update(app, |auth_manager, ctx| {
        auth_manager.log_out(ctx);
    });
    AIExecutionProfilesModel::handle(app).update(app, |ai_execution_profiles_model, _| {
        ai_execution_profiles_model.reset();
    });
    BlocklistAIHistoryModel::handle(app).update(app, |history_model, _| {
        history_model.reset();
    });
    OrchestrationPillBarModel::handle(app).update(app, |pill_bar_model, _| {
        pill_bar_model.reset();
    });
    AgentConversationsModel::handle(app).update(app, |agent_conversations_model, _| {
        agent_conversations_model.reset();
    });

    // COMMENTED: TeamUpdateManager disabled
    // crate::workspaces::update_manager::TeamUpdateManager::handle(app).update(app, |manager, _| {
    //     manager.stop_polling_for_workspace_metadata_updates();
    // });
    remove_cloud_persisted_settings(app);
    NotebookManager::handle(app).update(app, |manager, app| manager.reset(app));
    EnvVarCollectionManager::handle(app).update(app, |manager, _| manager.reset());
    WorkflowManager::handle(app).update(app, |manager, _| manager.reset());

    let window_ids = app.window_ids().collect_vec();
    for window_id in window_ids {
        if let Some(root_view_id) = app.root_view_id(window_id) {
            app.dispatch_action(
                window_id,
                &[root_view_id],
                "root_view:log_out",
                &(),
                log::Level::Info,
            );
        }
    }

    #[cfg(target_family = "wasm")]
    crate::platform::wasm::emit_event(crate::platform::wasm::WarpEvent::LoggedOut);
}

fn remove_cloud_persisted_settings(app: &mut AppContext) {
    if let Err(e) = app
        .private_user_preferences()
        .remove_value(TELEMETRY_ENABLED_DEFAULTS_KEY)
    {
        log::error!("Failed to remove Telemetry Enabled Defaults Key from user defaults: {e:?}");
    }

    if let Err(e) = app
        .private_user_preferences()
        .remove_value(CRASH_REPORTING_ENABLED_DEFAULTS_KEY)
    {
        log::error!(
            "Failed to remove Crash Reporting Enabled Defaults Key from user defaults: {e:?}"
        );
    }

    if let Err(e) = app
        .private_user_preferences()
        .remove_value(crate::ai_assistant::requests::REQUEST_LIMIT_INFO_CACHE_KEY)
    {
        log::error!("Failed to remove Request Limit Defaults Key from user defaults: {e:?}");
    }

    PrivacySettings::handle(app).update(app, |privacy_settings, _| {
        privacy_settings.refresh_to_default();
    });
}

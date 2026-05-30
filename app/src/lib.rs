// Suppress warnings about rustdoc style.
#![allow(clippy::doc_lazy_continuation)]

mod alloc;
mod git;
#[cfg(target_os = "macos")]
mod app;
mod app_services;
mod app_state;
mod command_palette;
mod debounce;
mod diff_panel;
#[cfg(windows)]
mod dynamic_libraries;
mod features;
mod file_explorer;
mod gpu_state;
mod input_classifier;
mod interval_timer;
mod network;
mod platform;
mod prefix;
#[cfg(target_os = "macos")]
mod preview_config_migration;
mod profiling;
mod resource_limits;
mod safe_triangle;
mod server;
mod settings;
#[cfg(test)]
mod test_util;
mod throttle;
mod tracing;
mod window_settings;

pub mod channel;
pub mod terminal;

use cute_cli::{CliCommand, GlobalOptions};

use std::borrow::Cow;
use std::sync::Arc;

use ::settings::{Setting, ToggleableSetting};
use anyhow::{anyhow, Result};
use channel::ChannelState;
use cute_core::execution_mode::{AppExecutionMode, ExecutionMode};
pub use cute_core::send_telemetry_from_app_ctx;
pub use cute_core::send_telemetry_from_ctx;
pub use cute_core::{safe_debug, safe_error, safe_info, safe_warn};
use cute_logging::LogDestination;
use cuteui::integration::TestDriver;
use cuteui::{App, AppContext, Event, WindowId};

use crate::app_state::AppState;
use crate::gpu_state::GPUState;
use crate::network::NetworkStatus;

/// Our embedded application assets.
pub static ASSETS: cute_assets::Assets = cute_assets::Assets;

/// Launch mode for how to start up Cute.
#[allow(clippy::large_enum_variant)]
pub enum LaunchMode {
    /// Run the regular GUI application.
    App {
        args: cute_cli::AppArgs,
        api_key: Option<String>,
    },

    /// Run the Cute command-line SDK.
    CommandLine {
        command: cute_cli::CliCommand,
        global_options: GlobalOptions,
        debug: bool,
        is_sandboxed: bool,
        computer_use_override: Option<bool>,
    },
    /// Run a test - this may be an integration test or an eval.
    Test {
        driver: Box<Option<TestDriver>>,
        is_integration_test: bool,
    },
}

impl LaunchMode {
    fn args(&self) -> Cow<'_, cute_cli::AppArgs> {
        match self {
            LaunchMode::App { args, .. } => Cow::Borrowed(args),
            LaunchMode::CommandLine { .. } | LaunchMode::Test { .. } => {
                Cow::Owned(cute_cli::AppArgs::default())
            }
        }
    }

    fn is_integration_test(&self) -> bool {
        match self {
            LaunchMode::Test {
                is_integration_test,
                ..
            } => *is_integration_test,
            LaunchMode::App { .. } | LaunchMode::CommandLine { .. } => false,
        }
    }

    fn take_test_driver(&mut self) -> Option<TestDriver> {
        match self {
            LaunchMode::Test { driver, .. } => driver.take(),
            LaunchMode::App { .. } | LaunchMode::CommandLine { .. } => None,
        }
    }

    fn execution_mode(&self) -> ExecutionMode {
        match self {
            LaunchMode::App { .. } => ExecutionMode::App,
            LaunchMode::CommandLine { .. } => ExecutionMode::Sdk,
            LaunchMode::Test { .. } => ExecutionMode::App,
        }
    }

    fn is_sandboxed(&self) -> bool {
        match self {
            LaunchMode::CommandLine { is_sandboxed, .. } => *is_sandboxed,
            LaunchMode::App { .. } | LaunchMode::Test { .. } => false,
        }
    }

    fn is_headless(&self) -> bool {
        match self {
            LaunchMode::CommandLine { .. } => true,
            LaunchMode::App { .. } | LaunchMode::Test { .. } => false,
        }
    }
}

/// The main Cute application.
pub struct CuteApp {
    execution_mode: ExecutionMode,
}

impl CuteApp {
    fn new(execution_mode: ExecutionMode) -> Self {
        Self { execution_mode }
    }
}

/// Initialize the application.
pub fn launch(_mode: LaunchMode, ctx: &mut AppContext) {
    // Initialize terminal module
    terminal::init(ctx);

    // Create the main window with terminal view
    ctx.add_window(cuteui::AddWindowOptions::default(), |ctx| {
        terminal::TerminalView::new()
    });

    log::info!("Cute terminal initialized");
}

/// Run the Cute application.
pub fn run() -> anyhow::Result<()> {
    // Perform platform-specific initialization
    platform::init();

    // Create app builder
    let app_builder = cuteui::platform::AppBuilder::new(
        cuteui::platform::AppCallbacks::default(),
        Box::new(ASSETS),
        None,
    );

    // Run the application
    let _ = app_builder.run(move |ctx| {
        // Launch with default mode
        launch(LaunchMode::App {
            args: cute_cli::AppArgs::default(),
            api_key: None,
        }, ctx);
    });

    Ok(())
}

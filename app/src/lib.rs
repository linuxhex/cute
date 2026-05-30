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
use cuteui::SingletonEntity;

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
    eprintln!("[DEBUG] launch() called");

    // Initialize terminal module
    terminal::init(ctx);
    eprintln!("[DEBUG] terminal::init() completed");

    // Initialize Appearance singleton with default values
    ctx.add_singleton_model(|ctx| {
        eprintln!("[DEBUG] Creating Appearance singleton");
        use cuteui::fonts::FamilyId;
        use cuteui::color::ColorU;
        use cute_core::ui::theme::{Fill, Details, TerminalColors, AnsiColors, AnsiColor};
        use cute_core::ui::appearance::Appearance;

        // Load fonts
        let font_family = cuteui::fonts::Cache::handle(ctx)
            .update(ctx, |cache, _| {
                cache.load_system_font("Menlo")
                    .or_else(|_| cache.load_system_font("Monaco"))
                    .or_else(|_| cache.load_system_font("Helvetica"))
                    .unwrap_or(FamilyId(0))
            });

        eprintln!("[DEBUG] Using font family: {:?}", font_family);

        // Create terminal colors (standard dark theme colors)
        let terminal_colors = TerminalColors::new(
            AnsiColors::new(
                AnsiColor::from(ColorU::new(0, 0, 0, 255)),      // black
                AnsiColor::from(ColorU::new(205, 49, 49, 255)),   // red
                AnsiColor::from(ColorU::new(13, 188, 121, 255)),  // green
                AnsiColor::from(ColorU::new(229, 229, 16, 255)),  // yellow
                AnsiColor::from(ColorU::new(36, 114, 200, 255)),  // blue
                AnsiColor::from(ColorU::new(188, 63, 188, 255)),  // magenta
                AnsiColor::from(ColorU::new(17, 168, 205, 255)),  // cyan
                AnsiColor::from(ColorU::new(229, 229, 229, 255)), // white
            ),
            AnsiColors::new(
                AnsiColor::from(ColorU::new(102, 102, 102, 255)), // bright black
                AnsiColor::from(ColorU::new(241, 76, 76, 255)),   // bright red
                AnsiColor::from(ColorU::new(35, 206, 135, 255)),  // bright green
                AnsiColor::from(ColorU::new(245, 245, 67, 255)),  // bright yellow
                AnsiColor::from(ColorU::new(59, 142, 234, 255)),  // bright blue
                AnsiColor::from(ColorU::new(214, 112, 214, 255)), // bright magenta
                AnsiColor::from(ColorU::new(41, 184, 219, 255)),  // bright cyan
                AnsiColor::from(ColorU::new(255, 255, 255, 255)), // bright white
            ),
        );

        // Create a simple dark theme
        let theme = cute_core::ui::theme::CuteTheme::new(
            Fill::Solid(ColorU::new(30, 30, 30, 255)),  // background
            ColorU::new(220, 220, 220, 255),             // foreground
            Fill::Solid(ColorU::new(100, 180, 100, 255)), // accent
            None,
            Some(Details::Darker),
            terminal_colors,
            None,
            Some("Dark".to_string()),
        );

        Appearance::new(
            theme,
            font_family,         // monospace font
            14.0,                // font size
            cuteui::fonts::Weight::Normal,
            font_family,         // ui font
            1.4,                 // line height ratio
            font_family,         // ai font
            font_family,         // password font
        )
    });
    eprintln!("[DEBUG] Appearance singleton created");

    // Create the main window (like examples do)
    let _ = ctx.add_window(cuteui::AddWindowOptions::default(), |_ctx| {
        eprintln!("[DEBUG] Building TerminalView...");
        terminal::TerminalView::new()
    });

    eprintln!("[DEBUG] Window created");
}

/// Run the Cute application.
pub fn run() -> anyhow::Result<()> {
    // Perform platform-specific initialization
    platform::init();

    // Create app builder
    let mut app_builder = cuteui::platform::AppBuilder::new(
        cuteui::platform::AppCallbacks::default(),
        Box::new(ASSETS),
        None,
    );

    // macOS specific configuration
    #[cfg(target_os = "macos")]
    {
        use cuteui::platform::mac::AppExt;
        use cuteui::AssetProvider as _;

        // Activate on launch so the window shows immediately
        app_builder.set_activate_on_launch(true);

        // Set dev icon for running outside app bundle
        if let Ok(dev_icon) = ASSETS.get("bundled/png/local.png") {
            app_builder.set_dev_icon(dev_icon);
        }
    }

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

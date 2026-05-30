//! Terminal view implementation
//! A simplified terminal view with input, output, sidebars, and status

use cuteui::{AppContext, Element, View, Entity, TypedActionView, ModelContext, SingletonEntity, ViewContext};
use cuteui::elements::{Rect, Stack, Flex, Empty, Text, Expanded, Container};
use cuteui::fonts::FamilyId;
use pathfinder_color::ColorU;
use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub mod mac;

const FONT_SIZE: f32 = 14.0;
const DEFAULT_FONT_FAMILY: FamilyId = FamilyId(0);

/// Main terminal view
pub struct TerminalView {
    model: Arc<Mutex<TerminalModel>>,
    file_explorer: FileExplorerSidebar,
    diff_panel: DiffPanelView,
    /// Command history navigation index
    history_index: Option<usize>,
}

pub struct TerminalModel {
    /// Terminal output lines
    output_lines: VecDeque<String>,
    /// Input buffer
    input_buffer: String,
    /// Command history
    command_history: VecDeque<String>,
    /// Current working directory
    current_dir: PathBuf,
    /// Git branch (if in a git repo)
    git_branch: Option<String>,
    /// Cursor position
    cursor_pos: usize,
    /// Last command status
    last_status: Option<i32>,
    /// Last command execution time
    last_exec_time: Option<f64>,
    /// Left sidebar visible
    left_sidebar_visible: bool,
    /// Right panel visible
    right_panel_visible: bool,
    /// Maximum output lines to keep
    max_output_lines: usize,
    /// Maximum command history
    max_history: usize,
}

impl TerminalModel {
    pub fn new() -> Self {
        let mut model = Self {
            output_lines: VecDeque::with_capacity(1000),
            input_buffer: String::new(),
            command_history: VecDeque::with_capacity(1000),
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("~")),
            git_branch: None,
            cursor_pos: 0,
            last_status: None,
            last_exec_time: None,
            left_sidebar_visible: true,
            right_panel_visible: false,
            max_output_lines: 1000,
            max_history: 1000,
        };

        // Add welcome message
        model.add_output("Welcome to Cute Terminal".to_string());
        model.add_output("Type 'help' for available commands".to_string());

        // Try to detect git branch
        model.detect_git_branch();

        model
    }

    pub fn with_model_context(_ctx: &mut ModelContext<Self>) -> Self {
        Self::new()
    }

    fn add_output(&mut self, line: String) {
        if self.output_lines.len() >= self.max_output_lines {
            self.output_lines.pop_front();
        }
        self.output_lines.push_back(line);
    }

    fn detect_git_branch(&mut self) {
        // Use the git module to get status
        if let Some(status) = crate::git::get_git_status(&self.current_dir) {
            self.git_branch = status.branch;
        } else {
            self.git_branch = None;
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        if cmd.is_empty() {
            return;
        }

        // Add to history
        if self.command_history.len() >= self.max_history {
            self.command_history.pop_front();
        }
        self.command_history.push_back(cmd.to_string());

        // Show command in output
        self.add_output(format!("$ {}", cmd));

        // Handle built-in commands
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let result = match parts.get(0).map(|s| *s) {
            Some("help") => self.cmd_help(),
            Some("clear") => self.cmd_clear(),
            Some("pwd") => self.cmd_pwd(),
            Some("ls") => self.cmd_ls(parts.get(1).map(|s| *s)),
            Some("cd") => self.cmd_cd(parts.get(1).map(|s| *s)),
            Some("exit") => {
                self.add_output("Goodbye!".to_string());
                return;
            }
            _ => self.cmd_external(cmd),
        };

        match result {
            Ok(output) => {
                for line in output.lines() {
                    self.add_output(line.to_string());
                }
                self.last_status = Some(0);
            }
            Err(e) => {
                self.add_output(format!("Error: {}", e));
                self.last_status = Some(1);
            }
        }
    }

    fn cmd_help(&mut self) -> Result<String, String> {
        Ok(r#"Available commands:
  help     - Show this help message
  clear    - Clear the terminal
  pwd      - Print working directory
  ls       - List directory contents
  cd       - Change directory
  exit     - Exit the terminal

Any other command will be executed in the shell."#.to_string())
    }

    fn cmd_clear(&mut self) -> Result<String, String> {
        self.output_lines.clear();
        Ok(String::new())
    }

    fn cmd_pwd(&mut self) -> Result<String, String> {
        Ok(self.current_dir.display().to_string())
    }

    fn cmd_ls(&mut self, path: Option<&str>) -> Result<String, String> {
        let dir = match path {
            Some(p) => self.current_dir.join(p),
            None => self.current_dir.clone(),
        };

        let mut result = String::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if is_dir {
                    result.push_str(&format!("[DIR]  {}\n", name));
                } else {
                    result.push_str(&format!("[FILE] {}\n", name));
                }
            }
        } else {
            return Err(format!("Cannot read directory: {}", dir.display()));
        }
        Ok(result)
    }

    fn cmd_cd(&mut self, path: Option<&str>) -> Result<String, String> {
        match path {
            Some(p) => {
                let new_dir = if p == "~" {
                    dirs::home_dir().unwrap_or_else(|| self.current_dir.clone())
                } else {
                    self.current_dir.join(p)
                };

                if new_dir.is_dir() {
                    self.current_dir = new_dir.canonicalize().unwrap_or(new_dir);
                    self.detect_git_branch();
                    Ok(String::new())
                } else {
                    Err(format!("Not a directory: {}", p))
                }
            }
            None => {
                self.current_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
                self.detect_git_branch();
                Ok(String::new())
            }
        }
    }

    fn cmd_external(&mut self, cmd: &str) -> Result<String, String> {
        use std::process::Command;

        let output = Command::new("sh")
            .args(&["-c", cmd])
            .current_dir(&self.current_dir)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            if !stderr.is_empty() {
                Err(stderr)
            } else {
                Ok(stdout)
            }
        }
    }
}

impl TerminalView {
    pub fn new() -> Self {
        Self {
            model: Arc::new(Mutex::new(TerminalModel::new())),
            file_explorer: FileExplorerSidebar::new(),
            diff_panel: DiffPanelView::new(),
            history_index: None,
        }
    }
}

impl View for TerminalView {
    fn ui_name() -> &'static str {
        "TerminalView"
    }

    fn render(&self, ctx: &AppContext) -> Box<dyn Element> {
        // Get appearance for proper font
        let appearance = cute_core::ui::appearance::Appearance::as_ref(ctx);
        let font_family = appearance.monospace_font_family();
        let font_size = appearance.monospace_font_size();

        let model = self.model.lock();
        let left_visible = model.left_sidebar_visible;
        let right_visible = model.right_panel_visible;
        let current_dir = model.current_dir.display().to_string();
        let git_branch = model.git_branch.clone();
        let input_buffer = model.input_buffer.clone();
        let output_lines: Vec<String> = model.output_lines.iter().cloned().collect();
        drop(model);

        // Background
        let background = Rect::new()
            .with_background_color(ColorU::new(30, 30, 30, 255))
            .finish();

        // Header bar
        let header = self.render_header_with_font(&current_dir, git_branch.as_deref(), font_family, font_size);

        // Main content area (expand to fill remaining space)
        let main_content = Expanded::new(1.0, self.render_main_content_with_font(
            ctx,
            &output_lines,
            &input_buffer,
            left_visible,
            right_visible,
            font_family,
            font_size,
        )).finish();

        // Status bar
        let status_bar = self.render_status_bar_with_font(font_family, font_size);

        // Vertical layout
        let mut layout = Flex::column();
        layout.extend(vec![header, main_content, status_bar]);
        let layout = layout.finish();

        // Stack background and layout
        let mut stack = Stack::new();
        stack.extend(vec![background, layout]);
        stack.finish()
    }
}

impl TerminalView {
    fn render_header_with_font(&self, current_dir: &str, git_branch: Option<&str>, font_family: FamilyId, font_size: f32) -> Box<dyn Element> {
        // Header background
        let header_bg = Rect::new()
            .with_background_color(ColorU::new(45, 45, 45, 255))
            .finish();

        // Directory text
        let dir_text = Text::new(current_dir.to_string(), font_family, font_size)
            .with_color(ColorU::new(200, 200, 200, 255))
            .finish();

        // Git branch (if present)
        let git_text = if let Some(branch) = git_branch {
            Text::new(format!("  ({})", branch), font_family, font_size)
                .with_color(ColorU::new(100, 180, 100, 255))
                .finish()
        } else {
            Empty::new().finish()
        };

        // Header layout
        let mut header_layout = Flex::row();
        header_layout.extend(vec![dir_text, git_text]);
        let header_layout = header_layout.finish();

        // Container with padding
        let header_content = Container::new(header_layout)
            .with_vertical_padding(8.0)
            .with_horizontal_padding(12.0)
            .finish();

        // Stack
        let mut stack = Stack::new();
        stack.extend(vec![header_bg, header_content]);
        stack.finish()
    }

    fn render_main_content_with_font(
        &self,
        ctx: &AppContext,
        output_lines: &[String],
        input_buffer: &str,
        left_visible: bool,
        right_visible: bool,
        font_family: FamilyId,
        font_size: f32,
    ) -> Box<dyn Element> {
        // Left sidebar
        let left_sidebar = if left_visible {
            self.file_explorer.render(ctx)
        } else {
            Empty::new().finish()
        };

        // Terminal area (output + input) - expand to fill remaining space
        let terminal_area = Expanded::new(1.0, self.render_terminal_area_with_font(output_lines, input_buffer, font_family, font_size)).finish();

        // Right panel
        let right_panel = if right_visible {
            self.diff_panel.render(ctx)
        } else {
            Empty::new().finish()
        };

        // Horizontal layout
        let mut layout = Flex::row();
        layout.extend(vec![left_sidebar, terminal_area, right_panel]);
        layout.finish()
    }

    fn render_terminal_area_with_font(&self, output_lines: &[String], input_buffer: &str, font_family: FamilyId, font_size: f32) -> Box<dyn Element> {
        // Terminal background
        let terminal_bg = Rect::new()
            .with_background_color(ColorU::new(30, 30, 30, 255))
            .finish();

        // Output area (expand to fill remaining space)
        let output_area = Expanded::new(1.0, self.render_output_with_font(output_lines, font_family, font_size)).finish();

        // Input area
        let input_area = self.render_input_with_font(input_buffer, font_family, font_size);

        // Vertical layout
        let mut layout = Flex::column();
        layout.extend(vec![output_area, input_area]);
        let layout = layout.finish();

        // Stack
        let mut stack = Stack::new();
        stack.extend(vec![terminal_bg, layout]);
        stack.finish()
    }

    fn render_output_with_font(&self, lines: &[String], font_family: FamilyId, font_size: f32) -> Box<dyn Element> {
        // Output background
        let output_bg = Rect::new()
            .with_background_color(ColorU::new(30, 30, 30, 255))
            .finish();

        // Build output lines
        let mut output_layout = Flex::column();
        for line in lines.iter().take(100) { // Limit to 100 lines for performance
            let text = Text::new(line.clone(), font_family, font_size)
                .with_color(ColorU::new(220, 220, 220, 255))
                .finish();
            output_layout.extend(vec![text]);
        }
        let output_layout = output_layout.finish();

        // Container with padding
        let output_content = Container::new(output_layout)
            .with_horizontal_padding(12.0)
            .with_padding_top(8.0)
            .finish();

        // Stack
        let mut stack = Stack::new();
        stack.extend(vec![output_bg, output_content]);
        stack.finish()
    }

    fn render_input_with_font(&self, buffer: &str, font_family: FamilyId, font_size: f32) -> Box<dyn Element> {
        // Input background
        let input_bg = Rect::new()
            .with_background_color(ColorU::new(40, 40, 40, 255))
            .finish();

        // Prompt
        let prompt = Text::new("$ ".to_string(), font_family, font_size)
            .with_color(ColorU::new(100, 200, 100, 255))
            .finish();

        // Input text
        let input_text = Text::new(buffer.to_string(), font_family, font_size)
            .with_color(ColorU::new(220, 220, 220, 255))
            .finish();

        // Cursor
        let cursor = Text::new("|".to_string(), font_family, font_size)
            .with_color(ColorU::new(200, 200, 200, 255))
            .finish();

        // Horizontal layout
        let mut layout = Flex::row();
        layout.extend(vec![prompt, input_text, cursor]);
        let layout = layout.finish();

        // Container with padding
        let input_content = Container::new(layout)
            .with_vertical_padding(8.0)
            .with_horizontal_padding(12.0)
            .finish();

        // Stack
        let mut stack = Stack::new();
        stack.extend(vec![input_bg, input_content]);
        stack.finish()
    }

    fn render_status_bar_with_font(&self, font_family: FamilyId, font_size: f32) -> Box<dyn Element> {
        let model = self.model.lock();
        let status = model.last_status;
        let exec_time = model.last_exec_time;
        drop(model);

        // Status bar background
        let status_bg = Rect::new()
            .with_background_color(ColorU::new(45, 45, 45, 255))
            .finish();

        // Status text
        let status_text = match status {
            Some(0) => "OK".to_string(),
            Some(_) => "ERR".to_string(),
            None => "---".to_string(),
        };

        let status_color = match status {
            Some(0) => ColorU::new(100, 200, 100, 255),
            Some(_) => ColorU::new(200, 100, 100, 255),
            None => ColorU::new(150, 150, 150, 255),
        };

        let status = Text::new(status_text, font_family, font_size)
            .with_color(status_color)
            .finish();

        // Execution time
        let time_text = if let Some(t) = exec_time {
            Text::new(format!(" {:.2}s", t), font_family, font_size)
                .with_color(ColorU::new(150, 150, 150, 255))
                .finish()
        } else {
            Empty::new().finish()
        };

        // Layout
        let mut layout = Flex::row();
        layout.extend(vec![status, time_text]);
        let layout = layout.finish();

        // Container with padding
        let status_content = Container::new(layout)
            .with_vertical_padding(6.0)
            .with_horizontal_padding(12.0)
            .finish();

        // Stack
        let mut stack = Stack::new();
        stack.extend(vec![status_bg, status_content]);
        stack.finish()
    }
}

impl Entity for TerminalView {
    type Event = TerminalEvent;
}

impl TypedActionView for TerminalView {
    type Action = TerminalAction;

    fn handle_action(&mut self, action: &TerminalAction, ctx: &mut ViewContext<Self>) {
        match action {
            TerminalAction::ExecuteCommand(cmd) => {
                let mut model = self.model.lock();
                model.execute_command(cmd);
                // Reset input buffer after execution
                model.input_buffer.clear();
                model.cursor_pos = 0;
                // Reset history index
                drop(model);
                self.history_index = None;
                ctx.notify();
            }
            TerminalAction::ClearOutput => {
                self.model.lock().output_lines.clear();
                ctx.notify();
            }
            TerminalAction::ToggleSidebar => {
                let mut model = self.model.lock();
                model.left_sidebar_visible = !model.left_sidebar_visible;
                drop(model);
                ctx.notify();
            }
            TerminalAction::TypedCharacters(chars) => {
                let mut model = self.model.lock();
                model.input_buffer.push_str(chars);
                model.cursor_pos += chars.len();
                drop(model);
                ctx.notify();
            }
            TerminalAction::KeyDown(key) => {
                // Handle control characters
                if key == "\x7f" || key == "\x08" {
                    // Backspace
                    let mut model = self.model.lock();
                    if model.cursor_pos > 0 && !model.input_buffer.is_empty() {
                        let pos = model.cursor_pos - 1;
                        model.cursor_pos = pos;
                        model.input_buffer.remove(pos);
                    }
                    drop(model);
                    ctx.notify();
                } else if key == "\x0d" || key == "\n" {
                    // Enter
                    let cmd = {
                        let mut model = self.model.lock();
                        let cmd = model.input_buffer.clone();
                        if !cmd.is_empty() {
                            model.execute_command(&cmd);
                            model.input_buffer.clear();
                            model.cursor_pos = 0;
                        }
                        cmd
                    };
                    if !cmd.is_empty() {
                        self.history_index = None;
                    }
                    ctx.notify();
                } else if key == "\x1b" {
                    // Escape - could be used for various things
                } else if key == "\x1b[A" {
                    // Up arrow - history previous
                    self.navigate_history_prev(ctx);
                } else if key == "\x1b[B" {
                    // Down arrow - history next
                    self.navigate_history_next(ctx);
                }
            }
        }
    }
}

impl TerminalView {
    fn navigate_history_prev(&mut self, ctx: &mut ViewContext<Self>) {
        let model = self.model.lock();
        let history_len = model.command_history.len();
        drop(model);

        if history_len == 0 {
            return;
        }

        let index = self.history_index.unwrap_or(history_len);
        if index > 0 {
            let new_index = index - 1;
            self.history_index = Some(new_index);

            let model = self.model.lock();
            if let Some(cmd) = model.command_history.iter().rev().nth(new_index) {
                let cmd = cmd.clone();
                let cmd_len = cmd.len();
                drop(model);
                let mut model = self.model.lock();
                model.input_buffer = cmd;
                model.cursor_pos = cmd_len;
            }
        }
        ctx.notify();
    }

    fn navigate_history_next(&mut self, ctx: &mut ViewContext<Self>) {
        let model = self.model.lock();
        let history_len = model.command_history.len();
        drop(model);

        if history_len == 0 {
            return;
        }

        if let Some(index) = self.history_index {
            if index < history_len - 1 {
                let new_index = index + 1;
                self.history_index = Some(new_index);

                let model = self.model.lock();
                if let Some(cmd) = model.command_history.iter().rev().nth(new_index) {
                    let cmd = cmd.clone();
                    let cmd_len = cmd.len();
                    drop(model);
                    let mut model = self.model.lock();
                    model.input_buffer = cmd;
                    model.cursor_pos = cmd_len;
                }
            } else {
                // At the end, clear input
                self.history_index = None;
                let mut model = self.model.lock();
                model.input_buffer.clear();
                model.cursor_pos = 0;
            }
        }
        ctx.notify();
    }
}

#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// Key down event (control characters)
    KeyDown(String),
    /// Typed characters (printable characters)
    TypedCharacters(String),
    /// Single character input
    Input(char),
    /// Backspace
    Backspace,
    /// Enter key
    Enter,
    /// Clear output
    Clear,
    /// Toggle left sidebar
    ToggleLeftSidebar,
    /// Toggle right panel
    ToggleRightPanel,
    /// Execute command
    ExecuteCommand(String),
}

#[derive(Debug, Clone)]
pub enum TerminalAction {
    /// Execute a command
    ExecuteCommand(String),
    /// Clear output
    ClearOutput,
    /// Toggle sidebar
    ToggleSidebar,
    /// Typed characters (printable)
    TypedCharacters(String),
    /// Key down (control characters)
    KeyDown(String),
}

// Re-export from file_explorer and diff_panel
pub use crate::file_explorer::{FileExplorerSidebar, FileExplorerEvent, FileExplorerAction, FileItem, FileExplorerModel};
pub use crate::diff_panel::{DiffPanelView, DiffPanelEvent, DiffPanelAction, DiffContent, DiffPanelModel};
pub use crate::git::{DiffHunk, DiffLine, DiffLineKind};

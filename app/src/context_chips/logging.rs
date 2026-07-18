#[derive(Default, Clone)]
pub struct PromptChipLogger;

#[derive(Debug, Clone)]
pub struct ChipCommandLogEntry<'a> {
    pub chip_kind: &'a crate::context_chips::ContextChipKind,
    pub chip_title: &'a str,
    pub phase: PromptChipExecutionPhase,
    pub shell_type: Option<cute_terminal::shell::ShellType>,
    pub working_directory: Option<&'a str>,
    pub command: &'a str,
    pub output: Option<&'a cute_completer::completer::CommandOutput>,
    pub timed_out: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptChipExecutionPhase {
    Value,
    OnClick,
}

impl PromptChipLogger {
    pub fn log_shell_command(&self, _entry: &ChipCommandLogEntry) {}
}

pub fn log_file_path() -> Option<std::path::PathBuf> {
    None
}

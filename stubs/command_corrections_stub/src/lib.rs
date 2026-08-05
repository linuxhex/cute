// Stub for command-corrections.
// Provides minimal types needed for compilation when command corrections are disabled.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub input: String,
}

impl Command {
    pub fn new() -> Self {
        Self { input: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    pub command: String,
    pub corrected_command: String,
    pub score: f64,
    pub rule_applied: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub command: String,
}

impl HistoryItem {
    pub fn new() -> Self {
        Self { command: String::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub shell: String,
    pub os: String,
}

impl SessionMetadata {
    pub fn new() -> Self {
        Self {
            shell: String::new(),
            os: String::new(),
        }
    }

    pub fn set_session_type(&mut self, _: SessionType) {}
    pub fn set_shell(&mut self, _: Shell) {}
    pub fn set_aliases(&mut self, _: Vec<String>) {}
    pub fn set_executables(&mut self, _: Vec<String>) {}
    pub fn set_functions(&mut self, _: Vec<String>) {}
    pub fn set_builtins(&mut self, _: Vec<String>) {}
    pub fn set_platform_type(&mut self, _: PlatformType) {}
    pub fn set_git_branches<'a>(&mut self, _: impl Iterator<Item = &'a str>) {}
    pub fn set_command(&mut self, _: &Command) {}
    pub fn set_history(&mut self, _: Vec<HistoryItem>) {}
}

pub mod rules {
    use super::*;

    pub trait Rule {
        fn id(&self) -> RuleId;
        fn apply(&self, _command: &Command, _metadata: &SessionMetadata) -> Vec<Correction>;
    }

    pub type RuleId = String;

    pub mod generic {
        pub mod history {
            use super::super::*;

            #[derive(Debug, Clone)]
            pub struct History {
                pub entries: Vec<HistoryItem>,
            }

            impl History {
                pub fn with_commands(commands: Vec<HistoryItem>) -> Self {
                    Self { entries: commands }
                }
            }

            impl Rule for History {
                fn id(&self) -> RuleId {
                    "history".to_string()
                }

                fn apply(&self, _command: &Command, _metadata: &SessionMetadata) -> Vec<Correction> {
                    vec![]
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformType {
    Macos,
    Linux,
    Windows,
    Unknown,
}

pub fn correct_command(_command: &Command, _metadata: &SessionMetadata) -> Vec<Correction> {
    vec![]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExitCode(pub i32);

impl Default for ExitCode {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for ExitCode {
    fn from(code: i32) -> Self {
        Self(code)
    }
}

impl ExitCode {
    pub fn raw(&self) -> i32 {
        self.0
    }
}
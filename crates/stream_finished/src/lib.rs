//! Stub implementation for stream_finished crate
//! This provides minimal types needed for persistence

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelTokenUsage {
    pub model_id: String,
    pub total_tokens: u32,
    pub token_usage_by_category: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCallStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunCommandStats {
    pub count: i32,
    pub command_executed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplyFileDiffStats {
    pub count: i32,
    pub lines_added: i32,
    pub lines_removed: i32,
    pub files_changed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadFilesStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchCodebaseStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrepStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileGlobStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WriteToLongRunningShellCommandStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadMcpResourceStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CallMcpToolStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuggestPlanStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuggestCreatePlanStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadShellCommandOutputStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UseComputerStats {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolUsageMetadata {
    pub tool_call_stats: Option<ToolCallStats>,
    pub run_command_stats: Option<RunCommandStats>,
    pub apply_file_diff_stats: Option<ApplyFileDiffStats>,
    pub read_files_stats: Option<ReadFilesStats>,
    pub search_codebase_stats: Option<SearchCodebaseStats>,
    pub grep_stats: Option<GrepStats>,
    pub file_glob_stats: Option<FileGlobStats>,
    pub write_to_long_running_shell_command_stats: Option<WriteToLongRunningShellCommandStats>,
    pub read_mcp_resource_stats: Option<ReadMcpResourceStats>,
    pub call_mcp_tool_stats: Option<CallMcpToolStats>,
    pub suggest_plan_stats: Option<SuggestPlanStats>,
    pub suggest_create_plan_stats: Option<SuggestCreatePlanStats>,
    pub read_shell_command_output_stats: Option<ReadShellCommandOutputStats>,
    pub use_computer_stats: Option<UseComputerStats>,
}

// Conversion implementations for ToolCallStats to/from various stat types
// These are needed because persistence uses ToolCallStats as a generic counter type

impl From<&ToolCallStats> for ReadFilesStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ReadFilesStats> for ToolCallStats {
    fn from(stats: &ReadFilesStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for SearchCodebaseStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&SearchCodebaseStats> for ToolCallStats {
    fn from(stats: &SearchCodebaseStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for GrepStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&GrepStats> for ToolCallStats {
    fn from(stats: &GrepStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for FileGlobStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&FileGlobStats> for ToolCallStats {
    fn from(stats: &FileGlobStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for WriteToLongRunningShellCommandStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&WriteToLongRunningShellCommandStats> for ToolCallStats {
    fn from(stats: &WriteToLongRunningShellCommandStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for ReadMcpResourceStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ReadMcpResourceStats> for ToolCallStats {
    fn from(stats: &ReadMcpResourceStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for CallMcpToolStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&CallMcpToolStats> for ToolCallStats {
    fn from(stats: &CallMcpToolStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for SuggestPlanStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&SuggestPlanStats> for ToolCallStats {
    fn from(stats: &SuggestPlanStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for SuggestCreatePlanStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&SuggestCreatePlanStats> for ToolCallStats {
    fn from(stats: &SuggestCreatePlanStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for ReadShellCommandOutputStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ReadShellCommandOutputStats> for ToolCallStats {
    fn from(stats: &ReadShellCommandOutputStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&ToolCallStats> for UseComputerStats {
    fn from(stats: &ToolCallStats) -> Self {
        Self { count: stats.count }
    }
}

impl From<&UseComputerStats> for ToolCallStats {
    fn from(stats: &UseComputerStats) -> Self {
        Self { count: stats.count }
    }
}

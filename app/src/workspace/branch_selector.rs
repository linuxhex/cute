use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub author_email: String,
    pub timestamp: DateTime<Local>,
    pub is_merge: bool,
    pub refs: Vec<String>,
    pub is_head: bool,
}

#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub full_name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub remote_name: Option<String>,
    pub last_commit: Option<CommitInfo>,
    pub recent_commits: Vec<CommitInfo>,
    pub graph_lines: Vec<String>,
    pub commit_row_indices: Vec<usize>,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    Add,
    Delete,
    Context,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub old_line: Option<u32>,
    pub new_line: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_count: u32,
    pub new_start: u32,
    pub new_count: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: String,
    pub status: FileStatus,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub file_path: String,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub hunks: Vec<DiffHunk>,
    pub is_binary: bool,
    pub image_local_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BranchSelectorState {
    pub branches: Vec<BranchInfo>,
    pub selected_branch_index: Option<usize>,
    pub selected_commit_index: Option<usize>,
    pub changed_files: Vec<ChangedFile>,
    pub selected_file_index: Option<usize>,
    pub current_diff: Option<FileDiff>,
    pub loading: bool,
}

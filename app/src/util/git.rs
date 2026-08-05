// Stub git module for OSS build.
// The real git module was in the deleted `app/src/util/git/` directory.

use std::path::Path;

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct FileChangeEntry {
    pub file_path: String,
    pub status: String,
}

pub async fn get_branch_commits(
    _repo_path: &Path,
    _branch_name: &str,
    _limit: usize,
) -> Result<Vec<Commit>, String> {
    Err("git not available".to_string())
}

pub async fn checkout_branch(_repo_path: &Path, _branch: &str) -> Result<(), String> {
    Err("git not available".to_string())
}

pub async fn merge_branch(_repo_path: &Path, _branch: &str) -> Result<(), String> {
    Err("git not available".to_string())
}

pub async fn delete_branch(_repo_path: &Path, _branch: &str) -> Result<(), String> {
    Err("git not available".to_string())
}

pub async fn get_commit_files(
    _repo_path: &Path,
    _commit_hash: &str,
) -> Result<Vec<FileChangeEntry>, String> {
    Err("git not available".to_string())
}

pub fn detect_current_branch_sync(_repo_path: &Path) -> Result<String, String> {
    Err("git not available".to_string())
}

pub fn list_local_branches_sync(_repo_path: &Path) -> Result<Vec<String>, String> {
    Err("git not available".to_string())
}

pub async fn run_git_command(_repo_path: &Path, _args: &[&str]) -> Result<String, String> {
    Err("git not available".to_string())
}

#[derive(Debug, Clone)]
pub struct GitStatusUpdateModel;

impl GitStatusUpdateModel {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct GitBranchOnClickValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitDeltaPreference { Always, Never }

#[derive(Debug, Clone)]
pub enum DiffMode { Head, MainBranch, OtherBranch(String) }

#[derive(Debug, Clone)]
pub struct DiffSetScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus { Added, Deleted, Modified, Renamed }

#[derive(Debug, Clone)]
pub struct BranchEntry {
    pub name: String,
    pub is_head: bool,
    pub is_main: bool,
}

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct RepositoryInfo {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct PrInfo {
    pub number: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct GitStatusMetadata;

#[derive(Debug, Clone)]
pub struct DiffStats {
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Debug, Clone)]
pub struct RepoGitSummary;

pub fn get_all_branches(_repo_path: &Path) -> Result<Vec<BranchEntry>, String> {
    Err("git not available".to_string())
}

pub fn get_all_branches_with_known_main(_repo_path: &Path, _main_branch: &str) -> Result<Vec<BranchEntry>, String> {
    Err("git not available".to_string())
}

pub fn detect_current_branch(_repo_path: &Path) -> Result<String, String> {
    Err("git not available".to_string())
}

pub fn detect_current_branch_display(_repo_path: &Path) -> Result<String, String> {
    Err("git not available".to_string())
}

pub fn sort_branches_main_first(_branches: &mut [BranchEntry], _main_branch: &str) {}

pub fn is_plausible_new_branch_name(_name: &str) -> bool {
    false
}

pub fn get_repo_git_summary(_repo_path: &Path) -> Result<RepoGitSummary, String> {
    Err("git not available".to_string())
}

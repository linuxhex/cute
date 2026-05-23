use std::path::PathBuf;
use std::process::Command;

use warpui::Model;

#[derive(Clone, Debug, PartialEq)]
pub enum GitFileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
    Untracked,
}

#[derive(Clone, Debug)]
pub struct GitFile {
    pub path: PathBuf,
    pub status: GitFileStatus,
}

#[derive(Clone, Debug)]
pub enum GitDiffModelEvent {
    FilesUpdated,
}

pub struct GitDiffModel {
    files: Vec<GitFile>,
    repo_root: Option<PathBuf>,
}

impl GitDiffModel {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            repo_root: None,
        }
    }

    pub fn files(&self) -> &[GitFile] {
        &self.files
    }

    pub fn repo_root(&self) -> Option<&PathBuf> {
        self.repo_root.as_ref()
    }

    pub fn refresh(&mut self, working_dir: &std::path::Path) {
        self.files.clear();

        let repo_root = self.find_git_root(working_dir);
        if repo_root.is_none() {
            self.repo_root = None;
            return;
        }
        self.repo_root = repo_root;

        if let Some(root) = &self.repo_root {
            self.parse_git_status(root);
        }
    }

    fn find_git_root(&self, dir: &std::path::Path) -> Option<PathBuf> {
        let mut current = dir.to_path_buf();
        loop {
            if current.join(".git").exists() {
                return Some(current);
            }
            if !current.pop() {
                return None;
            }
        }
    }

    fn parse_git_status(&mut self, repo_root: &std::path::Path) {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(repo_root)
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.len() < 3 {
                    continue;
                }
                let status_code = &line[0..2];
                let file_path = &line[3..];

                let status = match status_code.trim() {
                    "A" | "AA" => GitFileStatus::Added,
                    "M" | "MM" | "AM" => GitFileStatus::Modified,
                    "D" | "AD" => GitFileStatus::Deleted,
                    "R" => GitFileStatus::Renamed,
                    "??" => GitFileStatus::Untracked,
                    _ => continue,
                };

                let path = repo_root.join(file_path);
                self.files.push(GitFile { path, status });
            }
        }
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn has_changes(&self) -> bool {
        !self.files.is_empty()
    }
}

impl Default for GitDiffModel {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for GitDiffModel {
    type Event = GitDiffModelEvent;
}

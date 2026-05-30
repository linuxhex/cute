//! Git integration for the terminal
//! Provides branch detection, status, and diff functionality

use std::path::PathBuf;
use std::process::Command;

/// Git status information
#[derive(Debug, Clone, Default)]
pub struct GitStatus {
    pub branch: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub conflicted: usize,
}

impl GitStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_clean(&self) -> bool {
        self.staged == 0 && self.unstaged == 0 && self.untracked == 0 && self.conflicted == 0
    }

    pub fn has_changes(&self) -> bool {
        !self.is_clean()
    }
}

/// Get git status for a directory
pub fn get_git_status(dir: &PathBuf) -> Option<GitStatus> {
    let mut status = GitStatus::new();

    // Get branch name
    let branch_output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(dir)
        .output()
        .ok()?;

    if branch_output.status.success() {
        let branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
        if !branch.is_empty() {
            status.branch = Some(branch);
        } else {
            // Check for detached HEAD
            let head_output = Command::new("git")
                .args(&["rev-parse", "--short", "HEAD"])
                .current_dir(dir)
                .output()
                .ok()?;

            if head_output.status.success() {
                let head = String::from_utf8_lossy(&head_output.stdout).trim().to_string();
                status.branch = Some(format!("HEAD:{}", head));
            }
        }
    } else {
        return None;
    }

    // Get ahead/behind counts
    let ahead_behind_output = Command::new("git")
        .args(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
        .current_dir(dir)
        .output()
        .ok();

    if let Some(output) = ahead_behind_output {
        if output.status.success() {
            let counts = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = counts.trim().split_whitespace().collect();
            if parts.len() == 2 {
                status.ahead = parts[0].parse().unwrap_or(0);
                status.behind = parts[1].parse().unwrap_or(0);
            }
        }
    }

    // Get status counts using porcelain format
    let status_output = Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(dir)
        .output()
        .ok()?;

    if status_output.status.success() {
        let status_str = String::from_utf8_lossy(&status_output.stdout);
        for line in status_str.lines() {
            if line.len() >= 2 {
                let index_status = line.chars().next().unwrap_or(' ');
                let work_status = line.chars().nth(1).unwrap_or(' ');

                // Index status
                match index_status {
                    'M' | 'A' | 'D' | 'R' | 'C' => status.staged += 1,
                    _ => {}
                }

                // Work tree status
                match work_status {
                    'M' | 'D' => status.unstaged += 1,
                    '?' => status.untracked += 1,
                    'U' => status.conflicted += 1,
                    _ => {}
                }

                // Handle special cases
                if index_status == 'U' || work_status == 'U' {
                    status.conflicted += 1;
                }
            }
        }
    }

    Some(status)
}

/// Get git diff for a file
pub fn get_file_diff(dir: &PathBuf, file_path: &PathBuf) -> Option<String> {
    let output = Command::new("git")
        .args(&["diff", "--", file_path.to_string_lossy().as_ref()])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

/// Get staged diff
pub fn get_staged_diff(dir: &PathBuf) -> Option<String> {
    let output = Command::new("git")
        .args(&["diff", "--cached"])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

/// Get all changes diff
pub fn get_all_diff(dir: &PathBuf) -> Option<String> {
    let output = Command::new("git")
        .args(&["diff", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

/// Parse diff content into hunks
pub fn parse_diff_hunks(diff: &str) -> Vec<DiffHunk> {
    let mut hunks: Vec<DiffHunk> = Vec::new();
    let mut current_hunk: Option<DiffHunk> = None;

    for line in diff.lines() {
        if line.starts_with("@@") {
            // Save previous hunk
            if let Some(hunk) = current_hunk.take() {
                hunks.push(hunk);
            }

            // Parse hunk header
            // Format: @@ -old_start,old_count +new_start,new_count @@ optional_text
            let header = line.strip_prefix("@@").unwrap_or("");
            let parts: Vec<&str> = header.split_whitespace().collect();

            let (old_start, old_count) = if parts.len() > 0 {
                parse_range(parts[0].strip_prefix('-').unwrap_or("0,0"))
            } else {
                (0, 0)
            };

            let (new_start, new_count) = if parts.len() > 1 {
                parse_range(parts[1].strip_prefix('+').unwrap_or("0,0"))
            } else {
                (0, 0)
            };

            current_hunk = Some(DiffHunk {
                old_start,
                old_count,
                new_start,
                new_count,
                lines: Vec::new(),
            });
        } else if let Some(ref mut hunk) = current_hunk {
            // Add line to current hunk
            let kind = if line.starts_with('+') {
                DiffLineKind::Added
            } else if line.starts_with('-') {
                DiffLineKind::Removed
            } else {
                DiffLineKind::Context
            };

            hunk.lines.push(DiffLine {
                kind,
                content: line.to_string(),
            });
        }
    }

    // Save last hunk
    if let Some(hunk) = current_hunk {
        hunks.push(hunk);
    }

    hunks
}

fn parse_range(s: &str) -> (usize, usize) {
    let parts: Vec<&str> = s.split(',').collect();
    let start = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
    let count = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    (start, count)
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiffLineKind {
    Context,
    Added,
    Removed,
}

/// Check if directory is a git repository
pub fn is_git_repo(dir: &PathBuf) -> bool {
    Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .current_dir(dir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diff_hunks() {
        let diff = r#"@@ -1,3 +1,4 @@
 line1
-line2
+line2a
+line2b
 line3
"#;
        let hunks = parse_diff_hunks(diff);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[0].new_start, 1);
        assert_eq!(hunks[0].lines.len(), 5);
    }
}

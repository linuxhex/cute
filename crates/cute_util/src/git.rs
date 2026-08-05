use std::path::Path;

pub async fn run_git_command(_repo_path: &Path, _args: &[&str]) -> Result<String, String> {
    Err("git not available".to_string())
}
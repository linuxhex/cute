use cuteui::AppContext;

use crate::context_chips::display_chip::GitLineChanges;
use crate::context_chips::{git_line_changes_from_chips, ContextChipKind};
use crate::terminal::TerminalView;

impl TerminalView {
    pub fn preferred_tab_title(&self, ctx: &AppContext) -> String {
        // 优先使用用户设置的自定义标题
        let model = self.model.lock();
        if let Some(custom_title) = model.custom_title() {
            if !custom_title.trim().is_empty() {
                return custom_title;
            }
        }

        // 如果没有自定义标题，使用原有的逻辑（工作目录名或 shell 标题）
        if let Some(wd) = self.display_working_directory(ctx).filter(|wd| !wd.trim().is_empty()) {
            if let Some(name) = std::path::Path::new(&wd)
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.trim().is_empty())
            {
                return name.to_string();
            }
            return wd;
        }
        self.terminal_title_from_shell()
    }

    fn prompt_chip_value(&self, chip_kind: &ContextChipKind, ctx: &AppContext) -> Option<String> {
        self.current_prompt
            .as_ref(ctx)
            .latest_chip_value(chip_kind, ctx)
            .map(|v| v.to_string())
            .filter(|value| !value.trim().is_empty())
    }

    pub fn display_working_directory(&self, ctx: &AppContext) -> Option<String> {
        let raw = self
            .prompt_chip_value(&ContextChipKind::WorkingDirectory, ctx)
            .or_else(|| self.pwd())?;
        let home_dir = self
            .active_block_session_id()
            .and_then(|session_id| self.sessions.as_ref(ctx).get(session_id))
            .and_then(|session| session.home_dir().map(str::to_owned));
        Some(cute_util::path::user_friendly_path(&raw, home_dir.as_deref()).to_string())
    }

    pub fn terminal_title_from_shell(&self) -> String {
        let model = self.model.lock();
        let fallback_title = model.shell_launch_state().display_name().to_owned();
        let shell_title = model
            .terminal_title()
            .filter(|title| !title.trim().is_empty())
            .unwrap_or(fallback_title);
        // 当 shell 未上报动态标题时，优先用当前工作目录名做页签标题，
        // 避免回退到“zsh/bash”等通用 shell 名称导致所有页签同名。
        if let Some(pwd) = self.pwd().filter(|pwd| !pwd.trim().is_empty()) {
            let dir_name = std::path::Path::new(&pwd)
                .file_name()
                .and_then(|name| name.to_str())
                .map(str::to_owned)
                .filter(|name| !name.trim().is_empty())
                .unwrap_or(pwd);
            if shell_title == "zsh" || shell_title == "bash" || shell_title == "fish" {
                return dir_name;
            }
        }
        shell_title
    }

    #[cfg_attr(not(feature = "local_fs"), allow(clippy::unnecessary_lazy_evaluations))]
    pub fn current_git_branch(&self, ctx: &AppContext) -> Option<String> {
        self.prompt_chip_value(&ContextChipKind::ShellGitBranch, ctx)
            .or_else(|| {
                #[cfg(feature = "local_fs")]
                {
                    self.git_status_metadata(ctx)
                        .map(|metadata| metadata.current_branch_name.clone())
                        .filter(|branch| !branch.trim().is_empty())
                }
                #[cfg(not(feature = "local_fs"))]
                {
                    None
                }
            })
    }

    pub fn last_completed_command_text(&self) -> Option<String> {
        let model = self.model.lock();
        model.block_list().blocks().iter().rev().find_map(|block| {
            if block.finished()
                && !block.is_background()
                && !block.is_static()
                && (block.bootstrap_stage().is_done() || block.is_restored())
            {
                let cmd = block.command_to_string();
                if cmd.trim().is_empty() {
                    None
                } else {
                    Some(cmd)
                }
            } else {
                None
            }
        })
    }

    pub fn terminal_title_text(&self) -> String {
        if !self.terminal_title.trim().is_empty() {
            return self.terminal_title.clone();
        }
        self.terminal_title_from_shell()
    }

    pub fn current_pull_request_url(&self, ctx: &AppContext) -> Option<String> {
        self.current_prompt
            .as_ref(ctx)
            .latest_chip_value(&ContextChipKind::GithubPullRequest, ctx)
            .map(|v| v.to_string())
            .filter(|value| !value.trim().is_empty())
    }

    #[cfg_attr(not(feature = "local_fs"), allow(clippy::unnecessary_lazy_evaluations))]
    pub fn current_diff_line_changes(&self, ctx: &AppContext) -> Option<GitLineChanges> {
        // Prefer the filesystem-event-based GitRepoStatusModel (which includes
        // untracked files) over parsing the raw shell chip output. This matches
        // the preference order used by the prompt chip display (display.rs) and
        // agent footer (chips.rs).
        #[cfg(feature = "local_fs")]
        let from_model = self
            .git_status_metadata(ctx)
            .map(|metadata| GitLineChanges::from_diff_stats(&metadata.stats_against_head));
        #[cfg(not(feature = "local_fs"))]
        let from_model: Option<GitLineChanges> = None;

        from_model
            .or_else(|| {
                git_line_changes_from_chips(&self.current_prompt.as_ref(ctx).agent_view_chips(ctx))
            })
            .filter(|line_changes| {
                line_changes.files_changed > 0
                    || line_changes.lines_added > 0
                    || line_changes.lines_removed > 0
            })
    }
}

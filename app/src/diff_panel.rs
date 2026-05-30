use cuteui::{AppContext, Element, View, Entity, TypedActionView};
use cuteui::elements::{Rect, Stack, Flex, Empty, Text};
use cuteui::fonts::FamilyId;
use pathfinder_color::ColorU;
use std::sync::Arc;
use parking_lot::Mutex;
use std::path::PathBuf;

// Re-export git types for diff content
pub use crate::git::{DiffHunk, DiffLine, DiffLineKind};

const FONT_SIZE: f32 = 12.0;
const DEFAULT_FONT_FAMILY: FamilyId = FamilyId(0);

/// Diff 面板视图（右侧可收起面板）
pub struct DiffPanelView {
    model: Arc<Mutex<DiffPanelModel>>,
}

pub struct DiffPanelModel {
    /// 是否可见
    visible: bool,
    /// 当前 diff 内容
    diff_content: Option<DiffContent>,
    /// 面板宽度
    width: f32,
    /// 当前 hunk 索引
    current_hunk: usize,
    /// 工作目录
    work_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct DiffContent {
    pub file_path: String,
    pub old_content: Vec<String>,
    pub new_content: Vec<String>,
    pub hunks: Vec<DiffHunk>,
}

impl DiffPanelModel {
    pub fn new() -> Self {
        Self {
            visible: false,
            diff_content: None,
            width: 300.0,
            current_hunk: 0,
            work_dir: None,
        }
    }

    pub fn load_diff(&mut self, file_path: &PathBuf) {
        if let Some(work_dir) = &self.work_dir {
            if let Some(diff_str) = crate::git::get_file_diff(work_dir, file_path) {
                let hunks = crate::git::parse_diff_hunks(&diff_str);
                self.diff_content = Some(DiffContent {
                    file_path: file_path.display().to_string(),
                    old_content: Vec::new(),
                    new_content: Vec::new(),
                    hunks,
                });
            } else {
                self.diff_content = None;
            }
        }
    }

    pub fn load_staged_diff(&mut self) {
        if let Some(work_dir) = &self.work_dir {
            if let Some(diff_str) = crate::git::get_staged_diff(work_dir) {
                let hunks = crate::git::parse_diff_hunks(&diff_str);
                self.diff_content = Some(DiffContent {
                    file_path: "Staged changes".to_string(),
                    old_content: Vec::new(),
                    new_content: Vec::new(),
                    hunks,
                });
            } else {
                self.diff_content = None;
            }
        }
    }

    pub fn load_all_diff(&mut self) {
        if let Some(work_dir) = &self.work_dir {
            if let Some(diff_str) = crate::git::get_all_diff(work_dir) {
                let hunks = crate::git::parse_diff_hunks(&diff_str);
                self.diff_content = Some(DiffContent {
                    file_path: "All changes".to_string(),
                    old_content: Vec::new(),
                    new_content: Vec::new(),
                    hunks,
                });
            } else {
                self.diff_content = None;
            }
        }
    }

    pub fn next_hunk(&mut self) {
        if let Some(content) = &self.diff_content {
            if self.current_hunk < content.hunks.len() - 1 {
                self.current_hunk += 1;
            }
        }
    }

    pub fn prev_hunk(&mut self) {
        if self.current_hunk > 0 {
            self.current_hunk -= 1;
        }
    }
}

impl DiffPanelView {
    pub fn new() -> Self {
        Self {
            model: Arc::new(Mutex::new(DiffPanelModel::new())),
        }
    }

    pub fn is_visible(&self) -> bool {
        self.model.lock().visible
    }

    pub fn set_work_dir(&self, dir: PathBuf) {
        self.model.lock().work_dir = Some(dir);
    }

    pub fn load_diff(&self, file_path: &PathBuf) {
        self.model.lock().load_diff(file_path);
    }

    pub fn load_staged_diff(&self) {
        self.model.lock().load_staged_diff();
    }

    pub fn load_all_diff(&self) {
        self.model.lock().load_all_diff();
    }
}

impl View for DiffPanelView {
    fn ui_name() -> &'static str {
        "DiffPanelView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        let model = self.model.lock();
        let visible = model.visible;
        let diff_content = model.diff_content.clone();
        let current_hunk = model.current_hunk;
        drop(model);

        if !visible {
            return Empty::new().finish();
        }

        // 面板背景
        let background = Rect::new()
            .with_background_color(ColorU::new(35, 35, 35, 255))
            .finish();

        // 标题栏背景
        let title_bg = Rect::new()
            .with_background_color(ColorU::new(45, 45, 45, 255))
            .finish();

        // 标题文本
        let title_text = if let Some(ref content) = diff_content {
            Text::new(content.file_path.clone(), DEFAULT_FONT_FAMILY, FONT_SIZE)
                .with_color(ColorU::new(200, 200, 200, 255))
                .finish()
        } else {
            Text::new("No diff".to_string(), DEFAULT_FONT_FAMILY, FONT_SIZE)
                .with_color(ColorU::new(150, 150, 150, 255))
                .finish()
        };

        // 标题栏布局
        let mut title_bar = Flex::column();
        title_bar.extend(vec![title_bg, title_text]);
        let title_bar = title_bar.finish();

        // Diff 内容区
        let content = if let Some(ref content) = diff_content {
            let mut diff_layout = Flex::column();

            // 显示 hunk 信息
            let hunk_info = format!(
                "Hunk {}/{}",
                current_hunk + 1,
                content.hunks.len().max(1)
            );
            let hunk_text = Text::new(hunk_info, DEFAULT_FONT_FAMILY, FONT_SIZE)
                .with_color(ColorU::new(100, 180, 255, 255))
                .finish();
            diff_layout.extend(vec![hunk_text]);

            // 显示当前 hunk 的内容
            if let Some(hunk) = content.hunks.get(current_hunk) {
                for line in &hunk.lines {
                    let bg_color = match line.kind {
                        DiffLineKind::Added => ColorU::new(40, 60, 40, 255),
                        DiffLineKind::Removed => ColorU::new(60, 40, 40, 255),
                        DiffLineKind::Context => ColorU::new(35, 35, 35, 255),
                    };

                    let text_color = match line.kind {
                        DiffLineKind::Added => ColorU::new(100, 200, 100, 255),
                        DiffLineKind::Removed => ColorU::new(200, 100, 100, 255),
                        DiffLineKind::Context => ColorU::new(180, 180, 180, 255),
                    };

                    let line_bg = Rect::new()
                        .with_background_color(bg_color)
                        .finish();

                    let line_text = Text::new(line.content.clone(), DEFAULT_FONT_FAMILY, FONT_SIZE)
                        .with_color(text_color)
                        .finish();

                    let mut line_layout = Flex::row();
                    line_layout.extend(vec![line_bg, line_text]);
                    diff_layout.extend(vec![line_layout.finish()]);
                }
            }

            diff_layout.finish()
        } else {
            // 空状态
            let empty_text = Text::new("No changes".to_string(), DEFAULT_FONT_FAMILY, FONT_SIZE)
                .with_color(ColorU::new(150, 150, 150, 255))
                .finish();

            let mut empty_layout = Flex::column();
            empty_layout.extend(vec![empty_text]);
            empty_layout.finish()
        };

        // 主布局
        let mut layout = Flex::column();
        layout.extend(vec![title_bar, content]);
        let layout = layout.finish();

        // 叠加背景
        let mut stack = Stack::new();
        stack.extend(vec![background, layout]);
        stack.finish()
    }
}

impl Entity for DiffPanelView {
    type Event = DiffPanelEvent;
}

impl TypedActionView for DiffPanelView {
    type Action = DiffPanelAction;
}

#[derive(Debug, Clone)]
pub enum DiffPanelEvent {
    Toggle,
    SetContent(DiffContent),
    Clear,
    Resize(f32),
    NextHunk,
    PrevHunk,
    LoadFile(PathBuf),
    LoadStaged,
    LoadAll,
}

#[derive(Debug, Clone)]
pub enum DiffPanelAction {
    AcceptChanges,
    RejectChanges,
    NextHunk,
    PrevHunk,
}

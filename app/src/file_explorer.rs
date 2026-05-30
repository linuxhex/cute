use cuteui::{AppContext, Element, View, Entity, TypedActionView};
use cuteui::elements::{Rect, Stack, Flex, Empty, Text};
use cuteui::fonts::FamilyId;
use pathfinder_color::ColorU;
use std::sync::Arc;
use parking_lot::Mutex;
use std::path::PathBuf;

const FONT_SIZE: f32 = 12.0;
const DEFAULT_FONT_FAMILY: FamilyId = FamilyId(0);

/// 文件浏览器侧边栏
pub struct FileExplorerSidebar {
    model: Arc<Mutex<FileExplorerModel>>,
}

pub struct FileExplorerModel {
    /// 当前目录
    current_dir: PathBuf,
    /// 文件列表
    files: Vec<FileItem>,
    /// 选中的文件索引
    selected_index: Option<usize>,
    /// 是否展开
    expanded: bool,
    /// 错误信息
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub is_dir: bool,
    pub path: PathBuf,
    pub size: Option<u64>,
}

impl FileExplorerModel {
    pub fn new() -> Self {
        let mut model = Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("~")),
            files: Vec::new(),
            selected_index: None,
            expanded: true,
            error: None,
        };
        model.refresh();
        model
    }

    pub fn refresh(&mut self) {
        self.files.clear();
        self.error = None;

        // 展开路径（处理 ~ 等）
        let dir = if self.current_dir.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                let current_str = self.current_dir.to_string_lossy();
                let rest = current_str.strip_prefix("~").unwrap_or("");
                home.join(rest.strip_prefix("/").unwrap_or(rest))
            } else {
                self.current_dir.clone()
            }
        } else {
            self.current_dir.clone()
        };

        match std::fs::read_dir(&dir) {
            Ok(entries) => {
                let mut files: Vec<FileItem> = Vec::new();
                let mut dirs: Vec<FileItem> = Vec::new();

                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let path = entry.path();
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let size = if !is_dir {
                        entry.file_type().ok().and_then(|t| {
                            if t.is_file() {
                                std::fs::metadata(&path).ok().map(|m| m.len())
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    };

                    let item = FileItem { name, is_dir, path, size };

                    if is_dir {
                        dirs.push(item);
                    } else {
                        files.push(item);
                    }
                }

                // 排序：目录在前，按名称排序
                dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

                // 添加父目录项
                if dir.parent().is_some() {
                    self.files.push(FileItem {
                        name: "..".to_string(),
                        is_dir: true,
                        path: dir.parent().unwrap().to_path_buf(),
                        size: None,
                    });
                }

                self.files.extend(dirs);
                self.files.extend(files);
            }
            Err(e) => {
                self.error = Some(format!("无法读取目录: {}", e));
            }
        }
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.selected_index = None;
        self.refresh();
    }

    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected_index = None;
            self.refresh();
        }
    }

    pub fn select_next(&mut self) {
        if self.files.is_empty() {
            return;
        }
        match self.selected_index {
            Some(idx) if idx < self.files.len() - 1 => {
                self.selected_index = Some(idx + 1);
            }
            None => {
                self.selected_index = Some(0);
            }
            _ => {}
        }
    }

    pub fn select_prev(&mut self) {
        if self.files.is_empty() {
            return;
        }
        match self.selected_index {
            Some(idx) if idx > 0 => {
                self.selected_index = Some(idx - 1);
            }
            None => {
                self.selected_index = Some(self.files.len() - 1);
            }
            _ => {}
        }
    }

    pub fn open_selected(&mut self) -> Option<PathBuf> {
        let idx = self.selected_index?;
        let item = self.files.get(idx)?;
        if item.is_dir {
            self.navigate_to(item.path.clone());
            None
        } else {
            Some(item.path.clone())
        }
    }
}

impl FileExplorerSidebar {
    pub fn new() -> Self {
        Self {
            model: Arc::new(Mutex::new(FileExplorerModel::new())),
        }
    }

    pub fn current_dir(&self) -> PathBuf {
        self.model.lock().current_dir.clone()
    }
}

impl View for FileExplorerSidebar {
    fn ui_name() -> &'static str {
        "FileExplorerSidebar"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn Element> {
        let model = self.model.lock();
        let files = model.files.clone();
        let selected = model.selected_index;
        let current_dir = model.current_dir.display().to_string();
        let error = model.error.clone();
        drop(model);

        // 侧边栏背景
        let background = Rect::new()
            .with_background_color(ColorU::new(40, 40, 40, 255))
            .finish();

        // 标题栏背景
        let title_bg = Rect::new()
            .with_background_color(ColorU::new(50, 50, 50, 255))
            .finish();

        // 当前目录文本
        let title_text = Text::new(current_dir, DEFAULT_FONT_FAMILY, FONT_SIZE)
            .with_color(ColorU::new(200, 200, 200, 255))
            .finish();

        // 标题栏布局
        let mut title_bar = Flex::column();
        title_bar.extend(vec![title_bg, title_text]);
        let title_bar = title_bar.finish();

        // 文件列表区域
        let mut file_list = Flex::column();

        // 显示错误（如果有）
        if let Some(e) = error {
            let error_text = Text::new(e, DEFAULT_FONT_FAMILY, FONT_SIZE)
                .with_color(ColorU::new(255, 100, 100, 255))
                .finish();
            file_list.extend(vec![error_text]);
        } else {
            // 显示文件列表
            for (idx, file) in files.iter().enumerate() {
                let is_selected = selected == Some(idx);
                let bg_color = if is_selected {
                    ColorU::new(60, 60, 80, 255)
                } else {
                    ColorU::new(40, 40, 40, 255)
                };

                // 文件项背景
                let item_bg = Rect::new()
                    .with_background_color(bg_color)
                    .finish();

                // 图标和名称
                let icon = if file.is_dir { "📁" } else { "📄" };
                let display_name = format!("{} {}", icon, file.name);
                let text_color = if file.is_dir {
                    ColorU::new(100, 180, 255, 255)
                } else {
                    ColorU::new(200, 200, 200, 255)
                };

                let file_text = Text::new(display_name, DEFAULT_FONT_FAMILY, FONT_SIZE)
                    .with_color(text_color)
                    .finish();

                // 文件项布局
                let mut item_layout = Flex::row();
                item_layout.extend(vec![item_bg, file_text]);
                file_list.extend(vec![item_layout.finish()]);
            }
        }
        let file_list = file_list.finish();

        // 主布局
        let mut layout = Flex::column();
        layout.extend(vec![title_bar, file_list]);
        let layout = layout.finish();

        // 叠加背景
        let mut stack = Stack::new();
        stack.extend(vec![background, layout]);
        stack.finish()
    }
}

impl Entity for FileExplorerSidebar {
    type Event = FileExplorerEvent;
}

impl TypedActionView for FileExplorerSidebar {
    type Action = FileExplorerAction;
}

#[derive(Debug, Clone)]
pub enum FileExplorerEvent {
    SelectFile(usize),
    OpenFile(PathBuf),
    NavigateUp,
    NavigateDown,
    Refresh,
}

#[derive(Debug, Clone)]
pub enum FileExplorerAction {
    OpenFile(PathBuf),
    CreateFile(String),
    CreateDir(String),
}

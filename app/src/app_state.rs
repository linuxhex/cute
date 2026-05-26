use std::collections::HashMap;
use std::sync::Arc;

use cuteui::{AppContext, SingletonEntity as _};

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub windows: Vec<WindowSnapshot>,
    pub active_window_index: Option<usize>,
}

impl AppState {
    pub fn new(_ctx: &mut AppContext) -> Self {
        Self {
            windows: Vec::new(),
            active_window_index: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WindowSnapshot {
    pub tabs: Vec<TabSnapshot>,
    pub active_tab_index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabSnapshot {
    pub title: String,
}

use cuteui::{AppContext};
use cuteui::units::{Lines, Pixels};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
mod view;

pub use view::TerminalView;

pub fn init(_app: &mut AppContext) {
    // Initialize terminal module
}

/// Minimum number of visible lines.
const MIN_ROWS: usize = 1;

/// Minimum number of columns.
const MIN_COLUMNS: usize = 2;

/// Terminal size info.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeInfo {
    pane_width_px: f32,
    pane_height_px: f32,
    rows: usize,
    columns: usize,
    cell_width_px: Pixels,
    cell_height_px: Pixels,
    padding_x_px: Pixels,
    padding_y_px: Pixels,
}

impl SizeInfo {
    pub fn new_without_font_metrics(rows: usize, cols: usize) -> Self {
        SizeInfo {
            pane_width_px: cols as f32,
            pane_height_px: rows as f32,
            rows,
            columns: cols,
            cell_width_px: Pixels::zero(),
            cell_height_px: Pixels::zero(),
            padding_x_px: Pixels::zero(),
            padding_y_px: Pixels::zero(),
        }
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn columns(&self) -> usize {
        self.columns
    }
}

/// The padding around each block.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BlockPadding {
    pub padding_top: f32,
    pub command_padding_top: f32,
    pub middle: f32,
    pub bottom: f32,
}

impl BlockPadding {
    pub fn new(
        padding_top: f32,
        command_padding_top: f32,
        padding_middle: f32,
        padding_bottom: f32,
    ) -> Self {
        BlockPadding {
            padding_top,
            command_padding_top,
            middle: padding_middle,
            bottom: padding_bottom,
        }
    }
}

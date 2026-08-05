use crate::code::buffer_location::LocalOrRemotePath;

/// The current state of a code review.
#[derive(Debug, Clone, Default)]
pub struct CodeReview {
    /// Comments that are currently pending (have yet to be addressed).
    pub pending_comments: Vec<ReviewComment>,
    /// Comments that have been addressed.
    pub addressed_comments: Vec<ReviewComment>,
}

impl CodeReview {
    pub fn new_with_pending_comments(pending_comments: Vec<ReviewComment>) -> Self {
        Self {
            pending_comments,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReviewComment {
    pub id: CommentId,
    pub content: String,
    pub diff: ReviewDiff,
    pub head_title: Option<String>,
}

impl ReviewComment {
    pub fn title(&self) -> String {
        match (&self.diff.file_path, self.diff.line_number) {
            (Some(file_path), Some(line_number)) => {
                let path_component = file_path.path_component();
                let file_name = path_component.file_name().unwrap_or("Invalid File Name");
                let display_line = line_number + 1;
                format!("{file_name}:{display_line}")
            }
            (Some(file_path), None) => {
                let path_component = file_path.path_component();
                let file_name = path_component.file_name().unwrap_or("Invalid File Name");
                file_name.to_string()
            }
            (None, _) => self
                .head_title
                .as_ref()
                .cloned()
                .unwrap_or_else(|| "Review Comment".to_string()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReviewDiff {
    pub file_path: Option<LocalOrRemotePath>,
    pub line_number: Option<usize>,
}

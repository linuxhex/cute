// Stub for code_review module (deleted from OSS build)
use serde::{Deserialize, Serialize};

pub mod code_review_view;
pub mod diff_state;
pub mod review_comment_batch;

#[derive(Debug, Clone)]
pub struct CodeReviewAction;
#[derive(Debug, Clone)]
pub struct CodeReviewContextDestination;
#[derive(Debug, Clone)]
pub struct CodeReviewPaneEntrypoint;
#[derive(Debug, Clone)]
pub struct CodeReviewView;
#[derive(Debug, Clone)]
pub struct CodeReviewViewEvent;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentId(pub String);
#[derive(Debug, Clone)]
pub struct CommentOrigin;
#[derive(Debug, Clone)]
pub struct CommentViewCard;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineType { Add, Delete, Context }
#[derive(Debug, Clone)]
pub struct DiffStateModel;
#[derive(Debug, Clone)]
pub struct GlobalCodeReviewModel;
#[derive(Debug, Clone)]
pub struct LineDiffContent;
#[derive(Debug, Clone)]
pub struct LocalDiffStateModel;
#[derive(Debug, Clone)]
pub struct ReviewCommentBatch;
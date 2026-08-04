use std::collections::{HashMap, HashSet};

use crate::ai::blocklist::block::CommentElementState;

#[derive(Copy, Clone)]
pub(super) struct Props<'a> {
    pub(super) comments: &'a HashMap<CommentId, CommentElementState>,
    pub(super) addressed_comment_ids: &'a HashSet<CommentId>,
}

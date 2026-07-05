use crate::search::command_palette::mixer::CommandPaletteItemAction;
use crate::search::data_source::{FuzzyMatchResult, QueryResult};
use crate::server::ids::ObjectUid;

#[derive(Debug, Clone)]
pub struct NotebookSearchItem {
    pub object_id: ObjectUid,
    pub match_result: FuzzyMatchResult,
}

impl NotebookSearchItem {
    pub fn view_id(&self) -> String {
        format!("notebook:{}", self.object_id)
    }

    pub fn to_query_result(&self) -> QueryResult {
        QueryResult {
            view_id: self.view_id(),
            action: CommandPaletteItemAction::None,
            match_result: self.match_result.clone(),
        }
    }
}
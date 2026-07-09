use cuteui::AppContext;

use crate::cloud_stub_types::Space;
use crate::search::data_source::{Query, QueryResult};
use crate::search::mixer::{DataSourceRunErrorWrapper, SyncDataSource};
use crate::search::notebook_embedding::searcher::EmbeddingSearchItemAction;

pub struct CloudWorkflowsDataSource;

impl CloudWorkflowsDataSource {
    pub fn new(_notebook_space: Space, _app: &mut AppContext) -> Self {
        Self
    }
}

impl SyncDataSource for CloudWorkflowsDataSource {
    type Action = EmbeddingSearchItemAction;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        Ok(vec![])
    }
}

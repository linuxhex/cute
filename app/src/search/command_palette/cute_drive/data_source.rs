use cuteui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::search::command_palette::mixer::CommandPaletteItemAction;
use crate::search::data_source::{DataSourceSearchError, Query, QueryResult};
use crate::search::mixer::DataSourceRunErrorWrapper;
use crate::search::QueryFilter;
use crate::server::ids::{ObjectUid, SyncId};
use crate::settings::AISettings;

pub struct DataSource {
    _searcher: Box<dyn WarpDriveSearcher>,
}

impl DataSource {
    #[cfg(not(target_family = "wasm"))]
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        Self::new_fuzzy(ctx)
    }

    #[cfg(target_family = "wasm")]
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        Self::new_fuzzy(ctx)
    }

    pub fn new_fuzzy(ctx: &mut ModelContext<Self>) -> Self {
        let searcher = Box::new(FuzzyWarpDriveSearcher::default());
        searcher.refresh_search_index(ctx).unwrap_or_else(|err| {
            log::error!("Error refreshing search index: {err:?}");
        });
        DataSource { _searcher: searcher }
    }

    #[cfg(not(target_family = "wasm"))]
    fn new_full_text(ctx: &mut ModelContext<Self>) -> Self {
        Self::new_fuzzy(ctx)
    }
}

impl Entity for DataSource {
    type View = cuteui::elements::Empty;
    fn view(&self, _ctx: &AppContext) -> Self::View {
        cuteui::elements::Empty::default()
    }
}

trait WarpDriveSearcher {
    fn refresh_search_index(&mut self, ctx: &mut ModelContext<DataSource>) -> Result<(), DataSourceSearchError>;
    fn search(&self, query: &Query, _filter: &QueryFilter, _ctx: &AppContext) -> Result<Vec<QueryResult>, DataSourceSearchError>;
}

#[derive(Default)]
struct FuzzyWarpDriveSearcher;

impl WarpDriveSearcher for FuzzyWarpDriveSearcher {
    fn refresh_search_index(&mut self, _ctx: &mut ModelContext<DataSource>) -> Result<(), DataSourceSearchError> {
        Ok(())
    }
    fn search(&self, _query: &Query, _filter: &QueryFilter, _ctx: &AppContext) -> Result<Vec<QueryResult>, DataSourceSearchError> {
        Ok(Vec::new())
    }
}

#[cfg(not(target_family = "wasm"))]
mod full_text_searcher {
    use super::*;
    pub struct FullTextWarpDriveSearcher;
    impl FullTextWarpDriveSearcher {
        pub fn new(_executor: cuteui::BackgroundExecutor) -> Self {
            FullTextWarpDriveSearcher
        }
    }
    impl WarpDriveSearcher for FullTextWarpDriveSearcher {
        fn refresh_search_index(&mut self, _ctx: &mut ModelContext<DataSource>) -> Result<(), DataSourceSearchError> {
            Ok(())
        }
        fn search(&self, _query: &Query, _filter: &QueryFilter, _ctx: &AppContext) -> Result<Vec<QueryResult>, DataSourceSearchError> {
            Ok(Vec::new())
        }
    }
}
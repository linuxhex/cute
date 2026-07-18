// Stub for notebook pane (cloud functionality removed)
use cuteui::{AppContext, ModelHandle, ViewContext, ViewHandle};
use super::view::PaneView;
use super::{
    DetachType, PaneConfiguration, PaneContent, PaneGroup, PaneId, ShareableLink, ShareableLinkError,
};
use crate::pane_group::focus_state::PaneFocusHandle;
use crate::local_storage_types::NotebookView;
use crate::app_state::LeafContents;

pub struct NotebookPane {
    view: ViewHandle<PaneView<NotebookView>>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl NotebookPane {
    pub fn from_view(notebook_view: ViewHandle<NotebookView>, ctx: &mut AppContext) -> Self {
        let pane_configuration = ctx.add_model(|_ctx| PaneConfiguration::new("Notebook"));
        let view = ctx.add_typed_action_view(notebook_view.window_id(ctx), |ctx| {
            let pane_id = PaneId::from_notebook_pane_ctx(ctx);
            PaneView::new(pane_id, notebook_view, (), pane_configuration.clone(), ctx)
        });
        Self {
            view,
            pane_configuration,
        }
    }

    pub fn restore(
        _notebook_id: Option<crate::server::ids::SyncId>,
        _settings: Option<crate::local_storage_types::OpenCuteDriveObjectSettings>,
        _ctx: &mut ViewContext<PaneGroup>,
    ) -> Result<Self, anyhow::Error> {
        // Stub - cloud notebook restore is not available
        Err(anyhow::anyhow!("Notebook restore not available in local mode"))
    }

    pub fn notebook_view(&self, ctx: &AppContext) -> ViewHandle<NotebookView> {
        self.view.as_ref(ctx).child(ctx)
    }
}

impl PaneContent for NotebookPane {
    fn id(&self) -> PaneId {
        PaneId::from_notebook_pane_view(&self.view)
    }

    fn attach(
        &self,
        _group: &PaneGroup,
        focus_handle: PaneFocusHandle,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        self.view
            .update(ctx, |view, ctx| view.set_focus_handle(focus_handle, ctx));
    }

    fn detach(&self, _group: &PaneGroup, _detach_type: DetachType, _ctx: &mut ViewContext<PaneGroup>) {
        // Stub implementation
    }

    fn snapshot(&self, _app: &AppContext) -> LeafContents {
        // Stub: return a placeholder notebook snapshot
        LeafContents::Notebook(crate::app_state::NotebookPaneSnapshot::CloudNotebook {
            notebook_id: Some(crate::server::ids::SyncId::ClientId(crate::server::ids::ClientId::new())),
            settings: Some(crate::local_storage_types::OpenCuteDriveObjectSettings::default()),
        })
    }

    fn has_application_focus(&self, ctx: &mut ViewContext<PaneGroup>) -> bool {
        self.view.is_self_or_child_focused(ctx)
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        let notebook_view = self.view.as_ref(ctx).child(ctx);
        notebook_view.update(ctx, |view, ctx| view.focus(ctx));
    }

    fn shareable_link(
        &self,
        _ctx: &mut ViewContext<PaneGroup>,
    ) -> Result<ShareableLink, ShareableLinkError> {
        Err(ShareableLinkError::Expected)
    }

    fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    fn is_pane_being_dragged(&self, _ctx: &AppContext) -> bool {
        false
    }
}

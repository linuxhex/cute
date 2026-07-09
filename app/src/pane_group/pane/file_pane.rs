use std::sync::Arc;

use cute_util::local_or_remote_path::LocalOrRemotePath;
use cuteui::{AppContext, ModelHandle, View, ViewContext, ViewHandle};

use super::view::PaneView;
use super::{
    DetachType, PaneConfiguration, PaneContent, PaneGroup, PaneId, ShareableLink,
    ShareableLinkError,
};
use crate::app_state::{FilePaneSnapshot, LeafContents};
#[cfg(feature = "local_fs")]
use crate::code::editor_management::CodeSource;
use crate::cloud_stub_types::{FileNotebookEvent, FileNotebookView};
use crate::terminal::model::session::Session;
use crate::workflows::WorkflowSelectionSource;

pub struct FilePane {
    view: ViewHandle<PaneView<FileNotebookView>>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl FilePane {
    fn from_view(file_view: ViewHandle<FileNotebookView>, ctx: &mut AppContext) -> Self {
        let pane_configuration = file_view.as_ref(ctx).pane_configuration();

        let view = ctx.add_typed_action_view(file_view.window_id(ctx), |ctx| {
            let pane_id = PaneId::from_file_pane_ctx(ctx);
            PaneView::new(pane_id, file_view, (), pane_configuration.clone(), ctx)
        });

        Self {
            view,
            pane_configuration,
        }
    }

    /// Create a new file notebook pane for the given path and optional target session. If `path`
    /// is `None`, the pane is created but left empty. For local paths without a target session,
    /// the pane waits for a local session to become active. Remote paths are loaded directly
    /// via the remote server.
    pub fn new<V: View>(
        path: Option<LocalOrRemotePath>,
        target_session: Option<Arc<Session>>,
        #[cfg(feature = "local_fs")] code_source: Option<CodeSource>,
        ctx: &mut ViewContext<V>,
    ) -> Self {
        let view = ctx.add_typed_action_view(move |ctx| {
            let mut view = FileNotebookView::new(ctx);
            #[cfg(feature = "local_fs")]
            view.set_code_source(code_source, ctx);

            if let Some(path) = path {
                view.open(path, target_session, ctx);
            }

            view
        });
        Self::from_view(view, ctx)
    }
    pub fn file_view(&self, ctx: &AppContext) -> ViewHandle<FileNotebookView> {
        self.view.as_ref(ctx).child(ctx)
    }
}

impl PaneContent for FilePane {
    fn id(&self) -> PaneId {
        PaneId::from_file_pane_view(&self.view)
    }

    fn attach(
        &self,
        _group: &PaneGroup,
        focus_handle: crate::pane_group::focus_state::PaneFocusHandle,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        self.view
            .update(ctx, |view, ctx| view.set_focus_handle(focus_handle, ctx));

        let pane_id = self.id();
        let file_view = self.file_view(ctx);

        ctx.subscribe_to_view(
            &self.file_view(ctx),
            move |pane_group, _, event, ctx| match event {
                FileNotebookEvent::RunWorkflow { workflow, source } => {
                    ctx.emit(crate::pane_group::Event::RunWorkflow {
                        workflow: workflow.clone(),
                        workflow_source: source.clone(),
                        workflow_selection_source: WorkflowSelectionSource::Notebook,
                        argument_override: None,
                    });
                }
                FileNotebookEvent::TitleUpdated => {
                    ctx.emit(crate::pane_group::Event::PaneTitleUpdated)
                }
                FileNotebookEvent::FileLoaded => {
                    ctx.emit(crate::pane_group::Event::AppStateChanged)
                }
                #[cfg(feature = "local_fs")]
                FileNotebookEvent::OpenFileWithTarget {
                    path,
                    target,
                    line_col,
                } => {
                    ctx.emit(crate::pane_group::Event::OpenFileWithTarget {
                        path: path.clone(),
                        target: target.clone(),
                        line_col: *line_col,
                    });
                }
                FileNotebookEvent::Pane(pane_event) => {
                    pane_group.handle_pane_event(pane_id, pane_event, ctx)
                }
                FileNotebookEvent::FileLinked(_) | FileNotebookEvent::FileUnlinked(_) => {
                    // No action needed for file link events
                }
            },
        );

        ctx.subscribe_to_view(&self.view, move |group, _, event, ctx| {
            group.handle_pane_view_event(pane_id, event, ctx);
        });
    }

    fn detach(
        &self,
        _group: &PaneGroup,
        _detach_type: DetachType,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        // Always unsubscribe from views and models
        let file_view = self.file_view(ctx);
        ctx.unsubscribe_to_view(&file_view);
        // Note: links() returns &NotebookLinks, not &ModelHandle<NotebookLinks>
        // We skip unsubscribing since the stub links model doesn't need it
        ctx.unsubscribe_to_view(&self.view);
    }

    fn snapshot(&self, app: &AppContext) -> LeafContents {
        // Only persist local file paths in session snapshots; remote files
        // are not restorable across sessions.
        let path = self.file_view(app).as_ref(app).local_path();
        LeafContents::File(FilePaneSnapshot { path })
    }

    fn has_application_focus(&self, ctx: &mut ViewContext<PaneGroup>) -> bool {
        self.view.is_self_or_child_focused(ctx)
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        self.file_view(ctx).update(ctx, |view, ctx| view.focus(ctx));
    }

    fn shareable_link(
        &self,
        _ctx: &mut ViewContext<PaneGroup>,
    ) -> Result<ShareableLink, ShareableLinkError> {
        Ok(ShareableLink::Base)
    }

    fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    fn is_pane_being_dragged(&self, ctx: &AppContext) -> bool {
        self.view.as_ref(ctx).is_being_dragged()
    }
}

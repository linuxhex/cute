// Minimal stub types for cloud functionality (drive, notebooks)
// These types provide minimal definitions to allow compilation without actual cloud functionality
// Merged from cloud_object module (removed)
#![allow(dead_code)]

use std::any::Any;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;

use chrono::{Duration, Utc};
use derivative::Derivative;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use url::Url;
use cute_core::channel::Channel;
use cuteui::{AppContext, View, ViewContext, ModelHandle, Entity, SingletonEntity, ViewHandle, WeakViewHandle, ModelContext};
use crate::pane_group::{BackingView, PaneEvent, focus_state::PaneFocusHandle, pane::view::{HeaderContent, HeaderRenderContext}};
use crate::editor::InteractionState;
use crate::ai::document::ai_document_model::AIDocumentId;
use cute_editor::selection::TextUnit;
use cuteui::text::word_boundaries::WordBoundariesPolicy;
use crate::search::ai_context_menu::mixer::AIContextMenuSearchableAction;
use crate::search::data_source::{Query, QueryResult};
use crate::search::mixer::{AsyncDataSource, DataSourceRunErrorWrapper, SyncDataSource};
// use crate::search::notebook_embedding::searcher::EmbeddingSearchItemAction; // Removed: cloud notebook functionality
use futures_util::future::BoxFuture;
use cute_editor::content::buffer::{Buffer, BufferEditAction, EditOrigin as CuteEditOrigin, InitialBufferState};
use cute_editor::content::selection_model::BufferSelectionModel;
use cute_editor::content::text::{BufferBlockStyle, IndentBehavior, IndentUnit};
use cute_editor::model::{CoreEditorModel, PlainTextEditorModel};
use cute_editor::render::model::RenderState;
use cute_editor::render::element::{DisplayOptions, DisplayStateHandle, DisplayState, RichTextElement, RichTextAction, VerticalExpansionBehavior};
use cute_editor::editor::EditorView as CuteEditorView;
use cute_editor::selection::SelectionModel;
use string_offset::CharOffset;
use cuteui::elements::Axis;
use cuteui::units::Pixels;

// ===== Cloud Object Submodules (moved from cloud_object) =====

pub mod breadcrumbs;
pub mod model;
pub mod models;
pub mod toast_message;

/// Prefix for generic string objects in cloud storage
pub const GENERIC_STRING_OBJECT_PREFIX: &str = "GENERIC_STRING_";

// ===== Re-export submodules for drive/cloud_object compatibility =====

/// Re-export items module (was crate::drive::items)
pub mod items {
    pub use crate::cloud_stub_types::{WarpDriveItemId, WarpDriveWorkflow, WarpDriveEnvVarCollection, WarpDriveAIFact, WarpDriveMCPServer, DriveObjectType};

    /// Re-export ai_fact submodule
    pub mod ai_fact {
        pub use crate::cloud_stub_types::WarpDriveAIFact;
    }

    /// Re-export mcp_server submodule
    pub mod mcp_server {
        pub use crate::cloud_stub_types::WarpDriveMCPServer;
    }

    /// Re-export workflow submodule
    pub mod workflow {
        pub use crate::cloud_stub_types::WarpDriveWorkflow;
    }

    /// Re-export env_var_collection submodule
    pub mod env_var_collection {
        pub use crate::cloud_stub_types::WarpDriveEnvVarCollection;
    }
}

/// Re-export sharing module (was crate::drive::sharing)
pub mod sharing {
    pub use crate::cloud_stub_types::{ShareableObject, SharingAccessLevel, SharingDialogSource, ContentEditability, SharingDialog, SharingDialogEvent};

    /// Re-export dialog submodule
    pub mod dialog {
        pub use crate::cloud_stub_types::{SharingDialog, SharingDialogEvent, SharingDialogSource};
    }
}

/// Re-export export module (was crate::drive::export)
pub mod export {
    pub use crate::cloud_stub_types::{ExportManager, safe_filename};
}

/// Re-export folders module (was crate::drive::folders)
pub mod folders {
    pub use crate::cloud_stub_types::models::CloudFolder;
    pub use crate::cloud_stub_types::models::CloudFolderModel;
    pub use crate::cloud_stub_types::FolderId;
}

/// Re-export settings module (was crate::drive::settings)
pub mod settings {
    pub use crate::cloud_stub_types::{WarpDriveSettings, WarpDriveSettingsChangedEvent, CuteDriveSettings, CuteDriveSettingsChangedEvent};
}

/// Re-export workflows module (was crate::drive::workflows)
pub mod workflows {
    pub use crate::cloud_stub_types::{ArgumentsState, WorkflowModal, WorkflowModalEvent, WorkflowArgSelector, WorkflowArgSelectorEvent, WorkflowArgSelectorStyles, EnumCreationDialog, EnumCreationDialogEvent, EnumData, WorkflowEnumData, GeneratedCommandMetadata, GeneratedCommandMetadataError, ArgumentEditorRowIndex, ArgumentTypeEditor, AliasArgumentSelector, AliasArgumentSelectorEvent};

    /// Re-export arguments submodule
    pub mod arguments {
        pub use crate::cloud_stub_types::ArgumentsState;
    }

    /// Re-export modal submodule
    pub mod modal {
        pub use crate::cloud_stub_types::{WorkflowModal, WorkflowModalEvent};
    }

    /// Re-export ai_assist submodule
    pub mod ai_assist {
        pub use crate::cloud_stub_types::{GeneratedCommandMetadata, GeneratedCommandMetadataError};
    }

    /// Re-export enum_creation_dialog submodule
    pub mod enum_creation_dialog {
        pub use crate::cloud_stub_types::{EnumCreationDialog, EnumCreationDialogEvent, WorkflowEnumData, EnumData};
    }

    /// Re-export workflow_arg_selector submodule
    pub mod workflow_arg_selector {
        pub use crate::cloud_stub_types::{WorkflowArgSelector, WorkflowArgSelectorEvent, WorkflowArgSelectorStyles};
    }

    /// Re-export workflow_arg_type_helpers submodule
    pub use crate::cloud_stub_types::workflow_arg_type_helpers;
}

/// Re-export cloud_object_styling module
pub mod cloud_object_styling {
    // COMMENTED: warp_drive_icon_color deprecated, use cute_drive_icon_color instead
    // pub use crate::cloud_stub_types::{warp_drive_icon_color, cute_drive_icon_color, DriveObjectType};
    pub use crate::cloud_stub_types::{cute_drive_icon_color, DriveObjectType};
}

/// Re-export drive_helpers module
pub mod drive_helpers {
    pub use crate::cloud_stub_types::{has_feature_gated_anonymous_user_reached_env_var_limit, has_feature_gated_anonymous_user_reached_workflow_limit};
}

/// Re-export panel module
pub mod panel {
    pub use crate::cloud_stub_types::{MIN_SIDEBAR_WIDTH, MAX_SIDEBAR_WIDTH_RATIO};
}

/// Re-export import module
pub mod import {
    /// Re-export modal submodule
    pub mod modal {
        pub use crate::cloud_stub_types::{ImportModal, ImportModalEvent};
    }
}

/// Re-export keys module (was crate::drive::keys / crate::notebooks::keys)
pub mod keys {
    use cuteui::{Entity, ModelContext, SingletonEntity};
    use crate::settings_view::keybindings::KeybindingChangedNotifier;

    /// Stub for notebook keybindings cache
    pub struct NotebookKeybindings {}

    impl Entity for NotebookKeybindings {
        type Event = ();
    }

    impl SingletonEntity for NotebookKeybindings {}

    impl NotebookKeybindings {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self {}
        }
    }
}

// Re-export from submodules (from cloud_object/mod.rs)
pub use self::model::generic_string_model::{GenericStringObjectId, GenericStringModel, Serializer, StringModel};
pub use self::model::persistence::CloudModel;
pub use crate::server::ids::HashedSqliteId;
pub use cute_server_client::cloud_object::*;
// Override the stub CloudObjectTypeAndId (empty struct) from cloud_object::models
// with the real enum from cute_server_client::drive
pub use cute_server_client::drive::CloudObjectTypeAndId;
pub use models::{
    ServerCloudObject, ServerFolder, ServerNotebook, ServerWorkflow,
    CloudFolder, CloudFolderModel, CloudNotebook, CloudNotebookModel, NotebookId,
};

// REMOVED: SharedSessionSource and related types - cloud feature disabled in local version
// pub use crate::terminal::model::terminal_model::{SharedSessionSource, SessionSourceType};

/// Minimal stub for SharedSessionSource
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SharedSessionSource {
    pub source_type: SessionSourceType,
    #[allow(dead_code)]
    pub source_task_id: Option<String>,
}

impl SharedSessionSource {
    pub fn ambient_agent(task_id: Option<String>) -> Self {
        Self {
            source_type: SessionSourceType::AmbientAgent { task_id: task_id.clone() },
            source_task_id: task_id,
        }
    }

    pub fn orchestrator_task_id(&self) -> Option<&str> {
        self.source_type.orchestrator_task_id()
    }
}

// Define missing type aliases
pub type OpenWarpDriveObjectSettings = OpenCuteDriveObjectSettings;

use self::breadcrumbs::ContainingObject;
use self::model::actions::ObjectActions;
use crate::appearance::Appearance;
use crate::channel::ChannelState;
// CuteDriveItem, CloudObjectTypeAndId, OpenCuteDriveObjectArgs are defined/re-exported within this module
use crate::persistence::ModelEvent;
use crate::server::ids::{HashableId, ObjectUid, ServerId, ToServerId};
use crate::util::time_format::format_approx_duration_from_now_utc;
pub use crate::workflows::{CloudWorkflow, WorkflowSource};

// Re-export EditorLayout from util::openable_file_type

// Import CloudObjectId from cute_server_client
pub use cute_server_client::persistence::CloudObjectId;
pub use cute_server_client::ids::SyncId;

// ===== Drive Types =====

/// Minimal stub for CuteDriveItemId
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CuteDriveItemId {
    Object(CloudObjectTypeAndId),
    Folder(String),
    AIFactCollection,
    EnvVarCollection,
    MCPServerCollection,
    Space(crate::server::ids::ServerId),
}

/// Re-export Owner from cute_server_client (already exported via pub use cute_server_client::cloud_object::*)


/// Minimal stub for DriveObjectType
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriveObjectType {
    Workflow,
    Notebook {
        is_ai_document: bool,
    },
    EnvVarCollection,
    AIFactCollection,
    AIFact,  // Added for compatibility
    MCPServerCollection,
    Folder,
    File,
    AgentModeWorkflow,
}

/// Minimal stub for DrivePanelEvent
#[derive(Clone, Debug)]
pub enum DrivePanelEvent {
    OpenWorkflowInPane(CloudObjectTypeAndId, WorkflowOpenMode),
    OpenNotebook(CloudObjectTypeAndId),
    OpenEnvVarCollection(CloudObjectTypeAndId),
    OpenAIFactCollection,
    OpenMCPServerCollection,
    FocusCuteDrive,
    AttachPlanAsContext(AIDocumentId),
    OpenWorkflowModalWithNew {
        owner: Owner,
        initial_folder_id: Option<String>,
    },
    OpenWorkflowModalWithCloudWorkflow(i32),
    RunWorkflow(CloudObjectTypeAndId),
    Open,
    Close,
    OpenSearch,
    InvokeEnvironmentVariables {
        env_var_collection: CloudObjectTypeAndId,
        in_subshell: bool,
    },
    OpenTeamSettingsPage,
    OpenImportModal {
        owner: Owner,
        initial_folder_id: Option<String>,
    },
}

/// Minimal stub for WorkflowOpenMode
#[derive(Clone, Debug, Default, PartialEq)]
pub enum WorkflowOpenMode {
    #[default]
    Default,
    Edit,
    Run,
}

/// Minimal stub for OpenCuteDriveObjectSettings
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OpenCuteDriveObjectSettings {
    pub open_mode: WorkflowOpenMode,
    pub focus_pane: bool,
    pub focused_folder_id: Option<crate::server::ids::ServerId>,
    // COMMENTED: Team invitation - invitee_email field
    // pub invitee_email: Option<String>,
    #[allow(dead_code)]
    invitee_email: Option<String>, // Simplified: kept for struct compatibility but not used
}

/// Minimal stub for CuteDriveSettings
#[derive(Clone, Debug, Default)]
pub struct CuteDriveSettings {
    pub enable_warp_drive: StubSettingsValue<bool>,
    pub sharing_onboarding_block_shown: StubSettingsValue<bool>,
}

/// Minimal stub for settings value that mimics SettingsValue behavior
#[derive(Clone, Debug, Default)]
pub struct StubSettingsValue<T> {
    value: T,
}

impl<T: Clone + Default> StubSettingsValue<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn set_value(&mut self, new_value: T, _ctx: &cuteui::AppContext) -> Result<(), anyhow::Error> {
        self.value = new_value;
        Ok(())
    }

    pub fn toggle_and_save_value(&mut self, _ctx: &cuteui::AppContext) -> Result<(), anyhow::Error>
    where T: std::ops::Not<Output = T>
    {
        self.value = !self.value.clone();
        Ok(())
    }
}

impl<T: Clone + Default> std::ops::Deref for StubSettingsValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Implement Entity trait for CuteDriveSettings
impl cuteui::Entity for CuteDriveSettings {
    type Event = CuteDriveSettingsChangedEvent;
}

/// Implement SingletonEntity trait for CuteDriveSettings
impl SingletonEntity for CuteDriveSettings {}

impl CuteDriveSettings {
    pub fn is_warp_drive_enabled(_app: &cuteui::AppContext) -> bool {
        false // Disabled in stub
    }

    pub fn handle(_ctx: &AppContext) -> ModelHandle<Self> {
        // For SingletonEntity, we can use the default implementation
        <Self as SingletonEntity>::handle(_ctx)
    }

    pub fn toggle_enable_warp_drive(&mut self, _ctx: &mut AppContext) -> Result<(), anyhow::Error> {
        let current = self.enable_warp_drive.value().clone();
        self.enable_warp_drive.set_value(!current, _ctx)?;
        Ok(())
    }

    pub fn as_ref(_ctx: &AppContext) -> &Self {
        // This is a stub, so we just return a static instance
        static INSTANCE: CuteDriveSettings = CuteDriveSettings {
            enable_warp_drive: StubSettingsValue { value: false },
            sharing_onboarding_block_shown: StubSettingsValue { value: false },
        };
        &INSTANCE
    }

    /// Stub method for registering settings
    pub fn register(ctx: &mut AppContext) {
        ctx.add_singleton_model(|_| CuteDriveSettings::default());
    }
}

/// Import CloudObjectTypeAndId from cute_server_client (already exported via pub use cute_server_client::cloud_object::*)


// ===== Notebook Types =====

/// Minimal stub for NotebookLinks
#[derive(Clone, Debug, Default)]
pub struct NotebookLinks {
    pub cells: Vec<String>,
}

/// Implement Entity trait for NotebookLinks
impl cuteui::Entity for NotebookLinks {
    type Event = LinkEvent;
}

/// NotebooksEditorModel using real cute_editor components for rendering.
#[derive(Clone, Debug)]
pub struct NotebooksEditorModel {
    render_state: ModelHandle<RenderState>,
    content: ModelHandle<Buffer>,
    buffer_selection_model: ModelHandle<BufferSelectionModel>,
    selection: ModelHandle<SelectionModel>,
    file_link_resolution_context: Option<FileLinkResolutionContext>,
    interaction_state: InteractionState,
}

/// Implement Entity trait for NotebooksEditorModel
impl cuteui::Entity for NotebooksEditorModel {
    type Event = RichTextEditorModelEvent;
}

/// Implement SingletonEntity for NotebooksEditorModel (for ModelHandle::as_ref compatibility)
impl SingletonEntity for NotebooksEditorModel {}

impl CoreEditorModel for NotebooksEditorModel {
    type T = NotebooksEditorModel;

    fn content(&self) -> &ModelHandle<Buffer> {
        &self.content
    }

    fn buffer_selection_model(&self) -> &ModelHandle<BufferSelectionModel> {
        &self.buffer_selection_model
    }

    fn selection_model(&self) -> &ModelHandle<SelectionModel> {
        &self.selection
    }

    fn render_state(&self) -> &ModelHandle<RenderState> {
        &self.render_state
    }

    fn validate(&self, _ctx: &impl cuteui::ModelAsRef) {
        // No-op for notebooks editor model
    }

    fn active_text_style(&self) -> cute_editor::content::text::TextStyles {
        cute_editor::content::text::TextStyles::default()
    }
}

impl PlainTextEditorModel for NotebooksEditorModel {}

impl NotebooksEditorModel {
    pub fn new(styles: cute_editor::render::model::RichTextStyles, _window_id: cuteui::WindowId, ctx: &mut ModelContext<Self>) -> Self {
        Self::new_internal(styles, ctx)
    }

    pub fn new_unbound(styles: cute_editor::render::model::RichTextStyles, ctx: &mut ModelContext<Self>) -> Self {
        Self::new_internal(styles, ctx)
    }

    fn new_internal(styles: cute_editor::render::model::RichTextStyles, ctx: &mut ModelContext<Self>) -> Self {
        let content = ctx.add_model(|_| {
            Buffer::new(Box::new(|block_style, _| match block_style {
                BufferBlockStyle::PlainText => IndentBehavior::TabIndent(IndentUnit::Space(4)),
                _ => IndentBehavior::Ignore,
            }))
        });
        ctx.subscribe_to_model(&content, |me, event, ctx| {
            me.handle_content_model_event(event, ctx);
        });

        let buffer_selection_model = ctx.add_model(|_ctx| BufferSelectionModel::new(content.clone()));

        let render_state = ctx.add_model(|ctx| {
            RenderState::new(styles, false, None, ctx)
        });
        ctx.subscribe_to_model(&render_state, |me, event, ctx| {
            me.handle_render_state_model_event(event, ctx);
        });

        let selection = ctx.add_model(|ctx| {
            SelectionModel::new(
                content.clone(),
                render_state.clone(),
                buffer_selection_model.clone(),
                None,
                ctx,
            )
        });

        Self {
            render_state,
            content,
            buffer_selection_model,
            selection,
            file_link_resolution_context: None,
            interaction_state: InteractionState::Editable,
        }
    }

    fn handle_content_model_event(
        &mut self,
        event: &cute_editor::content::buffer::BufferEvent,
        ctx: &mut ModelContext<Self>,
    ) {
        use cute_editor::content::buffer::BufferEvent;
        match event {
            BufferEvent::ContentChanged { delta, buffer_version, .. } => {
                self.render_state.update(ctx, |render_state, _ctx| {
                    render_state.add_pending_edit(delta.clone(), *buffer_version);
                });
            }
            BufferEvent::SelectionChanged { .. } => {}
            BufferEvent::ContentReplaced { .. } => {}
            BufferEvent::AnchorUpdated { .. } => {}
        }
    }

    fn handle_render_state_model_event(
        &mut self,
        _event: &cute_editor::render::model::RenderEvent,
        _ctx: &mut ModelContext<Self>,
    ) {
        // No-op for now
    }

    pub fn set_default_mermaid_display_mode(&mut self, _mode: MarkdownDisplayMode, _ctx: &cuteui::AppContext) {
        // No-op for now
    }

    pub fn set_window_id(&mut self, _window_id: cuteui::WindowId, _ctx: &cuteui::AppContext) {
        // No-op for now
    }

    pub fn set_file_link_resolution_context(&mut self, ctx: Option<FileLinkResolutionContext>, _app: &cuteui::AppContext) {
        self.file_link_resolution_context = ctx;
    }

    pub fn file_link_resolution_context(&self) -> Option<&FileLinkResolutionContext> {
        self.file_link_resolution_context.as_ref()
    }

    pub fn selected_text(&self, ctx: &cuteui::AppContext) -> Option<String> {
        let buffer = self.content.as_ref(ctx);
        Some(buffer.selected_text_as_plain_text(self.buffer_selection_model.clone(), ctx).into_string())
    }

    pub fn is_empty(&self, ctx: &cuteui::AppContext) -> bool {
        self.content.as_ref(ctx).is_empty()
    }

    pub fn markdown_content(&self) -> String {
        // This is used by content() which returns a String, not ModelHandle<Buffer>
        // We'll keep this as a convenience method
        String::new()
    }

    pub fn content(&self) -> String {
        self.markdown_content()
    }

    pub fn markdown(&self, ctx: &cuteui::AppContext) -> String {
        self.content.as_ref(ctx).markdown()
    }

    pub fn markdown_unescaped(&self, ctx: &cuteui::AppContext) -> String {
        self.content.as_ref(ctx).markdown_unescaped()
    }

    pub fn update_to_new_markdown(&mut self, markdown: &str, ctx: &mut ModelContext<Self>) {
        CoreEditorModel::update_content(self, |mut wrapper, ctx| {
            wrapper.buffer().reset_undo_stack();
            wrapper.apply_edit(
                BufferEditAction::ReplaceWith(InitialBufferState::markdown(markdown)),
                CuteEditOrigin::SystemEdit,
                self.buffer_selection_model.clone(),
                ctx,
            );
        }, ctx);
        self.validate(ctx);
    }

    pub fn reset_with_markdown(&mut self, markdown: &str, ctx: &mut ModelContext<Self>) {
        CoreEditorModel::update_content(self, |mut wrapper, ctx| {
            wrapper.buffer().reset_undo_stack();
            wrapper.apply_edit(
                BufferEditAction::ReplaceWith(InitialBufferState::markdown(markdown)),
                CuteEditOrigin::SystemEdit,
                self.buffer_selection_model.clone(),
                ctx,
            );
        }, ctx);
        self.validate(ctx);
    }

    pub fn apply_diffs(&mut self, _diffs: Vec<ai::diff_validation::DiffDelta>, _ctx: &cuteui::AppContext) {
        // Stub implementation
    }

    /// Read operation
    pub fn read<R>(&self, ctx: &cuteui::AppContext, f: impl FnOnce(&Self, &cuteui::AppContext) -> R) -> R {
        f(self, ctx)
    }

    /// Update operation (ModelHandle compatible)
    pub fn update(&self, ctx: &cuteui::AppContext, f: impl FnOnce(&mut Self, &cuteui::AppContext)) {
        // This is called via ModelHandle::update which provides ModelContext, so this
        // method shouldn't normally be used. Kept for compatibility.
    }

    pub fn set_interaction_state(&mut self, state: InteractionState, _ctx: &cuteui::AppContext) {
        self.interaction_state = state;
    }

    pub fn interaction_state(&self) -> InteractionState {
        self.interaction_state
    }
}

/// RichTextEditorView using real cute_editor rendering.
#[derive(Clone)]
pub struct RichTextEditorView {
    model: ModelHandle<NotebooksEditorModel>,
    self_handle: WeakViewHandle<Self>,
    display_state: DisplayStateHandle,
}

/// Implement Entity trait for RichTextEditorView
impl cuteui::Entity for RichTextEditorView {
    type Event = EditorViewEvent;
}

impl std::fmt::Debug for RichTextEditorView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RichTextEditorView")
            .field("model", &self.model)
            .finish_non_exhaustive()
    }
}

/// Action type for RichTextEditorView (minimal — most actions are no-ops)
#[derive(Clone, Debug)]
pub enum RichTextEditorViewAction {
    ScrollVertical(Pixels),
    ScrollHorizontal(Pixels),
}

impl RichTextAction<RichTextEditorView> for RichTextEditorViewAction {
    fn scroll(delta: Pixels, axis: Axis) -> Option<Self> {
        Some(match axis {
            Axis::Vertical => RichTextEditorViewAction::ScrollVertical(delta),
            Axis::Horizontal => RichTextEditorViewAction::ScrollHorizontal(delta),
        })
    }

    fn user_typed(_chars: String, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn vim_user_typed(_chars: String, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn left_mouse_down(_location: cute_editor::render::model::Location, _modifiers: cuteui::event::ModifiersState, _click_count: u32, _is_first_mouse: bool, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn left_mouse_dragged(_location: cute_editor::render::model::Location, _cmd: bool, _shift: bool, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn left_mouse_up(_location: cute_editor::render::model::Location, _cmd: bool, _shift: bool, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Vec<Self> {
        vec![]
    }

    fn mouse_hovered(_location: Option<cute_editor::render::model::Location>, _parent_view: &WeakViewHandle<RichTextEditorView>, _cmd: bool, _is_covered: bool, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn task_list_clicked(_block_start: CharOffset, _parent_view: &WeakViewHandle<RichTextEditorView>, _ctx: &AppContext) -> Option<Self> {
        None
    }

    fn middle_mouse_down(_ctx: &AppContext) -> Option<Self> {
        None
    }
}

impl CuteEditorView for RichTextEditorView {
    type RichTextAction = RichTextEditorViewAction;

    fn runnable_command_at<'a>(&self, _block_offset: CharOffset, _ctx: &'a AppContext) -> Option<&'a dyn cute_editor::editor::RunnableCommandModel> {
        None
    }

    fn embedded_item_at<'a>(&self, _block_offset: CharOffset, _ctx: &'a AppContext) -> Option<&'a dyn cute_editor::editor::EmbeddedItemModel> {
        None
    }
}

/// Implement View trait for RichTextEditorView
impl View for RichTextEditorView {
    fn ui_name() -> &'static str {
        "RichTextEditorView"
    }

    fn render(&self, ctx: &AppContext) -> Box<dyn cuteui::Element> {
        let model = self.model.as_ref(ctx);
        let render_state = model.render_state().clone();

        let is_editable = matches!(model.interaction_state(), InteractionState::Editable);

        let display_options = DisplayOptions {
            editable: is_editable,
            blink_cursors: false,
            debug_bounds: false,
            hovered_block_start: None,
            focused: false,
            left_gutter: 0.,
            right_gutter: 0.,
            vertical_expansion_behavior: VerticalExpansionBehavior::default(),
        };

        let element = RichTextElement::<Self>::new(
            render_state,
            self.self_handle.clone(),
            display_options,
            self.display_state.clone(),
            None,
            vec![],
        );

        Box::new(element)
    }
}

/// Implement TypedActionView for RichTextEditorView
impl cuteui::TypedActionView for RichTextEditorView {
    type Action = RichTextEditorViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            RichTextEditorViewAction::ScrollVertical(delta) => {
                let render_state = self.model.as_ref(ctx).render_state().clone();
                render_state.update(ctx, |rs, ctx| {
                    rs.scroll(*delta, ctx);
                });
            }
            RichTextEditorViewAction::ScrollHorizontal(delta) => {
                let render_state = self.model.as_ref(ctx).render_state().clone();
                render_state.update(ctx, |rs, ctx| {
                    rs.scroll_horizontal(*delta, ctx);
                });
            }
        }
    }
}

impl RichTextEditorView {
    pub fn new(
        _view_position_id: String,
        model: ModelHandle<NotebooksEditorModel>,
        _links: ModelHandle<NotebookLinks>,
        _config: RichTextEditorConfig,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let self_handle = ctx.handle();
        let display_state = Arc::new(DisplayState::default());
        Self { model, self_handle, display_state }
    }

    pub fn model(&self) -> ModelHandle<NotebooksEditorModel> {
        self.model.clone()
    }

    pub fn set_interaction_state(&mut self, state: InteractionState, ctx: &mut ViewContext<Self>) {
        self.model.update(ctx, |m, inner_ctx| {
            m.set_interaction_state(state, inner_ctx);
        });
    }



    pub fn markdown_unescaped(&self, ctx: &cuteui::AppContext) -> String {
        self.model.as_ref(ctx).markdown_unescaped(ctx)
    }

    pub fn selected_text(&self, ctx: &cuteui::AppContext) -> Option<String> {
        self.model.as_ref(ctx).selected_text(ctx)
    }

    pub fn clear_text_selection(&mut self, _ctx: &mut ViewContext<Self>) {
        // No-op for now
    }

    pub fn reset_with_markdown(&mut self, markdown: &str, ctx: &mut ViewContext<Self>) {
        self.model.update(ctx, |m, ctx| {
            m.reset_with_markdown(markdown, ctx);
        });
    }
}

/// Minimal stub for EditorViewEvent
#[derive(Clone, Debug)]
pub enum EditorViewEvent {
    Edited,
    Focused,
    OpenFile {
        path: std::path::PathBuf,
        line_and_column_num: Option<cute_util::path::LineAndColumnArg>,
        force_open_in_warp: bool,
    },
    TextSelectionChanged,
    ContentChanged,
    Saved,
    CmdEnter,
    EscapePressed,
}

/// Minimal stub for NotebookLocation
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotebookLocation {
    PersonalCloud,
    TeamCloud(String),
}

/// Minimal stub for PaneLocator - actually use PaneViewLocator from workspace
pub use crate::workspace::PaneViewLocator as PaneLocator;


/// Minimal stub for FileLinkResolutionContext
#[derive(Clone, Debug, Default)]
pub struct FileLinkResolutionContext {
    pub workspace_root: Option<String>,
    pub working_directory: Option<String>,
    pub shell_launch_data: Option<crate::terminal::ShellLaunchData>,
}

/// Minimal stub for NotebookManager
#[derive(Clone, Debug)]
pub struct NotebookManager {}

/// Implement Entity trait for NotebookManager
impl cuteui::Entity for NotebookManager {
    type Event = ();
}

/// Implement SingletonEntity trait for NotebookManager
impl SingletonEntity for NotebookManager {}

impl NotebookManager {
    pub fn new(_ctx: &cuteui::AppContext) -> Self {
        Self {}
    }

    pub fn handle(_ctx: &AppContext) -> ModelHandle<Self> {
        <Self as SingletonEntity>::handle(_ctx)
    }

    pub fn open_notebook(&mut self, _source: &NotebookSource, _ctx: &mut ViewContext<Self>) -> Option<ViewHandle<NotebookView>> {
        None
    }

    /// Stub method for finding an existing pane
    pub fn find_pane(&self, _source: &NotebookSource) -> Option<(cuteui::WindowId, PaneLocator)> {
        None
    }

    /// Stub method for creating a new pane - removed, returns empty result
    pub fn create_pane(
        &mut self,
        _source: &NotebookSource,
        _settings: &OpenCuteDriveObjectSettings,
        _window_id: cuteui::WindowId,
        _ctx: &mut cuteui::ModelContext<Self>,
    ) {
        // Cloud notebook pane creation removed
    }

    /// Stub method for reset
    pub fn reset(&mut self, _ctx: &cuteui::AppContext) {
        // Stub implementation
    }

    /// Stub method for closing all notebooks
    pub fn close_notebooks(&mut self, _ctx: &cuteui::AppContext) {
        // Stub implementation
    }
}

/// Minimal stub for NotebookEditor
#[derive(Clone, Debug)]
pub struct NotebookEditor {}

/// Minimal stub for ArgumentsState
#[derive(Clone, Debug, Default)]
pub struct ArgumentsState {
    pub arguments: Vec<crate::cloud_stub_types::models::workflow::Argument>,
    pub invalid_arguments_char_ranges: Vec<std::ops::Range<usize>>,
    pub valid_arguments_char_ranges_and_arg_index: Vec<(std::ops::Range<usize>, usize)>,
}

impl ArgumentsState {
    pub fn for_command_workflow(_arguments_state: &ArgumentsState, _command: String) -> Self {
        Self {
            arguments: Vec::new(),
            invalid_arguments_char_ranges: Vec::new(),
            valid_arguments_char_ranges_and_arg_index: Vec::new(),
        }
    }

    pub fn for_saved_prompt(_arguments_state: &ArgumentsState, _content: String) -> Self {
        Self {
            arguments: Vec::new(),
            invalid_arguments_char_ranges: Vec::new(),
            valid_arguments_char_ranges_and_arg_index: Vec::new(),
        }
    }
}

// ===== Utility Functions and Constants =====

/// Placeholder workspace UID for local mode
pub const PLACEHOLDER_WORKSPACE_UID: &str = "local-workspace";

/// Argument editor row index wrapper
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct ArgumentEditorRowIndex(pub usize);

/// Minimum sidebar width
pub const MIN_SIDEBAR_WIDTH: f32 = 200.0;

/// Maximum sidebar width ratio
pub const MAX_SIDEBAR_WIDTH_RATIO: f32 = 0.4;

/// Stub function for checking if file is markdown
pub fn is_markdown_file(_path: impl AsRef<std::path::Path>) -> bool {
    false
}

/// Stub function for rich text styles
pub struct RichTextStyles {
    pub base_line_height: f32,
    pub header_text_color: pathfinder_color::ColorU,
    pub text_color: pathfinder_color::ColorU,
    pub border_color: pathfinder_color::ColorU,
    pub cell_padding: f32,
    pub inline_code_style: InlineCodeStyle,
    pub outer_border: pathfinder_color::ColorU,
    pub column_dividers: pathfinder_color::ColorU,
    pub row_dividers: pathfinder_color::ColorU,
    pub header_background: pathfinder_color::ColorU,
    pub cell_background: pathfinder_color::ColorU,
    pub alternate_row_background: pathfinder_color::ColorU,
    pub base_text: Option<String>,
    pub block_spacings: Vec<f32>,
    pub show_placeholder_text_on_empty_block: bool,
    pub minimum_paragraph_height: f32,
    pub cursor_width: f32,
    pub highlight_urls: bool,
}

impl RichTextStyles {
    pub fn base_line_height(&self) -> f32 {
        self.base_line_height
    }
}

pub struct InlineCodeStyle {
    pub background: pathfinder_color::ColorU,
    pub foreground: pathfinder_color::ColorU,
    pub font_color: pathfinder_color::ColorU,
}

pub fn rich_text_styles(appearance: &cute_core::ui::appearance::Appearance, _font_settings: &crate::settings::FontSettings) -> cute_editor::render::model::RichTextStyles {
    use cute_editor::render::model::*;
    use cuteui::elements::{Fill, Border};
    use cuteui::units::{Lines, Pixels};
    use cute_core::ui::theme::color::internal_colors;
    let theme = appearance.theme();
    let bg_fill = theme.background();
    let text_color: pathfinder_color::ColorU = theme.main_text_color(bg_fill).into_solid();
    let bg_color: pathfinder_color::ColorU = bg_fill.into_solid();
    let ui_font_size = appearance.ui_font_size();
    let line_height = Pixels::new(ui_font_size * appearance.line_height_ratio());
    let base_text = ParagraphStyles {
        font_family: appearance.ui_font_family(),
        font_size: ui_font_size,
        font_weight: Default::default(),
        line_height_ratio: appearance.line_height_ratio(),
        text_color,
        baseline_ratio: 0.8,
        fixed_width_tab_size: None,
    };
    let code_text = ParagraphStyles {
        font_family: appearance.monospace_font_family(),
        font_size: appearance.monospace_font_size(),
        font_weight: Default::default(),
        line_height_ratio: appearance.line_height_ratio(),
        text_color,
        baseline_ratio: 0.8,
        fixed_width_tab_size: Some(4),
    };
    let code_background: Fill = internal_colors::fg_overlay_1(theme).into();
    let selection_fill = Fill::Solid(pathfinder_color::ColorU::new(0x40, 0x80, 0xff, 80));
    let cursor_fill = Fill::Solid(text_color);
    let sub_text_color: pathfinder_color::ColorU = theme.sub_text_color(bg_fill).into_solid();
    let inline_code_style = InlineCodeStyle {
        font_family: appearance.monospace_font_family(),
        background: internal_colors::fg_overlay_1(theme).into_solid(),
        font_color: text_color,
    };
    RichTextStyles {
        base_text,
        code_text,
        code_background,
        embedding_background: bg_fill.into(),
        embedding_text: base_text.clone(),
        code_border: Border::all(0.).with_border_color(pathfinder_color::ColorU::new(0, 0, 0, 0)),
        placeholder_color: sub_text_color,
        selection_fill,
        cursor_fill,
        inline_code_style,
        check_box_style: CheckBoxStyle {
            border_width: 1.,
            border_color: sub_text_color,
            icon_path: "",
            background: pathfinder_color::ColorU::new(0, 0, 0, 0),
            hover_background: pathfinder_color::ColorU::new(0, 0, 0, 0),
        },
        horizontal_rule_style: HorizontalRuleStyle {
            rule_height: 1.,
            color: sub_text_color,
        },
        broken_link_style: BrokenLinkStyle {
            icon_path: "",
            icon_color: sub_text_color,
        },
        block_spacings: BlockSpacings::default(),
        minimum_paragraph_height: Some(Lines::new(1.).to_pixels(line_height)),
        show_placeholder_text_on_empty_block: true,
        cursor_width: 2.,
        highlight_urls: true,
        table_style: TableStyle {
            border_color: sub_text_color,
            header_background: internal_colors::fg_overlay_2(theme).into_solid(),
            cell_background: pathfinder_color::ColorU::new(0, 0, 0, 0),
            alternate_row_background: None,
            text_color,
            header_text_color: text_color,
            scrollbar_nonactive_thumb_color: sub_text_color,
            scrollbar_active_thumb_color: sub_text_color,
            font_family: appearance.ui_font_family(),
            font_size: ui_font_size,
            cell_padding: 4.,
            outer_border: true,
            column_dividers: true,
            row_dividers: false,
        },
    }
}

/// Stub function for safe filename
pub fn safe_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_")
}

/// Stub function for markdown table appearance
/// Minimal stub for TableAppearance
#[derive(Clone, Debug, Default)]
pub struct TableAppearance {
    pub header_text_color: pathfinder_color::ColorU,
    pub text_color: pathfinder_color::ColorU,
    pub border_color: pathfinder_color::ColorU,
    pub cell_padding: f32,
    pub outer_border: bool,
    pub column_dividers: bool,
    pub row_dividers: bool,
    pub header_background: pathfinder_color::ColorU,
    pub cell_background: pathfinder_color::ColorU,
    pub alternate_row_background: Option<pathfinder_color::ColorU>,
}

pub fn markdown_table_appearance(_appearance: &cute_core::ui::appearance::Appearance) -> TableAppearance {
    TableAppearance::default()
}

/// Stub function for word unit
pub fn word_unit(_ctx: &cuteui::AppContext) -> TextUnit {
    TextUnit::Word(WordBoundariesPolicy::Default)
}

// ===== Missing Types for Compilation =====

/// Minimal stub for CuteDriveWorkflow
#[derive(Clone, Debug)]
pub struct CuteDriveWorkflow {
    pub id: crate::server::ids::SyncId,
}

impl CuteDriveWorkflow {
    pub fn new(
        _type_and_id: CloudObjectTypeAndId,
        _workflow: crate::workflows::CloudWorkflow,
        _is_agent_mode: bool,
    ) -> Self {
        Self {
            id: _type_and_id.sync_id(),
        }
    }
}

impl CuteDriveItem for CuteDriveWorkflow {
    fn id(&self) -> CuteDriveItemId {
        CuteDriveItemId::Object(CloudObjectTypeAndId::Workflow(self.id.clone()))
    }

    fn display_name(&self) -> Option<String> {
        Some("Workflow".to_string())
    }

    fn icon(&self) -> Option<cute_core::ui::Icon> {
        None
    }

    fn icon_color(&self, _appearance: &cute_core::ui::appearance::Appearance) -> Option<pathfinder_color::ColorU> {
        None
    }
}

/// Minimal stub for CuteDriveEnvVarCollection
#[derive(Clone, Debug)]
pub struct CuteDriveEnvVarCollection {
    pub id: crate::server::ids::SyncId,
}

impl CuteDriveEnvVarCollection {
    pub fn new(
        _type_and_id: CloudObjectTypeAndId,
        _env_var_collection: crate::cloud_stub_types::models::CloudEnvVarCollection,
    ) -> Self {
        Self {
            id: _type_and_id.sync_id(),
        }
    }
}

/// Minimal stub for CuteDriveAIFact
#[derive(Clone, Debug)]
pub struct CuteDriveAIFact {}

impl CuteDriveAIFact {
    pub fn new(_type_and_id: CloudObjectTypeAndId, _display_name: String) -> Self {
        Self {}
    }
}

impl CuteDriveItem for CuteDriveAIFact {
    fn id(&self) -> CuteDriveItemId {
        CuteDriveItemId::AIFactCollection
    }

    fn display_name(&self) -> Option<String> {
        Some("AI Fact".to_string())
    }

    fn icon(&self) -> Option<cute_core::ui::Icon> {
        None
    }

    fn icon_color(&self, _appearance: &cute_core::ui::appearance::Appearance) -> Option<pathfinder_color::ColorU> {
        None
    }
}

/// Minimal stub for CuteDriveMCPServer
#[derive(Clone, Debug)]
pub struct CuteDriveMCPServer {}

impl CuteDriveMCPServer {
    pub fn new(_type_and_id: CloudObjectTypeAndId, _name: String) -> Self {
        Self {}
    }
}

impl CuteDriveItem for CuteDriveMCPServer {
    fn id(&self) -> CuteDriveItemId {
        CuteDriveItemId::MCPServerCollection
    }

    fn display_name(&self) -> Option<String> {
        Some("MCP Server".to_string())
    }

    fn icon(&self) -> Option<cute_core::ui::Icon> {
        None
    }

    fn icon_color(&self, _appearance: &cute_core::ui::appearance::Appearance) -> Option<pathfinder_color::ColorU> {
        None
    }
}

impl CuteDriveItem for CuteDriveEnvVarCollection {
    fn id(&self) -> CuteDriveItemId {
        CuteDriveItemId::EnvVarCollection
    }

    fn display_name(&self) -> Option<String> {
        Some("Env Var Collection".to_string())
    }

    fn icon(&self) -> Option<cute_core::ui::Icon> {
        None
    }

    fn icon_color(&self, _appearance: &cute_core::ui::appearance::Appearance) -> Option<pathfinder_color::ColorU> {
        None
    }
}

/// Minimal stub for DriveIndexVariant
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriveIndexVariant {
    Personal,
    Team,
    Shared,
}

/// Minimal stub for TeamMetadata
#[derive(Clone, Debug, Default)]
pub struct TeamMetadata {
    pub name: String,
    pub uid: crate::server::ids::ServerId,
    pub billing_metadata: Option<BillingMetadata>,
    pub members: Vec<TeamMember>,
    pub organization_settings: Option<OrganizationSettings>,
}

/// Minimal stub for BillingMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BillingMetadata {}

/// Minimal stub for OrganizationSettings
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OrganizationSettings {}

impl TeamMetadata {
    pub fn from_local_cache(_ctx: &cuteui::AppContext) -> Self {
        Self::default()
    }

    pub fn uid(&self) -> &crate::server::ids::ServerId {
        &self.uid
    }

    pub fn has_admin_permissions(&self, email: &str) -> bool {
        self.members.iter().any(|member| {
            member.email == email && (member.role == MembershipRole::Admin || member.role == MembershipRole::Owner)
        })
    }
}

/// Minimal stub for NotebookView
#[derive(Clone, Debug)]
pub struct NotebookView {}

/// Implement Entity trait for NotebookView
impl cuteui::Entity for NotebookView {
    type Event = crate::pane_group::PaneEvent;
}

/// Implement SingletonEntity trait for NotebookView
impl SingletonEntity for NotebookView {
    fn as_ref(_ctx: &AppContext) -> &Self {
        // Stub - returns a static instance
        static INSTANCE: NotebookView = NotebookView {};
        &INSTANCE
    }
}

impl NotebookView {
    pub fn notebook_id(&self, _ctx: &AppContext) -> Option<crate::server::ids::SyncId> {
        None // Stub - always returns None in stub mode
    }

    pub fn is_plan(&self, _ctx: &AppContext) -> bool {
        false // Stub - always returns false
    }

    /// Stub method for getting selected text
    pub fn selected_text(&self, _ctx: &AppContext) -> Option<String> {
        None // Stub - always returns None
    }

    /// Stub method for as_ref operation
    pub fn as_ref(&self, _ctx: &AppContext) -> &Self {
        self
    }

    pub fn focus(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

/// Minimal stub for OpenCuteDriveObjectArgs
#[derive(Clone, Debug, PartialEq)]
pub struct OpenCuteDriveObjectArgs {
    pub object_type: crate::cloud_stub_types::ObjectType,
    pub server_id: crate::server::ids::ServerId,
    pub settings: OpenCuteDriveObjectSettings,
}

/// Minimal stub for FileNotebookView
#[derive(Clone, Debug)]
pub struct FileNotebookView {
    pane_configuration: ModelHandle<crate::pane_group::pane::PaneConfiguration>,
}

impl FileNotebookView {
    /// Stub method for compatibility
    pub fn path(&self) -> Option<cute_util::local_or_remote_path::LocalOrRemotePath> {
        None
    }

    pub fn new(ctx: &mut cuteui::ViewContext<Self>) -> Self {
        let pane_configuration = ctx.add_model(|_ctx| crate::pane_group::pane::PaneConfiguration::new("Notebook"));
        Self { pane_configuration }
    }

    pub fn pane_configuration(&self) -> ModelHandle<crate::pane_group::pane::PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn set_code_source(&mut self, _source: Option<crate::code::editor_management::CodeSource>, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open(&mut self, _path: cute_util::local_or_remote_path::LocalOrRemotePath, _target_session: Option<std::sync::Arc<crate::terminal::model::session::Session>>, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open_static(&mut self, _title: &str, _content: &str, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn links(&self) -> &crate::cloud_stub_types::NotebookLinks {
        // Stub implementation - returns a reference to a static instance
        static LINKS: crate::cloud_stub_types::NotebookLinks = crate::cloud_stub_types::NotebookLinks {
            cells: Vec::new(),
        };
        &LINKS
    }

    pub fn local_path(&self) -> Option<std::path::PathBuf> {
        None
    }

    pub fn focus(&mut self, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }
}

/// Minimal action enum for FileNotebookView
#[derive(Debug, Clone)]
pub enum FileNotebookViewAction {}

impl cuteui::TypedActionView for FileNotebookView {
    type Action = FileNotebookViewAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation - no actions to handle
    }
}

/// Minimal stub for workflow_arg_type_helpers module
pub mod workflow_arg_type_helpers {
    use super::*;
    use crate::cloud_stub_types::models::workflow::Argument;
    use crate::workflows::workflow_view::argument_editor::ArgumentEditorRow;

    pub use super::{ArgumentEditorRowIndex, ArgumentTypeEditor};

    pub fn create_enum(
        _enum_data: EnumData,
        _all_workflow_enums: &mut std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>,
        _arguments_rows: &[ArgumentEditorRow],
        _pending_argument_editor_row: &mut Option<ArgumentEditorRowIndex>,
        _ctx: &cuteui::AppContext,
    ) {
        // Stub implementation
    }

    pub fn edit_enum(
        _enum_data: &EnumData,
        _did_visibility_change: bool,
        _all_workflow_enums: &mut std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>,
        _arguments_rows: &[ArgumentEditorRow],
        _pending_argument_editor_row: &mut Option<ArgumentEditorRowIndex>,
        _ctx: &cuteui::AppContext,
    ) -> EnumData {
        EnumData::default()
    }

    pub fn load_enum(
        _id: usize,
        _all_workflow_enums: &std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>,
        _enum_creation_dialog: &cuteui::ViewHandle<EnumCreationDialog>,
        _ctx: &cuteui::AppContext,
    ) -> bool {
        false
    }

    pub fn extract_typed_argument_from_selector(
        _argument: &Argument,
        _description: Option<String>,
        _type_selector: &WorkflowArgSelector,
        _text_editor: Option<&cuteui::ViewHandle<crate::editor::EditorView>>,
        _ctx: &cuteui::AppContext,
    ) -> Option<Argument> {
        None
    }

    pub fn save_enum(_enum_data: &WorkflowEnumData, _owner: Option<cute_server_client::cloud_object::Owner>, _ctx: &cuteui::AppContext) {}

    pub fn load_workflow_enums_with_owner(_owner: cute_server_client::cloud_object::Owner, _ctx: &cuteui::AppContext) -> std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData> {
        std::collections::HashMap::new()
    }

    pub fn load_argument_into_selector(
        _selector: &mut WorkflowArgSelector,
        _argument: &Argument,
        _all_workflow_enums: &mut std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>,
        _ctx: &cuteui::AppContext,
    ) {
        // Stub implementation
    }
}

/// Minimal stub for CuteDriveItem trait
pub trait CuteDriveItem: std::fmt::Debug + Send + Sync {
    fn id(&self) -> CuteDriveItemId;
    fn display_name(&self) -> Option<String>;
    fn icon(&self) -> Option<cute_core::ui::Icon>;
    fn icon_color(&self, _appearance: &cute_core::ui::appearance::Appearance) -> Option<pathfinder_color::ColorU>;
    fn sync_status_icon(
        &self,
        _is_dequeueing: bool,
        _mouse_state: cuteui::elements::MouseStateHandle,
        _appearance: &cute_core::ui::appearance::Appearance,
    ) -> Option<cuteui::elements::Icon> {
        None
    }
}

/// Minimal stub for RichTextEditorModelEvent
#[derive(Clone, Debug)]
pub enum RichTextEditorModelEvent {
    Edited,
    Saved,
    FocusChanged,
    TextSelectionChanged,
    ContentChanged(CuteEditOrigin),
}

/// Stub functions for warp drive icon colors
pub fn warp_drive_icon_color(_appearance: &cute_core::ui::appearance::Appearance, _object_type: DriveObjectType) -> pathfinder_color::ColorU {
    pathfinder_color::ColorU::new(128, 128, 128, 255)
}

pub fn cute_drive_icon_color(_appearance: &cute_core::ui::appearance::Appearance, _object_type: DriveObjectType) -> pathfinder_color::ColorU {
    pathfinder_color::ColorU::new(128, 128, 128, 255)
}

/// Stub function for feature gated anonymous user limit checks
pub fn has_feature_gated_anonymous_user_reached_env_var_limit(_ctx: &cuteui::AppContext) -> bool {
    false
}

pub fn has_feature_gated_anonymous_user_reached_workflow_limit(_ctx: &cuteui::AppContext) -> bool {
    false
}

// ===== Additional Stub Types =====

/// Minimal stub for WorkflowModalEvent
#[derive(Clone, Debug)]
pub enum WorkflowModalEvent {
    ViewInCuteDrive(CloudObjectTypeAndId),
    ViewInWarpDrive(CloudObjectTypeAndId),
    AiAssistUpgradeError(String, String),
    OpenFromCuteDrive(CloudObjectTypeAndId, OpenCuteDriveObjectSettings),
    Close,
    CreateWorkflow,
    AiAssistError(String),
    UpdatedWorkflow(String),
}

/// Minimal stub for EmbeddingSearchItemAction (used in search data source)
#[derive(Clone, Debug)]
pub enum EmbeddingSearchItemAction {
    Open(String),
    Copy(String),
}

/// Minimal stub for NotebookDataSource
#[derive(Clone, Debug)]
pub struct NotebookDataSource {}

impl NotebookDataSource {
    /// Stub constructor - accepts a boolean parameter for compatibility
    pub fn new(_is_ai_context: bool) -> Self {
        Self {}
    }
}

impl Entity for NotebookDataSource {
    type Event = ();
}

impl SyncDataSource for NotebookDataSource {
    type Action = AIContextMenuSearchableAction;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        // Stub implementation - returns empty results
        Ok(Vec::new())
    }
}

impl AsyncDataSource for NotebookDataSource {
    type Action = crate::search::command_search::searcher::CommandSearchItemAction;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> BoxFuture<'static, Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper>> {
        // Stub implementation - returns empty results
        Box::pin(futures_util::future::ready(Ok(Vec::new())))
    }
}

/// Minimal stub for notebooks_data_source function
pub fn notebooks_data_source() -> NotebookDataSource {
    NotebookDataSource {}
}

/// Minimal stub for CloudNotebooksDataSource
#[derive(Clone, Debug)]
pub struct CloudNotebooksDataSource {}

impl CloudNotebooksDataSource {
    /// Stub constructor
    pub fn new(_embedding_space: crate::cloud_stub_types::Space, _ctx: &AppContext) -> Self {
        Self {}
    }
}

impl Entity for CloudNotebooksDataSource {
    type Event = ();
}

impl SyncDataSource for CloudNotebooksDataSource {
    type Action = EmbeddingSearchItemAction;

    fn run_query(
        &self,
        _query: &Query,
        _app: &AppContext,
    ) -> Result<Vec<QueryResult<Self::Action>>, DataSourceRunErrorWrapper> {
        // Stub implementation - returns empty results
        Ok(Vec::new())
    }
}

// ===== Sharing and Collaboration Types =====

/// Minimal stub for ShareableObject
#[derive(Clone, Debug)]
pub enum ShareableObject {
    AIConversation(String),
    WarpDriveObject(CloudObjectTypeAndId),
    CuteDriveObject(String),
    Notebook(String),
    Workflow(String),
    EnvVarCollection(String),
    #[allow(dead_code)]
    Session {
        #[allow(dead_code)]
        handle: cuteui::EntityId,
        session_id: String,
        #[allow(dead_code)]
        started_at: chrono::DateTime<chrono::Local>,
    },
}

impl ShareableObject {
    pub fn link(&self, _ctx: &cuteui::AppContext) -> Option<String> {
        None
    }
}

/// Minimal stub for SharingDialogSource
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SharingDialogSource {
    ConversationList,
    // COMMENTED: Team invitation - InviteeRequest variant
    // InviteeRequest,
    WorkflowView,
    DrivePanel,
    PaneHeader,
    CommandPalette,
    StartedSessionShare,
    AIBlockContextMenu,
    OnboardingBlock,
}

/// Minimal stub for DrivePanel (view type)
#[derive(Clone, Debug)]
pub struct DrivePanel {}

/// Implement Entity trait for DrivePanel
impl cuteui::Entity for DrivePanel {
    type Event = ();
}

impl DrivePanel {
    pub fn reset_focused_index_in_warp_drive(&mut self, _should_scroll: bool, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn scroll_item_into_view(&mut self, _item_id: CuteDriveItemId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn expand_section_for_drive_item_id(&mut self, _item_id: &CuteDriveItemId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn initialize_drive_section_states(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn reset_and_open_to_main_index(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn set_focused_item(&mut self, _item_id: &CuteDriveItemId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open_object_sharing_settings(&mut self, _object_id: &CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn move_object_to_team_owner(&mut self, _cloud_object_type_and_id: CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn set_focused_index(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn set_selected_object(&mut self, _id: Option<CuteDriveItemId>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open_cloud_object_dialog(
        &mut self,
        _object_type: DriveObjectType,
        _space: crate::cloud_stub_types::Space,
        _initial_folder_id: Option<String>,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }

    pub fn undo_trash(&mut self, _cloud_object_type_and_id: CloudObjectTypeAndId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn create_workflow_with_content(
        &mut self,
        _space: crate::cloud_stub_types::Space,
        _initial_folder_id: Option<String>,
        _content: String,
        _is_for_agent_mode: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }
}

/// Minimal stub for SharingAccessLevel
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SharingAccessLevel {
    Owner,
    Editor,
    Edit,  // Added for compatibility
    Viewer,
    View,  // Added for compatibility
    Full,
}

impl SharingAccessLevel {
    pub fn can_trash(&self) -> bool {
        matches!(self, SharingAccessLevel::Owner)
    }
}

impl From<cute_graphql::object_permissions::AccessLevel> for SharingAccessLevel {
    fn from(level: cute_graphql::object_permissions::AccessLevel) -> Self {
        match level {
            cute_graphql::object_permissions::AccessLevel::Editor => SharingAccessLevel::Editor,
            cute_graphql::object_permissions::AccessLevel::Full => SharingAccessLevel::Full,
            cute_graphql::object_permissions::AccessLevel::Viewer => SharingAccessLevel::Viewer,
        }
    }
}

/// Minimal stub for SharingDialog
#[derive(Clone, Debug)]
pub struct SharingDialog {
    target: Option<ShareableObject>,
}

impl cuteui::Entity for SharingDialog {
    type Event = SharingDialogEvent;
}

impl cuteui::View for SharingDialog {
    fn ui_name() -> &'static str {
        "SharingDialog"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for SharingDialog
impl cuteui::TypedActionView for SharingDialog {
    type Action = ();
}

impl SharingDialog {
    pub fn new(_target: Option<ShareableObject>, _ctx: &mut ViewContext<Self>) -> Self {
        Self {
            target: None,
        }
    }

    pub fn set_target(&mut self, _target: Option<ShareableObject>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn report_open(&mut self, _source: SharingDialogSource, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    pub fn has_shared_session_target(&self) -> bool {
        false
    }

    pub fn editability(&self) -> ContentEditability {
        ContentEditability::ReadOnly
    }

    pub fn copy_link(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn show_qr_code(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn is_unsharable_conversation(&self) -> bool {
        false
    }
}

/// Minimal stub for SharingDialogEvent
#[derive(Clone, Debug)]
pub enum SharingDialogEvent {
    Close,
    UpdateAccessLevel(SharingAccessLevel),
    Share,
}

/// Minimal stub for WorkflowArgSelector
#[derive(Clone, Debug)]
pub struct WorkflowArgSelector {}

/// Minimal stub for AliasArgumentSelector (cloud feature)
#[derive(Clone, Debug)]
pub struct AliasArgumentSelector {}

/// Minimal stub for AliasArgumentSelectorEvent (cloud feature)
#[derive(Clone, Debug)]
pub enum AliasArgumentSelectorEvent {
    ValueSet(String),
    Navigate(cute_editor::editor::NavigationKey),
}

/// Minimal stub for AliasArgumentSelectorAction (cloud feature)
#[derive(Clone, Debug)]
pub enum AliasArgumentSelectorAction {}

/// Implement Entity trait for AliasArgumentSelector
impl cuteui::Entity for AliasArgumentSelector {
    type Event = AliasArgumentSelectorEvent;
}

/// Implement View trait for AliasArgumentSelector
impl cuteui::View for AliasArgumentSelector {
    fn ui_name() -> &'static str {
        "AliasArgumentSelector"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for AliasArgumentSelector
impl cuteui::TypedActionView for AliasArgumentSelector {
    type Action = AliasArgumentSelectorAction;
}

/// Implement new method for AliasArgumentSelector
impl AliasArgumentSelector {
    pub fn new() -> Self {
        Self {}
    }
}

/// Minimal stub for AliasBar (cloud feature)
#[derive(Clone, Debug)]
pub struct AliasBar {}

/// Minimal stub for AliasBarEvent (cloud feature)
#[derive(Clone, Debug)]
pub enum AliasBarEvent {
    SelectedAliasChanged,
    AliasesUpdated,
}

/// Minimal stub for AliasBarAction (cloud feature)
#[derive(Clone, Debug)]
pub enum AliasBarAction {}

/// Implement Entity trait for AliasBar
impl cuteui::Entity for AliasBar {
    type Event = AliasBarEvent;
}

/// Implement View trait for AliasBar
impl cuteui::View for AliasBar {
    fn ui_name() -> &'static str {
        "AliasBar"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for AliasBar
impl cuteui::TypedActionView for AliasBar {
    type Action = AliasBarAction;
}

/// Implement methods for AliasBar
impl AliasBar {
    pub fn new(_workflow_id: crate::server::ids::SyncId, _ctx: &mut cuteui::ViewContext<Self>) -> Self {
        Self {}
    }

    pub fn set_workflow_id(&mut self, _id: crate::server::ids::SyncId, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn set_current_argument_value(&mut self, _name: &str, _value: String, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn set_current_env_vars(&mut self, _id: Option<crate::server::ids::SyncId>, _ctx: &mut cuteui::ViewContext<Self>) {
        // Stub implementation
    }

    pub fn save(&mut self, _ctx: &mut cuteui::ViewContext<Self>) -> Result<(), String> {
        Ok(())
    }

    pub fn get_all_argument_values(&self) -> Vec<String> {
        Vec::new()
    }

    // 删除：云端功能已禁用
    pub fn has_selected_alias(&self) -> bool {
        false
    }

    // 删除：云端功能已禁用
    pub fn has_unsaved_changes(&self) -> bool {
        false
    }
}

/// Minimal stub for SyntaxHighlightable (cloud feature)
#[derive(Clone, Debug)]
pub struct SyntaxHighlightable {}

/// Minimal stub for SyntaxHighlightableEvent (cloud feature)
#[derive(Clone, Debug)]
pub enum SyntaxHighlightableEvent {}

/// Implement Entity trait for SyntaxHighlightable
impl cuteui::Entity for SyntaxHighlightable {
    type Event = SyntaxHighlightableEvent;
}

/// Implement methods for SyntaxHighlightable
impl SyntaxHighlightable {
    pub fn new(_editor: ViewHandle<crate::editor::EditorView>, _ctx: &mut cuteui::ModelContext<Self>) -> Self {
        Self {}
    }

    // 删除：云端功能已禁用
    pub fn highlight_syntax(&mut self, _ctx: &mut cuteui::ModelContext<Self>) {
        // Stub implementation
    }

    // 删除：云端功能已禁用
    pub fn debounce_highlight(&mut self) {
        // Stub implementation
    }
}

/// Implement Entity trait for WorkflowArgSelector
impl cuteui::Entity for WorkflowArgSelector {
    type Event = WorkflowArgSelectorEvent;
}

/// Implement View trait for WorkflowArgSelector
impl View for WorkflowArgSelector {
    fn ui_name() -> &'static str {
        "WorkflowArgSelector"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for WorkflowArgSelector
impl cuteui::TypedActionView for WorkflowArgSelector {
    type Action = ();
}

impl WorkflowArgSelector {
    pub fn new(
        _styles: WorkflowArgSelectorStyles,
        _enums: &std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {}
    }

    pub fn set_workflow_enums(&mut self, _enums: &std::collections::HashMap<crate::server::ids::SyncId, WorkflowEnumData>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn is_dirty(&self) -> bool {
        false
    }

    pub fn get_selected_enum(&self) -> Option<crate::server::ids::SyncId> {
        None
    }

    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn enable(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn disable(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn get_created_enums(&self) -> Vec<crate::server::ids::SyncId> {
        Vec::new()
    }

    pub fn clear_created_enums(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn text_editor(&self) -> Option<&cuteui::ViewHandle<crate::editor::EditorView>> {
        None
    }
}

/// Minimal stub for WorkflowArgSelectorStyles
#[derive(Clone)]
pub struct WorkflowArgSelectorStyles {
    pub editor_padding: cuteui::ui_components::components::Coords,
    pub height: Option<f32>,
    pub width: Option<f32>,
    pub dropdown_background: Arc<dyn Fn(&Appearance) -> pathfinder_color::ColorU + Send + Sync>,
    pub border_color: Arc<dyn Fn(&Appearance) -> pathfinder_color::ColorU + Send + Sync>,
    pub border_radius: f32,
}

impl Default for WorkflowArgSelectorStyles {
    fn default() -> Self {
        Self {
            editor_padding: cuteui::ui_components::components::Coords::default(),
            height: None,
            width: None,
            dropdown_background: Arc::new(|_| pathfinder_color::ColorU::new(0, 0, 0, 0)),
            border_color: Arc::new(|_| pathfinder_color::ColorU::new(0, 0, 0, 0)),
            border_radius: 0.0,
        }
    }
}

impl std::fmt::Debug for WorkflowArgSelectorStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowArgSelectorStyles")
            .field("editor_padding", &self.editor_padding)
            .field("height", &self.height)
            .field("width", &self.width)
            .field("border_radius", &self.border_radius)
            .finish()
    }
}

/// Minimal stub for AdminEnablementSetting (cloud feature)
#[derive(Clone, Debug, PartialEq)]
pub enum AdminEnablementSetting {
    Enable,
    Disable,
    Enabled,
    Disabled,
    RespectUserSetting,
}

/// Minimal stub for UgcCollectionEnablementSetting (cloud feature)
#[derive(Clone, Debug, PartialEq)]
pub enum UgcCollectionEnablementSetting {
    Enable,
    Disable,
    Enabled,
    Disabled,
    RespectUserSetting,
}

/// Minimal stub for AiAutonomySettings (cloud feature)
/// Re-exported from crates/graphql/src/api/workspace.rs
#[derive(Clone, Debug, Default)]
pub struct AiAutonomySettings {
    pub apply_code_diffs_setting: Option<AiAutonomyValue>,
    pub read_files_setting: Option<AiAutonomyValue>,
    pub read_files_allowlist: Option<Vec<String>>,
    pub create_plans_setting: Option<AiAutonomyValue>,
    pub execute_commands_setting: Option<AiAutonomyValue>,
    pub execute_commands_allowlist: Option<Vec<String>>,
    pub execute_commands_denylist: Option<Vec<String>>,
    pub write_to_pty_setting: Option<WriteToPtyAutonomyValue>,
    pub computer_use_setting: Option<ComputerUseAutonomyValue>,
}

/// Minimal stub for AiAutonomyValue (cloud feature)
#[derive(Clone, Debug, PartialEq)]
pub enum AiAutonomyValue {
    AgentDecides,
    AlwaysAllow,
    AlwaysAsk,
    RespectUserSetting,
    Other(String),
}

/// Minimal stub for WriteToPtyAutonomyValue (cloud feature)
#[derive(Clone, Debug, PartialEq)]
pub enum WriteToPtyAutonomyValue {
    AlwaysAllow,
    AlwaysAsk,
    AskOnFirstWrite,
    RespectUserSetting,
    Other(String),
}

/// Minimal stub for ComputerUseAutonomyValue (cloud feature)
#[derive(Clone, Debug, PartialEq)]
pub enum ComputerUseAutonomyValue {
    Never,
    AlwaysAsk,
    AlwaysAllow,
    RespectUserSetting,
    Other(String),
}

// Local version: No workspace overrides, so all has_override methods return false
impl AiAutonomySettings {
    pub fn has_override_for_code_diffs(&self) -> bool {
        false
    }

    pub fn has_override_for_read_files(&self) -> bool {
        false
    }

    pub fn has_override_for_execute_commands(&self) -> bool {
        false
    }

    pub fn has_override_for_write_to_pty(&self) -> bool {
        false
    }

    pub fn has_override_for_execute_commands_allowlist(&self) -> bool {
        false
    }

    pub fn has_override_for_read_files_allowlist(&self) -> bool {
        false
    }

    pub fn has_override_for_computer_use(&self) -> bool {
        false
    }
}

/// Minimal stub for ContentEditability
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentEditability {
    Editable,
    RequiresLogin,
    ReadOnly,
}

impl ContentEditability {
    pub fn can_edit(&self) -> bool {
        matches!(self, ContentEditability::Editable)
    }
}

// ===== Workflow Argument Types =====

/// Minimal stub for WorkflowEnumData
#[derive(Clone, Debug, Default)]
pub struct WorkflowEnumData {
    pub name: String,
    pub values: Vec<String>,
    pub owner: String,
    pub new_data: Option<EnumData>,
}

/// Minimal stub for ArgumentTypeEditor trait
pub trait ArgumentTypeEditor {
    fn arg_type_editor(&self) -> &cuteui::ViewHandle<WorkflowArgSelector>;
}

// ===== Import/Export Modal Types =====

/// Minimal stub for ImportModal
#[derive(Clone, Debug)]
pub struct ImportModal {}

/// Implement Entity trait for ImportModal
impl cuteui::Entity for ImportModal {
    type Event = ImportModalEvent;
}

/// Implement View trait for ImportModal
impl View for ImportModal {
    fn ui_name() -> &'static str {
        "ImportModal"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        // Stub implementation - returns empty element
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for ImportModal
impl cuteui::TypedActionView for ImportModal {
    type Action = ();
}

impl ImportModal {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {}
    }

    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn focus_contents(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open_with_target(&mut self, _owner: Owner, _initial_folder_id: Option<SyncId>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

/// Minimal stub for ImportModalEvent
#[derive(Clone, Debug)]
pub enum ImportModalEvent {
    OpenTargetWithHashedId(String),
    Close,
    Cancel,
    Import,
}

/// Minimal stub for ExportManager
#[derive(Clone, Debug)]
pub struct ExportManager {}

/// Implement Entity trait for ExportManager
impl cuteui::Entity for ExportManager {
    type Event = ();
}

/// Implement SingletonEntity trait for ExportManager
impl SingletonEntity for ExportManager {}

impl ExportManager {
    pub fn handle(_ctx: &AppContext) -> ModelHandle<Self> {
        <Self as SingletonEntity>::handle(_ctx)
    }

    pub fn export(&mut self, _window_id: cuteui::WindowId, _exportable_objects: &Vec<CloudObjectTypeAndId>, _ctx: &mut cuteui::ModelContext<Self>) {
        // Stub implementation
    }
}

// ===== Workflow Related Types =====

/// Minimal stub for WorkflowArgSelectorEvent
#[derive(Clone, Debug)]
pub enum WorkflowArgSelectorEvent {
    NewEnum,
    LoadEnum(usize),
    Edited,
    Close,
    ToggleExpanded,
    InputTab,
    InputShiftTab,
}

/// Minimal stub for EnumCreationDialog
#[derive(Clone, Debug)]
pub struct EnumCreationDialog {}

/// Implement Entity trait for EnumCreationDialog
impl cuteui::Entity for EnumCreationDialog {
    type Event = EnumCreationDialogEvent;
}

/// Implement View trait for EnumCreationDialog
impl View for EnumCreationDialog {
    fn ui_name() -> &'static str {
        "EnumCreationDialog"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Empty::new())
    }
}

/// Implement TypedActionView trait for EnumCreationDialog
impl cuteui::TypedActionView for EnumCreationDialog {
    type Action = ();
}

impl EnumCreationDialog {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self {}
    }

    pub fn initialize(&mut self, _is_new: bool, _enum_data: Option<EnumData>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

/// Minimal stub for EnumCreationDialogEvent
#[derive(Clone, Debug)]
pub enum EnumCreationDialogEvent {
    Close,
    CreateEnum(EnumData),
    EditEnum(EnumData, bool),
}

/// Minimal stub for EnumData
#[derive(Clone, Debug, Default)]
pub struct EnumData {
    pub name: String,
    pub values: Vec<String>,
}

/// Minimal stub for GeneratedCommandMetadata
#[derive(Clone, Debug)]
pub struct GeneratedCommandMetadata {
    pub title: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<crate::cloud_stub_types::models::workflow::Argument>,
}

/// Minimal stub for GeneratedCommandMetadataError
#[derive(Clone, Debug)]
pub enum GeneratedCommandMetadataError {
    RateLimited,
    ParsingError,
    InvalidFormat,
    NetworkError,
}

impl GeneratedCommandMetadataError {
    pub fn user_facing_message(&self) -> String {
        match self {
            Self::RateLimited => "Rate limited, please try again later".to_string(),
            Self::ParsingError => "Error parsing AI response".to_string(),
            Self::InvalidFormat => "Invalid format in AI response".to_string(),
            Self::NetworkError => "Network error".to_string(),
        }
    }
}

/// Minimal stub for WorkflowModal
#[derive(Clone, Debug)]
pub struct WorkflowModal {}

/// Implement Entity trait for WorkflowModal
impl cuteui::Entity for WorkflowModal {
    type Event = WorkflowModalEvent;
}

/// Implement View trait for WorkflowModal
impl View for WorkflowModal {
    fn ui_name() -> &'static str {
        "WorkflowModal"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        // Stub implementation - returns empty element
        Box::new(cuteui::elements::Empty::new())
    }
}

impl WorkflowModal {
    pub fn new(_ai_client: std::sync::Arc<dyn crate::server::server_api::ai::AIClient>, _ctx: &mut ViewContext<Self>) -> Self {
        Self {}
    }

    pub fn is_open(&self) -> bool {
        false // Stub implementation - always returns false
    }

    pub fn open_with_new(&mut self, _owner: Owner, _initial_folder_id: Option<crate::server::ids::SyncId>, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn open_with_cloud_workflow(&mut self, _workflow_id: CloudObjectId, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

// ===== Notebook Event Types =====

/// Minimal stub for NotebookEvent
#[derive(Clone, Debug)]
pub enum NotebookEvent {
    Saved,
    CellExecuted(usize),
    CellAdded,
    CellRemoved(usize),
}

/// Minimal stub for LinkEvent
#[derive(Clone, Debug)]
pub enum LinkEvent {
    LinkAdded(String),
    LinkRemoved(String),
    LinkUpdated(String),
}

/// Minimal stub for FileNotebookEvent
#[derive(Clone, Debug)]
pub enum FileNotebookEvent {
    FileLinked(String),
    FileUnlinked(String),
    RunWorkflow {
        workflow: std::sync::Arc<crate::workflows::WorkflowType>,
        source: crate::workflows::WorkflowSource,
    },
    TitleUpdated,
    FileLoaded,
    #[cfg(feature = "local_fs")]
    OpenFileWithTarget {
        path: std::path::PathBuf,
        target: crate::util::openable_file_type::FileTarget,
        line_col: Option<cute_util::path::LineAndColumnArg>,
    },
    Pane(crate::pane_group::PaneEvent),
}

/// Minimal stub for NotebookSource
#[derive(Clone, Debug, Default)]
pub enum NotebookSource {
    Cloud(SyncId),
    Local(SyncId),
    Existing(SyncId),
    #[default]
    Unsaved,
    New {
        title: Option<String>,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
    },
}

impl NotebookSource {
    /// Create from SyncId
    pub fn from_id(id: SyncId) -> Self {
        NotebookSource::Existing(id)
    }
}

// Removed: From<&str> implementation doesn't work with SyncId

impl std::fmt::Display for NotebookSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotebookSource::Cloud(id) => write!(f, "Cloud({})", id),
            NotebookSource::Local(id) => write!(f, "Local({})", id),
            NotebookSource::Existing(id) => write!(f, "Existing({})", id),
            NotebookSource::Unsaved => write!(f, "Unsaved"),
            NotebookSource::New { title, .. } => write!(f, "New({:?})", title),
        }
    }
}

/// Minimal stub for SessionSource
#[derive(Clone, Debug)]
pub enum SessionSource {
    Active(cuteui::WindowId),
    Inactive,
}

/// Minimal stub for SessionSourceType
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SessionSourceType {
    #[default]
    User,
    AmbientAgent { task_id: Option<String> },
    CommandPalette { command_id: String },
    Workflow { workflow_id: String },
}

impl SessionSourceType {
    pub fn orchestrator_task_id(&self) -> Option<&str> {
        match self {
            SessionSourceType::AmbientAgent { task_id } => task_id.as_deref(),
            SessionSourceType::CommandPalette { .. } => None,
            SessionSourceType::Workflow { .. } => None,
        }
    }
}

// ===== Editor Related Types =====

/// Minimal stub for LineCol (line and column position)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub column: u32,
}

/// Minimal stub for MarkdownDisplayMode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkdownDisplayMode {
    #[default]
    Rendered,
    Source,
    Raw,
}

/// Minimal stub for RichTextEditorConfig
#[derive(Clone, Debug, Default)]
pub struct RichTextEditorConfig {
    pub enable_spell_check: bool,
    pub enable_auto_save: bool,
    pub gutter_width: Option<f32>,
    pub embedded_objects_enabled: bool,
    pub vertical_expansion_behavior: Option<cute_editor::render::element::VerticalExpansionBehavior>,
    pub max_width: Option<f32>,
    pub can_execute_shell_commands: bool,
    pub disable_block_insertion_menu: bool,
    pub disable_scrolling: bool,
}

// ===== Notebook Manager and Editor =====

impl NotebookLinks {
    pub fn new(_source: SessionSource, _ctx: &cuteui::AppContext) -> Self {
        Self::default()
    }
}

// ===== Notebook Event Handling =====

impl EditorViewEvent {
    #[allow(non_snake_case)]
    pub fn Edited() -> Self {
        Self::Edited
    }

    #[allow(non_snake_case)]
    pub fn Focused() -> Self {
        Self::Focused
    }

    #[allow(non_snake_case)]
    pub fn OpenFile(_path: String, _line: Option<usize>) -> Self {
        Self::ContentChanged
    }
}

// ===== CuteDrive Settings Event =====

/// Minimal stub for CuteDriveSettingsChangedEvent
#[derive(Clone, Debug)]
pub enum CuteDriveSettingsChangedEvent {
    EnableCuteDrive { enabled: bool },
    EnableWarpDrive { enabled: bool },
    DisableCuteDrive,
    SettingsUpdated,
}

// ===== Workspace Related Types =====

/// Minimal stub for WorkspaceUid
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct WorkspaceUid(pub String);

impl From<String> for WorkspaceUid {
    fn from(s: String) -> Self {
        WorkspaceUid(s)
    }
}

impl From<cute_server_client::ids::ServerId> for WorkspaceUid {
    fn from(id: cute_server_client::ids::ServerId) -> Self {
        WorkspaceUid::from(String::from(id))
    }
}

impl From<WorkspaceUid> for String {
    fn from(uid: WorkspaceUid) -> Self {
        uid.0
    }
}

impl std::fmt::Display for WorkspaceUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Minimal stub for WorkspaceMetadata
#[derive(Clone, Debug, Default)]
pub struct WorkspaceMetadata {
    pub uid: WorkspaceUid,
    pub name: String,
    pub teams: Vec<TeamMetadata>, // Changed from Vec<String> to Vec<TeamMetadata>
}

impl WorkspaceMetadata {
    pub fn new(uid: WorkspaceUid, name: String) -> Self {
        Self {
            uid,
            name,
            teams: Vec::new(),
        }
    }

    pub fn from_local_cache(_uid: WorkspaceUid, _name: String, _teams: Vec<TeamMetadata>) -> Self {
        Self {
            uid: _uid,
            name: _name,
            teams: _teams,
        }
    }

    pub fn are_overages_toggleable(&self) -> bool {
        false
    }

    pub fn are_overages_enabled(&self) -> bool {
        false
    }
}

/// Minimal stub for TeamMember
#[derive(Clone, Debug)]
pub struct TeamMember {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub role: MembershipRole,
}

/// Minimal stub for MembershipRole
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MembershipRole {
    Owner,
    Admin,
    Member,
    User,
}

/// Minimal stub for FolderId
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct FolderId(pub String);

impl FolderId {
    pub fn from_hash(hash: &str) -> Option<Self> {
        // Parse "Folder-{id}" format, similar to cute_server_client::ids::FolderId
        if let Some(id) = hash.strip_prefix("Folder-") {
            Some(Self(id.to_string()))
        } else {
            // If not in proper format, return None (stub behavior)
            None
        }
    }
}

impl From<FolderId> for crate::server::ids::ServerId {
    fn from(folder_id: FolderId) -> Self {
        crate::server::ids::ServerId::from_string_lossy(folder_id.0)
    }
}

impl From<FolderId> for crate::server::ids::SyncId {
    fn from(folder_id: FolderId) -> Self {
        crate::server::ids::SyncId::ServerId(folder_id.into())
    }
}

// ===== Cloud Object Metadata Types =====

// ObjectType and GenericStringObjectFormat are already imported from cute_server_client::cloud_object
// See line 185 for re-export: pub use cute_server_client::cloud_object::*;

/// Minimal stub for CloudObjectMetadata
#[derive(Clone, Debug, Default)]
pub struct CloudObjectMetadata {
    pub revision: Option<Revision>,
    pub metadata_last_updated_ts: Option<cute_graphql::scalars::time::ServerTimestamp>,
    pub current_editor_uid: Option<String>,
    pub pending_changes_statuses: CloudObjectStatuses,
    pub trashed_ts: Option<cute_graphql::scalars::time::ServerTimestamp>,
    pub folder_id: Option<crate::server::ids::SyncId>,
    pub is_welcome_object: bool,
    pub last_editor_uid: Option<String>,
    pub creator_uid: Option<String>,
    pub last_task_run_ts: Option<cute_graphql::scalars::time::ServerTimestamp>,
}

impl CloudObjectMetadata {
    pub fn has_pending_content_changes(&self) -> bool {
        !matches!(
            self.pending_changes_statuses.content_sync_status,
            CloudObjectSyncStatus::NoLocalChanges | CloudObjectSyncStatus::InConflict
        )
    }

    pub fn is_errored(&self) -> bool {
        matches!(
            self.pending_changes_statuses.content_sync_status,
            CloudObjectSyncStatus::Errored
        )
    }

    pub fn has_pending_online_only_change(&self) -> bool {
        false
    }

    pub fn has_online_only_content(&self) -> bool {
        false
    }
}

/// Minimal stub for CloudObjectStatuses
#[derive(Clone, Debug, Default)]
pub struct CloudObjectStatuses {
    pub content_sync_status: CloudObjectSyncStatus,
    pub has_pending_permissions_change: bool,
    pub has_pending_metadata_change: bool,
    pub pending_untrash: bool,
    pub pending_delete: bool,
}

/// Minimal stub for CloudObjectPermissions
#[derive(Clone, Debug, Default)]
pub struct CloudObjectPermissions {
    pub owner: Owner,
    pub permissions_last_updated_ts: Option<cute_graphql::scalars::time::ServerTimestamp>,
    pub anyone_with_link: Option<CloudLinkSharing>,
    pub guests: Vec<CloudObjectGuest>,
}

/// Minimal stub for CloudLinkSharing
#[derive(Clone, Debug, PartialEq, Default)]
pub struct CloudLinkSharing {
    pub access_level: SharingAccessLevel,
    pub source: Option<cute_server_client::cloud_object::ServerObjectContainer>,
}

/// Minimal stub for Subject (cloud object subject)
#[derive(Clone, Debug, PartialEq)]
pub enum Subject {
    User(UserKind),
    Team(cute_server_client::auth::TeamUid),
}

/// Minimal stub for UserKind
#[derive(Clone, Debug, PartialEq)]
pub enum UserKind {
    Account(cute_server_client::auth::UserUid),
    Agent(cute_server_client::auth::UserUid),
}

/// Minimal stub for CloudObjectGuest
#[derive(Clone, Debug, PartialEq)]
pub struct CloudObjectGuest {
    pub subject: Subject,
    pub access_level: SharingAccessLevel,
    pub source: Option<cute_server_client::cloud_object::ServerObjectContainer>,
}

impl Default for CloudObjectGuest {
    fn default() -> Self {
        Self {
            subject: Subject::User(UserKind::Account(cute_server_client::auth::UserUid::new("default"))),
            access_level: SharingAccessLevel::Viewer,
            source: None,
        }
    }
}

// Owner is already imported from cute_server_client::cloud_object
// See line 185 for re-export: pub use cute_server_client::cloud_object::*;

// NotebookId and GenericStringObjectId are already imported from models and model::generic_string_model
// See lines 182 and 191 for re-exports

/// Minimal stub for Revision (version tracking)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Revision(pub u64);

impl Revision {
    pub fn from_unix_timestamp_micros(timestamp: i64) -> Result<Self, &'static str> {
        Ok(Self(timestamp as u64))
    }

    pub fn unix_timestamp_micros(&self) -> i64 {
        self.0 as i64
    }

    pub fn utc(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp_micros(self.0 as i64)
            .unwrap_or_else(chrono::Utc::now)
    }
}

impl Default for Revision {
    fn default() -> Self {
        Self(0)
    }
}

impl From<Revision> for cute_graphql::scalars::time::ServerTimestamp {
    fn from(revision: Revision) -> Self {
        cute_graphql::scalars::time::ServerTimestamp::from_unix_timestamp_micros(revision.unix_timestamp_micros())
            .unwrap_or_else(|_| cute_graphql::scalars::time::ServerTimestamp::now())
    }
}

/// Minimal stub for CloudObjectSyncStatus
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CloudObjectSyncStatus {
    InFlight(NumInFlightRequests),
    Synced,
    Failed,
    Pending,
    NoLocalChanges,
    Errored,
    InConflict,
}

impl Default for CloudObjectSyncStatus {
    fn default() -> Self {
        CloudObjectSyncStatus::Synced
    }
}

/// Minimal stub for NumInFlightRequests
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumInFlightRequests(pub u32);

/// Minimal stub for ObjectAction (object change action)
#[derive(Clone, Debug)]
pub enum ObjectAction {
    Create,
    Update,
    Delete,
    Upsert,
    Move,
    Trash,
    Untrash,
    Share,
}

/// Minimal stub for UserProfileWithUID
#[derive(Clone, Debug)]
pub struct UserProfileWithUID {
    pub uid: String,
    pub name: String,
    pub email: String,
}

/// Minimal stub for WorkspacesMetadataWithPricing
#[derive(Clone, Debug)]
pub struct WorkspacesMetadataWithPricing {
    pub workspaces: Vec<WorkspaceMetadata>,
}

/// Minimal stub for AiOverages
#[derive(Clone, Debug, Default)]
pub struct AiOverages {
    pub enabled: bool,
}

// ===== BackingView Implementations =====

/// Implement Entity trait for FileNotebookView
impl cuteui::Entity for FileNotebookView {
    type Event = FileNotebookEvent;
}

/// Implement BackingView trait for NotebookView
impl BackingView for NotebookView {
    type PaneHeaderOverflowMenuAction = ();
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(PaneEvent::Close);
    }

    fn focus_contents(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple("Notebook")
    }

    fn set_focus_handle(&mut self, _focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

/// Implement BackingView trait for FileNotebookView
impl BackingView for FileNotebookView {
    type PaneHeaderOverflowMenuAction = ();
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(FileNotebookEvent::Pane(PaneEvent::Close));
    }

    fn focus_contents(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple("File Notebook")
    }

    fn set_focus_handle(&mut self, _focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }
}

/// Implement View trait for NotebookView
impl View for NotebookView {
    fn ui_name() -> &'static str {
        "NotebookView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Flex::column())
    }
}

/// Implement View trait for FileNotebookView
impl View for FileNotebookView {
    fn ui_name() -> &'static str {
        "FileNotebookView"
    }

    fn render(&self, _ctx: &AppContext) -> Box<dyn cuteui::Element> {
        Box::new(cuteui::elements::Flex::column())
    }
}

// ===== Cloud Object Core Types (moved from cloud_object/mod.rs) =====

pub trait CloudObject: Debug {
    fn model_type_name(&self) -> &'static str;

    fn uid(&self) -> ObjectUid;

    fn sync_id(&self) -> crate::server::ids::SyncId;

    fn hashed_sqlite_id(&self) -> HashedSqliteId;

    fn metadata(&self) -> &CloudObjectMetadata;

    fn metadata_mut(&mut self) -> &mut CloudObjectMetadata;

    fn permissions(&self) -> &CloudObjectPermissions;

    fn permissions_mut(&mut self) -> &mut CloudObjectPermissions;

    fn object_type(&self) -> ObjectType;

    fn cloud_object_type_and_id(&self) -> CloudObjectTypeAndId;

    fn set_server_id(&mut self, _server_id: ServerId) {}

    fn can_move_to_space(&self, _space: Space, _app: &AppContext) -> bool {
        true
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        false
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        true
    }

    fn upsert_event(&self) -> ModelEvent;

    fn display_name(&self) -> String;

    fn versions(&self, _app: &AppContext) -> Option<cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput> {
        None
    }

    fn renders_in_warp_drive(&self) -> bool;

    fn should_show_activity_toasts(&self) -> bool {
        true
    }

    fn to_warp_drive_item(&self, appearance: &Appearance) -> Option<Box<dyn CuteDriveItem>>;

    fn object_link(&self) -> Option<String> {
        None
    }

    fn space(&self, app: &AppContext) -> Space {
        // COMMENTED: UserWorkspaces disabled in local version - 注释掉云端工作空间/团队功能 - 本地版本不支持
        Space::Personal // UserWorkspaces::as_ref(app).owner_to_space(self.permissions().owner, app)
    }

    fn can_leave(&self, _app: &AppContext) -> bool {
        false
    }

    fn containing_object_name(&self, app: &AppContext) -> String {
        self.containing_objects_path(app)
            .into_iter()
            .next_back()
            .expect("Object should have at least one ancestor")
            .name
    }

    fn containing_objects_path(&self, app: &AppContext) -> Vec<ContainingObject> {
        let space = self.space(app);

        match self.metadata().folder_id {
            Some(folder_id) => {
                let cloud_model = CloudModel::as_ref(app);
                if let Some(folder) = cloud_model.get_folder_by_uid(&folder_id.uid()) {
                    let mut path = vec![];
                    let ancestors = folder.containing_objects_path(app);
                    path.extend(ancestors);
                    path.push(folder.into());
                    path
                } else {
                    vec![space.into_containing_object(app)]
                }
            }
            None => vec![space.into_containing_object(app)],
        }
    }

    fn breadcrumbs(&self, app: &AppContext) -> String {
        self.containing_objects_path(app)
            .into_iter()
            .map(|object| object.name)
            .collect::<Vec<String>>()
            .join(" / ")
    }

    fn is_in_space(&self, space: Space, app: &AppContext) -> bool {
        self.space(app) == space
    }

    fn is_welcome_object(&self) -> bool {
        self.metadata().is_welcome_object
    }

    fn location(&self, cloud_model: &CloudModel, app: &AppContext) -> CloudObjectLocation {
        if let Some(folder_id) = self.metadata().folder_id {
            if cloud_model.get_folder(&folder_id).is_some() {
                return CloudObjectLocation::Folder(folder_id);
            }
        }

        CloudObjectLocation::Space(self.space(app))
    }

    fn is_trashed(&self, cloud_model: &CloudModel) -> bool {
        self.is_trashed_internal(cloud_model, &mut HashSet::new())
    }

    fn is_trashed_internal(
        &self,
        cloud_model: &CloudModel,
        ancestors: &mut HashSet<String>,
    ) -> bool {
        if self.metadata().trashed_ts.is_some() {
            return true;
        }

        match self.metadata().folder_id.map(|parent_id| parent_id.uid()) {
            Some(hashed_parent_id) => {
                if ancestors.contains(&hashed_parent_id) {
                    return true;
                }
                ancestors.insert(hashed_parent_id.clone());

                match cloud_model.get_by_uid(&hashed_parent_id) {
                    Some(parent) => parent.is_trashed_internal(cloud_model, ancestors),
                    None => false,
                }
            }
            None => false,
        }
    }

    fn has_conflicting_changes(&self) -> bool {
        false
    }

    fn conflicting_object_revision(&self) -> Option<Revision> {
        None
    }

    fn clear_conflict_status(&mut self) {}

    fn replace_object_with_conflict(&mut self) {}

    fn increment_in_flight_request_count(&mut self) {}

    fn decrement_in_flight_request_count(&mut self, _status_if_no_reqs: CloudObjectSyncStatus) -> bool {
        true
    }

    fn set_pending_content_changes_status(&mut self, _pending_content_changes_status: CloudObjectSyncStatus) {}

    fn can_export(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_model_type<K, M>(cloud_object: &dyn CloudObject) -> Option<&GenericCloudObject<K, M>>
    where
        Self: Sized,
        K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        cloud_object
            .as_any()
            .downcast_ref::<GenericCloudObject<K, M>>()
    }

    fn as_model_type_mut<K, M>(
        cloud_object: &mut dyn CloudObject,
    ) -> Option<&mut GenericCloudObject<K, M>>
    where
        Self: Sized,
        K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
        M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
    {
        cloud_object
            .as_any_mut()
            .downcast_mut::<GenericCloudObject<K, M>>()
    }

    fn clone_box(&self) -> Box<dyn CloudObject>;

    fn create_object_queue_item(
        &self,
        _entrypoint: CloudObjectEventEntrypoint,
        _initiated_by: crate::server::sync_queue::InitiatedBy,
    ) -> Option<crate::server::sync_queue::QueueItem> {
        None
    }

    fn update_object_queue_item(&self, _revision: Option<Revision>) -> crate::server::sync_queue::QueueItem {
        panic!("update_object_queue_item: cloud sync has been removed")
    }
}

#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
pub trait CloudModelType: Debug + Clone + Send + Sync {
    type CloudObjectType: CloudObject + 'static;
    type IdType: HashableId + ToServerId + Debug + Into<String> + Clone + 'static;

    fn model_type_name(&self) -> &'static str;

    fn cloud_object_type_and_id(&self, id: crate::server::ids::SyncId) -> CloudObjectTypeAndId;

    fn object_type(&self) -> ObjectType;

    fn renders_in_warp_drive(&self) -> bool;

    fn should_show_activity_toasts(&self) -> bool {
        true
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        true
    }

    fn to_warp_drive_item(
        &self,
        id: crate::server::ids::SyncId,
        appearance: &Appearance,
        object: &Self::CloudObjectType,
    ) -> Option<Box<dyn CuteDriveItem>>;

    fn display_name(&self) -> String;

    fn set_display_name(&mut self, _name: &str) {}

    fn upsert_event(params: CloudObjectUpsertParams<Self>) -> ModelEvent
    where
        Self: Sized;

    fn bulk_upsert_event(objects: Vec<CloudObjectUpsertParams<Self>>) -> ModelEvent
    where
        Self: Sized;

    async fn send_create_request(
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _request: CreateObjectRequest,
    ) -> anyhow::Result<CreateCloudObjectResult> {
        Err(anyhow::anyhow!("cloud sync has been removed"))
    }

    async fn send_update_request(
        &self,
        _object_client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _server_id: ServerId,
        _revision: Option<Revision>,
    ) -> anyhow::Result<UpdateCloudObjectResult<GenericServerObject<Self::IdType, Self>>> {
        Err(anyhow::anyhow!("cloud sync has been removed"))
    }

    fn can_move_to_space(&self, _current_space: Space, _new_space: Space) -> bool {
        true
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        false
    }

    fn supports_linking(&self) -> bool {
        false
    }

    fn should_update_after_server_conflict(&self) -> bool {
        false
    }

    fn can_export(&self) -> bool {
        false
    }
}

pub trait CloudObjectLookup: Sized + Clone {
    fn get_all(app: &AppContext) -> Vec<Self>;

    fn get_by_id<'a>(sync_id: &'a crate::server::ids::SyncId, app: &'a AppContext) -> Option<&'a Self>;
}

impl<K, M> CloudObjectLookup for GenericCloudObject<K, M>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn get_all(app: &AppContext) -> Vec<Self> {
        CloudModel::as_ref(app)
            .get_all_objects_of_type::<K, M>()
            .cloned()
            .collect()
    }

    fn get_by_id<'a>(sync_id: &'a crate::server::ids::SyncId, app: &'a AppContext) -> Option<&'a Self> {
        CloudModel::as_ref(app).get_object_of_type::<K, M>(sync_id)
    }
}

pub trait CloudObjectUuid {
    fn uuid(&self) -> uuid::Uuid;
}

pub trait CloudObjectUuidLookup: Sized {
    fn get_by_uuid<'a>(uuid: &'a uuid::Uuid, app: &'a AppContext) -> Option<&'a Self>;
}

impl<T, S> CloudObjectUuidLookup
    for GenericCloudObject<GenericStringObjectId, GenericStringModel<T, S>>
where
    T: StringModel<
            CloudObjectType = GenericCloudObject<GenericStringObjectId, GenericStringModel<T, S>>,
        > + CloudObjectUuid,
    S: Serializer<T>,
{
    fn get_by_uuid<'a>(uuid: &'a uuid::Uuid, app: &'a AppContext) -> Option<&'a Self> {
        CloudModel::as_ref(app)
            .get_all_objects_of_type::<GenericStringObjectId, GenericStringModel<T, S>>()
            .find(|object| object.model().string_model.uuid() == *uuid)
    }
}

lazy_static! {
    static ref SPACE_DETECT_RE: Regex = Regex::new(r"\s+").expect("Expect regex to be valid");
    static ref SAFE_URL_CHAR_RE: Regex =
        Regex::new(r"[^a-zA-Z0-9\s-]").expect("Expect regex to be valid");
}

impl<K, M> CloudObject for GenericCloudObject<K, M>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn model_type_name(&self) -> &'static str {
        self.model().model_type_name()
    }

    fn uid(&self) -> ObjectUid {
        self.id.uid()
    }

    fn hashed_sqlite_id(&self) -> HashedSqliteId {
        self.id.sqlite_uid_hash(self.object_type().into())
    }

    fn sync_id(&self) -> crate::server::ids::SyncId {
        self.id
    }

    fn should_show_activity_toasts(&self) -> bool {
        self.model().should_show_activity_toasts()
    }

    fn warn_if_unsaved_at_quit(&self) -> bool {
        self.model().warn_if_unsaved_at_quit()
    }

    fn metadata(&self) -> &CloudObjectMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut CloudObjectMetadata {
        &mut self.metadata
    }

    fn permissions(&self) -> &CloudObjectPermissions {
        &self.permissions
    }

    fn permissions_mut(&mut self) -> &mut CloudObjectPermissions {
        &mut self.permissions
    }

    fn object_type(&self) -> ObjectType {
        self.model().object_type()
    }

    fn cloud_object_type_and_id(&self) -> CloudObjectTypeAndId {
        self.model().cloud_object_type_and_id(self.id)
    }

    fn should_clear_on_unique_key_conflict(&self) -> bool {
        self.model().should_clear_on_unique_key_conflict()
    }

    fn can_move_to_space(&self, space: Space, app: &AppContext) -> bool {
        self.model().can_move_to_space(self.space(app), space)
    }

    fn has_conflicting_changes(&self) -> bool {
        self.conflict_status.has_conflicts()
    }

    fn conflicting_object_revision(&self) -> Option<Revision> {
        match &self.conflict_status {
            ConflictStatus::ConflictingChanges { object } => Some(object.metadata.revision.clone()),
            ConflictStatus::NoConflicts => None,
        }
    }

    fn clear_conflict_status(&mut self) {
        self.conflict_status = ConflictStatus::NoConflicts;
    }

    fn replace_object_with_conflict(&mut self) {
        let mut new_conflict = ConflictStatus::NoConflicts;
        std::mem::swap(&mut new_conflict, &mut self.conflict_status);

        self.set_pending_content_changes_status(CloudObjectSyncStatus::NoLocalChanges);

        if let ConflictStatus::ConflictingChanges { object } = new_conflict {
            if self.model().should_update_after_server_conflict() {
                self.metadata.update_revision_from_server(&object.metadata);
                self.set_model(object.model.clone());
                if self.metadata.has_pending_content_changes() {
                    self.conflict_status = ConflictStatus::ConflictingChanges { object };
                } else {
                    self.conflict_status = ConflictStatus::NoConflicts;
                }
            }
        }
    }

    fn set_server_id(&mut self, server_id: ServerId) {
        self.id = crate::server::ids::SyncId::ServerId(server_id);
    }

    fn object_link(&self) -> Option<String> {
        if !self.model().supports_linking() {
            return None;
        }

        let display_name = self.model().display_name();
        let name_without_unsafe_chars = SAFE_URL_CHAR_RE.replace_all(display_name.trim(), "");
        let link_safe_name = SPACE_DETECT_RE.replace_all(&name_without_unsafe_chars, "-");
        match &self.id {
            crate::server::ids::SyncId::ClientId(_) => None,
            crate::server::ids::SyncId::ServerId(id) => {
                let object_type = self.object_type();
                let object_type_for_link = if self
                    .as_any()
                    .downcast_ref::<CloudWorkflow>()
                    .is_some_and(|w| w.model().data.is_agent_mode_workflow())
                {
                    "prompt".to_string()
                } else {
                    object_type.to_string()
                };

                let mut link = format!(
                    "{}/drive/{}/{}-{}",
                    ChannelState::server_root_url(),
                    object_type_for_link,
                    link_safe_name,
                    id.uid()
                );

                if matches!(ChannelState::channel(), Channel::Preview) {
                    link.push_str("?preview=true");
                }

                Some(link)
            }
        }
    }

    fn upsert_event(&self) -> ModelEvent {
        M::upsert_event(self.upsert_params(self.object_type()))
    }

    fn display_name(&self) -> String {
        self.model().display_name()
    }

    fn versions(&self, app: &AppContext) -> Option<cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput> {
        match (self.id, self.metadata.revision.as_ref()) {
            (crate::server::ids::SyncId::ServerId(id), Some(revision)) => {
                let actions_ts = ObjectActions::as_ref(app)
                    .get_latest_processed_at_ts(&self.id.uid())
                    .map(|t| t.into());
                Some(cute_graphql::queries::get_updated_cloud_objects::UpdatedObjectInput {
                    uid: id.into(),
                    revision_ts: revision.timestamp(),
                    metadata_ts: self.metadata.metadata_last_updated_ts,
                    permissions_ts: self.permissions.permissions_last_updated_ts,
                    actions_ts,
                })
            }
            _ => None,
        }
    }

    fn renders_in_warp_drive(&self) -> bool {
        self.model().renders_in_warp_drive()
    }

    fn to_warp_drive_item(&self, appearance: &Appearance) -> Option<Box<dyn CuteDriveItem>> {
        self.model().to_warp_drive_item(self.id, appearance, self)
    }

    fn can_export(&self) -> bool {
        self.model().can_export()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn CloudObject> {
        Box::new(self.clone())
    }
}

pub fn extract_server_id_and_object_type_from_warp_drive_link(
    _url: &Url,
) -> Option<OpenCuteDriveObjectArgs> {
    None
}

impl<'a, K, M> From<&'a dyn CloudObject> for Option<&'a GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a dyn CloudObject) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type(value)
    }
}

impl<'a, K, M> From<&'a Box<dyn CloudObject>> for Option<&'a GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a Box<dyn CloudObject>) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type(value.as_ref())
    }
}

impl<'a, K, M> From<&'a mut Box<dyn CloudObject>> for Option<&'a mut GenericCloudObject<K, M>>
where
    K: HashableId + ToServerId + Debug + Into<String> + Clone + 'static,
    M: CloudModelType<IdType = K, CloudObjectType = GenericCloudObject<K, M>> + 'static,
{
    fn from(value: &'a mut Box<dyn CloudObject>) -> Self {
        <GenericCloudObject<K, M> as CloudObject>::as_model_type_mut(value.as_mut())
    }
}

impl Clone for Box<dyn CloudObject> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl From<&dyn CloudObject> for ObjectType {
    fn from(value: &dyn CloudObject) -> Self {
        value.object_type()
    }
}

impl From<&Box<dyn CloudObject>> for ObjectType {
    fn from(value: &Box<dyn CloudObject>) -> Self {
        <ObjectType as From<&dyn CloudObject>>::from(value.as_ref())
    }
}

pub trait CloudObjectMetadataExt {
    fn semantic_editing_history(&self, app: &AppContext) -> Option<String>;

    #[cfg_attr(target_family = "wasm", expect(dead_code))]
    fn semantic_creator(&self, app: &AppContext) -> Option<String>;

    fn semantic_permadeletion_countdown(&self, app: &AppContext) -> Option<String>;
}

impl CloudObjectMetadataExt for CloudObjectMetadata {
    fn semantic_editing_history(&self, app: &AppContext) -> Option<String> {
        let user_profiles = UserProfiles::as_ref(app);

        let editor_string = self
            .last_editor_uid
            .as_ref()
            .and_then(|uid| user_profiles.displayable_identifier_for_uid(crate::auth::UserUid::new(uid)));

        let time_ago_string = self
            .revision
            .clone()
            .map(|r| format_approx_duration_from_now_utc(r.utc()));

        let full_string = match (editor_string, time_ago_string) {
            (Some(name), Some(time_ago)) if name.is_empty() => format!("Edited {time_ago}"),
            (Some(name), Some(time_ago)) => format!("{name} edited {time_ago}"),
            (None, Some(time_ago)) => format!("Edited {time_ago}"),
            (Some(name), None) => format!("Last edited by {name}"),
            _ => return None,
        };

        Some(full_string)
    }

    fn semantic_creator(&self, app: &AppContext) -> Option<String> {
        let user_profiles = UserProfiles::as_ref(app);
        self.creator_uid
            .as_ref()
            .and_then(|uid| user_profiles.displayable_identifier_for_uid(crate::auth::UserUid::new(uid)))
    }

    fn semantic_permadeletion_countdown(&self, app: &AppContext) -> Option<String> {
        if let Some(trashed_ts) = self
            .trashed_ts
            .or_else(|| get_top_folder_trashed_ts(self.folder_id, app))
        {
            let deletion_time = trashed_ts.utc() + Duration::days(31);
            let current_time = Utc::now();
            let days_left = deletion_time.signed_duration_since(current_time).num_days();

            let full_string = match days_left {
                0 | 1 => "1 day until permanent deletion".to_string(),
                _ => format!("{days_left} days until permanent deletion"),
            };
            Some(full_string)
        } else {
            None
        }
    }
}

fn get_top_folder_trashed_ts(
    folder_id: Option<crate::server::ids::SyncId>,
    app: &AppContext,
) -> Option<cute_graphql::scalars::time::ServerTimestamp> {
    let mut folder_id = folder_id;
    let cloud_model = CloudModel::as_ref(app);
    while let Some(current_folder_id) = folder_id {
        let folder = cloud_model.get_folder_by_uid(&current_folder_id.uid())?;

        if let Some(_parent_folder_id) = folder.metadata.folder_id {
            folder_id = folder.metadata.folder_id
        } else {
            return folder.metadata.trashed_ts;
        }
    }
    None
}

#[derive(Default, Clone, Copy, Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum Space {
    #[default]
    Personal,
    Team { team_uid: ServerId },
    Shared,
}

impl Space {
    pub fn name(&self, app: &AppContext) -> String {
        match self {
            Space::Personal => "Personal".to_string(),
            Space::Team { team_uid, .. } => {
                let user_workspaces = UserWorkspaces::as_ref(app);
                if user_workspaces.team_from_uid(*team_uid).is_some() {
                    "Team".to_string()
                } else {
                    "Team".to_string()
                }
            }
            Space::Shared => "Shared with me".to_string(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum CloudObjectLocation {
    Space(Space),
    Folder(crate::server::ids::SyncId),
    Trash,
}

impl From<Space> for WorkflowSource {
    fn from(space: Space) -> Self {
        match space {
            Space::Personal => WorkflowSource::PersonalCloud,
            Space::Team { team_uid } => WorkflowSource::Team { team_uid },
            Space::Shared => WorkflowSource::PersonalCloud,
        }
    }
}

impl From<Owner> for Space {
    fn from(owner: Owner) -> Self {
        match owner {
            Owner::User { .. } => Space::Personal,
            Owner::Team { team_uid } => Space::Team { team_uid },
        }
    }
}

impl From<Owner> for WorkflowSource {
    fn from(owner: Owner) -> Self {
        match owner {
            Owner::User { .. } => Self::PersonalCloud,
            Owner::Team { team_uid } => Self::Team { team_uid },
        }
    }
}

// ===== Compatibility Type Aliases (from removed drive module) =====

pub type WarpDriveItemId = CuteDriveItemId;
// Note: WarpDriveItem cannot be a type alias to a trait (CuteDriveItem).
// Use CuteDriveItem directly wherever a trait object is needed (e.g., dyn CuteDriveItem).
// WarpDriveItem is still available as a re-export in the items submodule for non-trait-bound usage.
pub type WarpDriveWorkflow = CuteDriveWorkflow;
pub type WarpDriveEnvVarCollection = CuteDriveEnvVarCollection;
pub type WarpDriveAIFact = CuteDriveAIFact;
pub type WarpDriveMCPServer = CuteDriveMCPServer;
pub type OpenWarpDriveObjectArgs = OpenCuteDriveObjectArgs;
pub type WarpDriveSettings = CuteDriveSettings;
pub type WarpDriveSettingsChangedEvent = CuteDriveSettingsChangedEvent;

// ===== SharedSessionSource Stub (Session Sharing) =====

/// SharedSessionSource is defined earlier in this file

/// Minimal stub for IsSharedSessionCreator - shared session creator status
/// This is a cloud feature that's disabled in the local version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IsSharedSessionCreator {
    Yes { source: SharedSessionSource },
    No,
}

impl Default for IsSharedSessionCreator {
    fn default() -> Self {
        IsSharedSessionCreator::No
    }
}

// ===== SharedSessionStatus Stub (Session Sharing) =====

/// Minimal stub for SharedSessionStatus - session sharing status
/// This is a cloud feature that's disabled in the local version.
/// All variants return "NotShared" for keymap context.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SharedSessionStatus {
    #[default]
    NotShared,
    SharePending,
    SharePendingPreBootstrap {
        source: SharedSessionSource,
    },
}

impl SharedSessionStatus {
    /// Returns the keymap context string for this status.
    /// Always returns "NotShared" in the stub implementation.
    pub fn as_keymap_context(&self) -> &'static str {
        "NotShared"
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_viewer(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_executor(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_finished_viewer(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_sharer(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_sharer_or_viewer(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_reader(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_active_viewer(&self) -> bool {
        false
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_view_pending(&self) -> bool {
        matches!(self, SharedSessionStatus::SharePending | SharedSessionStatus::SharePendingPreBootstrap)
    }

    /// Returns false in stub - cloud sharing is disabled
    pub fn is_active_sharer(&self) -> bool {
        false
    }
}

/// Minimal stub for CloudAgentSettings
#[derive(Clone, Debug, Default)]
pub struct CloudAgentSettings {
    pub enabled: bool,
    pub model: String,
}

impl cuteui::Entity for CloudAgentSettings {
    type Event = ();
}

impl cuteui::SingletonEntity for CloudAgentSettings {}

impl CloudAgentSettings {
    pub fn as_ref(_ctx: &AppContext) -> &Self {
        static INSTANCE: CloudAgentSettings = CloudAgentSettings::default();
        &INSTANCE
    }

    pub fn handle(_ctx: &AppContext) -> cuteui::ModelHandle<Self> {
        <Self as cuteui::SingletonEntity>::handle(_ctx)
    }
}

/// Minimal stub for UpdateManager
#[derive(Clone, Debug)]
pub struct UpdateManager {
    pub initial_load_complete: bool,
}

impl UpdateManager {
    pub fn new() -> Self {
        Self { initial_load_complete: false }
    }

    pub fn initial_load_complete(&self) -> bool {
        self.initial_load_complete
    }

    pub fn create_ai_fact(&mut self, _ai_fact: crate::cloud_stub_types::models::ai_fact::AIFact, _client_id: crate::server::ids::ClientId, _owner: crate::cloud_stub_types::Owner, _ctx: &mut cuteui::ModelContext<Self>) {
        // Stub - no-op
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl cuteui::Entity for UpdateManager {
    type Event = ();
}

impl cuteui::SingletonEntity for UpdateManager {}

impl UpdateManager {
    pub fn as_ref(_ctx: &AppContext) -> &Self {
        static INSTANCE: UpdateManager = UpdateManager::default();
        &INSTANCE
    }

    pub fn handle(_ctx: &AppContext) -> cuteui::ModelHandle<Self> {
        <Self as cuteui::SingletonEntity>::handle(_ctx)
    }
}

// ===== IapManager Stub (IAP Authentication) =====

/// Minimal stub for IapManager - IAP authentication manager
/// This is a cloud feature that's disabled in the local version.
#[derive(Clone, Debug, Default)]
pub struct IapManager {}

impl cuteui::Entity for IapManager {
    type Event = ();
}

impl cuteui::SingletonEntity for IapManager {}

impl IapManager {
    pub fn new(_ctx: &mut cuteui::ModelContext<Self>) -> Self {
        Self::default()
    }

    pub fn handle(_ctx: &AppContext) -> cuteui::ModelHandle<Self> {
        <Self as cuteui::SingletonEntity>::handle(_ctx)
    }

    pub fn as_ref(_ctx: &AppContext) -> &Self {
        static INSTANCE: IapManager = IapManager::default();
        &INSTANCE
    }

    /// Stub: handle IAP challenge (no-op)
    pub fn handle_challenge(&mut self, _ctx: &mut cuteui::ModelContext<Self>) {
        // No-op in local version
    }
}

// ===== IapState Stub (IAP State) =====

/// Minimal stub for IapState - IAP authentication state
/// This is a cloud feature that's disabled in the local version.
#[derive(Clone, Debug, Default)]
pub struct IapState {}

// ===== OAuth2Client Stub =====

/// Minimal stub for OAuth2Client - OAuth2 client
/// This is a cloud feature that's disabled in the local version.
#[derive(Clone, Debug)]
pub struct OAuth2Client {}

impl OAuth2Client {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OAuth2Client {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Dropdown Stubs (UI Component) =====

/// Minimal stub for Dropdown - dropdown UI component
#[derive(Clone, Debug)]
pub struct Dropdown<T> {
    pub items: Vec<DropdownItem<T>>,
    pub selected_index: Option<usize>,
}

/// Minimal stub for DropdownItem - dropdown item
#[derive(Clone, Debug)]
pub struct DropdownItem<T> {
    pub label: String,
    pub value: T,
}

impl<T: Clone + std::fmt::Debug> Dropdown<T> {
    pub fn new(items: Vec<DropdownItem<T>>) -> Self {
        Self {
            items,
            selected_index: None,
        }
    }

    pub fn selected(&self) -> Option<&T> {
        self.selected_index.and_then(|idx| self.items.get(idx).map(|item| &item.value))
    }
}

impl<T: Clone + std::fmt::Debug> DropdownItem<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

// ===== UserProfiles Stub (User Profile Management) =====

/// Minimal stub for UserProfile - individual user profile
/// This is a cloud feature that's disabled in the local version.
#[derive(Clone, Debug, Default)]
pub struct UserProfile {
    pub email: String,
    pub photo_url: String,
}

impl UserProfile {
    /// Returns a displayable identifier for this user.
    /// Always returns empty string in the stub implementation.
    pub fn displayable_identifier(&self) -> String {
        String::new()
    }
}

/// Minimal stub for UserProfiles - user profile management
/// This is a cloud feature that's disabled in the local version.
/// All methods return default values or empty strings.
#[derive(Clone, Debug, Default)]
pub struct UserProfiles {
    // No internal state needed for stub
}

impl cuteui::Entity for UserProfiles {
    type Event = ();
}

impl cuteui::SingletonEntity for UserProfiles {}

impl UserProfiles {
    /// Returns a displayable identifier for the given user UID.
    /// Always returns None in the stub implementation.
    pub fn displayable_identifier_for_uid(&self, _uid: crate::auth::UserUid) -> Option<String> {
        None
    }

    /// Returns a reference to the UserProfiles singleton.
    pub fn as_ref(_ctx: &AppContext) -> &Self {
        static INSTANCE: UserProfiles = UserProfiles {};
        &INSTANCE
    }

    /// Returns a handle to the UserProfiles singleton.
    pub fn handle(_ctx: &AppContext) -> cuteui::ModelHandle<Self> {
        <Self as cuteui::SingletonEntity>::handle(_ctx)
    }

    /// Creates a new UserProfiles instance.
    pub fn new(_profiles: Vec<()>) -> Self {
        Self::default()
    }

    /// Stub method for getting user profile by UID.
    /// Always returns None in the stub implementation.
    pub fn profile_for_uid(&self, _uid: crate::auth::UserUid) -> Option<UserProfile> {
        None
    }

    /// Stub method for getting user profile by UID (reference version).
    pub fn get(&self, _uid: &crate::auth::UserUid) -> Option<&UserProfile> {
        None
    }
}

// ===== UserWorkspaces Stub (Cloud Workspace Management) =====

/// Minimal stub for UserWorkspaces - cloud workspace management
/// This is a cloud feature that's disabled in the local version.
/// All methods return appropriate default values (false/None/empty collections).
#[derive(Clone, Debug, Default)]
pub struct UserWorkspaces {
    // No internal state needed for stub
}

/// Event type for UserWorkspaces (stub)
#[derive(Clone, Debug)]
pub enum UserWorkspacesEvent {
    TeamsChanged,
    CodebaseContextEnablementChanged,
    // Add other events as needed for compatibility
}

impl cuteui::Entity for UserWorkspaces {
    type Event = UserWorkspacesEvent;
}

impl cuteui::SingletonEntity for UserWorkspaces {}

impl UserWorkspaces {
    pub fn new(
        _team_client: std::sync::Arc<dyn crate::server::server_api::team::TeamClient>,
        _workspace_client: std::sync::Arc<dyn crate::server::server_api::workspace::WorkspaceClient>,
        _cached_workspaces: Vec<WorkspaceMetadata>,
        _ctx: &mut cuteui::ModelContext<Self>,
    ) -> Self {
        Self::default()
    }

    pub fn handle(_ctx: &AppContext) -> cuteui::ModelHandle<Self> {
        <Self as cuteui::SingletonEntity>::handle(_ctx)
    }

    pub fn as_ref(_ctx: &AppContext) -> &Self {
        // Stub - returns a static instance
        static INSTANCE: UserWorkspaces = UserWorkspaces {};
        &INSTANCE
    }

    /// Returns whether enterprise secret redaction is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_enterprise_secret_redaction_enabled(&self) -> bool {
        // Cloud feature disabled in local version
        false
    }

    /// Returns whether AI is allowed for the current team.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn ai_allowed_for_current_team(&self) -> bool {
        // Cloud feature disabled in local version
        false
    }

    /// Returns whether codebase context is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_codebase_context_enabled(&self, _ctx: &AppContext) -> bool {
        // Cloud feature disabled in local version
        false
    }

    /// Returns whether team allows codebase context.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn team_allows_codebase_context(&self) -> bool {
        // Cloud feature disabled in local version
        false
    }

    /// Returns the default host slug for orchestration.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn default_host_slug(&self) -> Option<&str> {
        // Cloud feature disabled in local version
        None
    }

    /// Returns the team metadata for a given team UID.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn team_from_uid(&self, _team_uid: ServerId) -> Option<&TeamMetadata> {
        // Cloud feature disabled in local version
        None
    }

    /// Returns all user spaces (personal + teams).
    /// COMMENTED: Cloud feature disabled in local version
    pub fn all_user_spaces(&self, _ctx: &AppContext) -> Vec<Space> {
        // Cloud feature disabled in local version - return empty vector
        Vec::new()
    }

    /// Returns the current team UID if on a team.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn current_team_uid(&self) -> Option<ServerId> {
        // Cloud feature disabled in local version
        None
    }

    /// Returns the current team metadata if on a team.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn current_team(&self) -> Option<&TeamMetadata> {
        // Cloud feature disabled in local version
        None
    }

    /// Returns the personal drive space.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn personal_drive(&self, _ctx: &AppContext) -> Option<Owner> {
        // Cloud feature disabled in local version
        None
    }

    /// Returns cloud conversation storage enablement setting.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn get_cloud_conversation_storage_enablement_setting(&self) -> crate::settings::AdminEnablementSetting {
        // Cloud feature disabled in local version
        crate::settings::AdminEnablementSetting::Disabled
    }

    /// Converts owner to space.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn owner_to_space(&self, owner: Owner, _ctx: &AppContext) -> Space {
        // Simplified: convert owner to space without cloud logic
        match owner {
            Owner::User { .. } => Space::Personal,
            Owner::Team { team_uid } => Space::Team { team_uid },
        }
    }

    /// Returns whether voice is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_voice_enabled(&self) -> bool {
        false
    }

    /// Returns whether prompt suggestions are toggleable.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_prompt_suggestions_toggleable(&self) -> bool {
        false
    }

    /// Returns whether code suggestions are toggleable.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_code_suggestions_toggleable(&self) -> bool {
        false
    }

    /// Returns whether AI autonomy is allowed.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_custom_inference_enabled(&self) -> bool {
        false
    }

    /// Returns whether BYO API key is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_byo_api_key_enabled(&self) -> bool {
        false
    }

    /// Returns whether AWS Bedrock credentials are enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_aws_bedrock_credentials_enabled(&self) -> bool {
        false
    }

    /// Returns whether AWS Bedrock is available from workspace.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_aws_bedrock_available_from_workspace(&self) -> bool {
        false
    }

    /// Returns whether AWS Bedrock credentials are toggleable.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_aws_bedrock_credentials_toggleable(&self) -> bool {
        false
    }

    /// Returns whether user has teams.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn has_teams(&self) -> bool {
        false
    }

    /// Returns AWS Bedrock host enablement setting.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn aws_bedrock_host_enablement_setting(&self) -> AdminEnablementSetting {
        AdminEnablementSetting::Disabled
    }

    /// Returns upgrade link.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn upgrade_link(&self) -> Option<String> {
        None
    }
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_ai_autonomy_allowed(&self) -> bool {
        false
    }

    /// Returns the current workspace.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn current_workspace(&self) -> Option<&WorkspaceMetadata> {
        None
    }

    /// Returns UGC collection enablement setting.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn get_ugc_collection_enablement_setting(&self) -> crate::settings::AdminEnablementSetting {
        crate::settings::AdminEnablementSetting::Disabled
    }

    /// Returns AI autonomy settings.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn ai_autonomy_settings(&self) -> crate::settings::AiAutonomySettings {
        crate::settings::AiAutonomySettings::default()
    }


    /// Returns whether next command is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_next_command_enabled(&self) -> bool {
        false
    }

    /// Returns whether git operations AI is enabled.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn is_git_operations_ai_enabled(&self) -> bool {
        false
    }

    /// Returns agent attribution setting.
    /// COMMENTED: Cloud feature disabled in local version
    pub fn get_agent_attribution_setting(&self) -> crate::settings::AdminEnablementSetting {
        crate::settings::AdminEnablementSetting::Disabled
    }
}

/// Minimal stub for ObjectIdType - used in sqlite persistence
/// COMMENTED: Cloud feature disabled in local version
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectIdType {
    Notebook,
    Workflow,
    EnvVarCollection,
    AIFact,
    MCPServer,
    Folder,
    AIConversation,
    Command,
}

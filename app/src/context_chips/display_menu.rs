use std::cmp;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use fuzzy_match::{match_indices_case_insensitive, FuzzyMatchResult};
use instant::Instant;
use pathfinder_geometry::vector::vec2f;
use cute_core::ui::appearance::Appearance;
use cute_core::ui::builder::MIN_FONT_SIZE;
use cute_core::ui::theme::color::internal_colors;
use cute_core::ui::theme::Fill;
use cute_editor::editor::NavigationKey;
use cuteui::clipboard::ClipboardContent;
use cuteui::color::ColorU;
use cuteui::elements::{
    Border, ChildAnchor, ChildView, ClippedScrollStateHandle, ClippedScrollable, ConstrainedBox,
    Container, CornerRadius, CrossAxisAlignment, Dismiss, DispatchEventResult, DropShadow, Empty,
    EventHandler, Flex, Highlight, Hoverable, MainAxisAlignment, MainAxisSize, MouseInBehavior,
    MouseStateHandle, OffsetPositioning, ParentAnchor, ParentElement, ParentOffsetBounds,
    PositionedElementAnchor, PositionedElementOffsetBounds, Radius, SavePosition,
    ScrollStateHandle, Scrollable, ScrollableElement, ScrollbarWidth, Shrinkable, Stack, Text,
    UniformList, UniformListState,
};
use cuteui::fonts::{Properties, Weight};
use cuteui::keymap::FixedBinding;
use cuteui::platform::Cursor;
use cuteui::r#async::Timer;
use cuteui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use cuteui::units::Pixels;
use cuteui::{
    AppContext, Element, Entity, FocusContext, SingletonEntity as _, TypedActionView, View,
    ViewContext, ViewHandle, WindowId,
};

use crate::ai::cloud_environments::CloudAmbientAgentEnvironment;
use crate::cloud_object::model::generic_string_model::StringModel;
use crate::cloud_object::CloudObjectLookup as _;
use crate::editor::{
    EditorOptions, EditorView, Event as EditorEvent, PropagateAndNoOpNavigationKeys, TextOptions,
};
use crate::server::ids::{ClientId, HashableId, ServerId, SyncId};
use crate::ui_components::icons::Icon;
use crate::view_components::copyable_text_field::{
    render_copyable_text_field, CopyButtonPlacement, CopyableTextFieldConfig,
    COPY_FEEDBACK_DURATION,
};

/// Trait for items that can be displayed in a generic menu
pub trait GenericMenuItem: Debug + 'static {
    /// Enable downcasting to concrete types
    fn as_any(&self) -> &dyn std::any::Any;

    /// Display name for the menu item
    fn name(&self) -> String;

    /// Icon to display for the menu item (None for no icon)
    fn icon(&self, _app: &AppContext) -> Option<Icon>;

    /// Data associated with this menu item action
    fn action_data(&self) -> String;

    /// Optional element to render on the right side of the menu item
    fn right_side_element(&self, _app: &AppContext) -> Option<Box<dyn Element>> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct FixedFooter {
    action_item: Arc<dyn GenericMenuItem>,
    mouse_state: MouseStateHandle,
}

impl FixedFooter {
    pub fn new(action_item: Arc<dyn GenericMenuItem>) -> Self {
        Self {
            action_item,
            mouse_state: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChipMenuType {
    Directories,
    Branches,
    CodeReview,
    Environments,
}

const LABEL_HORIZONTAL_PADDING: f32 = 12.;
const SEARCH_INPUT_HORIZONTAL_PADDING: f32 = 8.;
const LABEL_VERTICAL_PADDING: f32 = 6.;
const MENU_VERTICAL_PADDING: f32 = 8.;
const MENU_WIDTH: f32 = 320.;
const COMMIT_PANEL_WIDTH: f32 = 380.;
const MENU_MAX_HEIGHT: f32 = 400.;

// Environments menu sizing from Figma mock.
const ENV_MENU_WIDTH: f32 = 321.;
const ENV_MENU_MAX_HEIGHT: f32 = 200.;
const ENV_MENU_VERTICAL_PADDING: f32 = 4.;
const ENV_MENU_ITEM_HORIZONTAL_PADDING: f32 = 16.;
const ENV_MENU_ITEM_VERTICAL_PADDING: f32 = 4.;
const ENV_MENU_ICON_SIZE: f32 = 16.;
const ENV_MENU_ICON_SLOT_SIZE: f32 = 16.;
const ENV_MENU_ITEM_FONT_SIZE: f32 = 14.;
const ENV_MENU_SEARCH_VERTICAL_PADDING: f32 = 4.;
// Bottom padding under the search field. The model selector's bottom padding
// is effectively `SEARCH_VERTICAL_PADDING (4) + MENU_CONTENT_VERTICAL_PADDING
// (4) = 8` because its `Menu` wraps the pinned footer in another 4px of
// content padding. We don't have that wrapper, so we bake the same 8px
// directly into the footer container.
const ENV_MENU_SEARCH_BOTTOM_PADDING: f32 = 8.;
const ENV_MENU_SEARCH_FOOTER_TOP_MARGIN: f32 = 4.;

// Environments sidecar sizing from Figma mock.
const ENV_SIDE_CAR_WIDTH: f32 = 320.;
const ENV_SIDE_CAR_HEIGHT: f32 = 108.;
const ENV_SIDE_CAR_HORIZONTAL_GAP: f32 = 1.;
const ENV_SIDE_CAR_PADDING: f32 = 12.;
const ENV_SIDE_CAR_ROW_GAP: f32 = 8.;
const ENV_SIDE_CAR_ICON_LABEL_GAP: f32 = 4.;
const ENV_SIDE_CAR_ICON_SIZE: f32 = 12.;
const ENV_SIDE_CAR_COPY_BUTTON_SIZE: f32 = 16.;
const ENV_SIDE_CAR_OUTER_RADIUS: f32 = 6.;
const ENV_SIDE_CAR_INNER_RADIUS: f32 = 4.;

pub fn init(app: &mut AppContext) {
    use cuteui::keymap::macros::*;

    app.register_fixed_bindings([
        FixedBinding::new(
            "up",
            DisplayChipMenuAction::SelectUp,
            id!(DisplayChipMenu::ui_name()),
        ),
        FixedBinding::new(
            "down",
            DisplayChipMenuAction::SelectDown,
            id!(DisplayChipMenu::ui_name()),
        ),
        FixedBinding::new(
            "escape",
            DisplayChipMenuAction::Close,
            id!(DisplayChipMenu::ui_name()),
        ),
        FixedBinding::new(
            "enter",
            DisplayChipMenuAction::SelectEnter,
            id!(DisplayChipMenu::ui_name()),
        ),
    ]);
}

#[derive(Debug, Clone)]
struct FilteredMenuItem {
    item: Arc<dyn GenericMenuItem>,
    match_result: Option<FuzzyMatchResult>,
}

#[derive(Clone, Debug)]
struct EnvironmentSidecarData {
    name: String,
    id: String,
    image: String,
    repos_text: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum EnvironmentSidecarSide {
    Left,
    Right,
}

/// Builds an optional synthetic menu item from the current search query.
///
/// When set, [`DisplayChipMenu`] calls the builder on every search-query
/// change. If the builder returns `Some(item)` and no existing menu item
/// already has the same name (compared ASCII case-insensitively), the
/// returned item is prepended to the filtered results so the user can act on
/// the unmatched query (for example, "Create new branch <name>"). The
/// builder itself is responsible for validating the query (e.g. rejecting
/// empty / invalid inputs) and returning `None` when no synthetic item
/// should be offered.
pub type CreateItemFromQueryFn =
    dyn Fn(&str) -> Option<Arc<dyn GenericMenuItem>> + Send + Sync + 'static;

/// Returns whether `query` matches any of `item_names`, ignoring ASCII case.
///
/// Used by [`DisplayChipMenu::update_filtered_items`] to suppress the
/// "create from query" affordance when an existing item already covers the
/// query. The comparison is case-insensitive on purpose: case-insensitive
/// filesystems (the default on macOS and Windows) treat refs like `main` and
/// `Main` as the same branch, so offering "Create new branch \"Main\"" while
/// `main` already exists would just hand the user a `branch already exists`
/// failure from git.
fn query_matches_existing_name<I, S>(item_names: I, query: &str) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    item_names
        .into_iter()
        .any(|name| name.as_ref().eq_ignore_ascii_case(query))
}

pub struct DisplayChipMenu {
    list_state: UniformListState,
    scroll_state: ScrollStateHandle,
    menu_items: Vec<Arc<dyn GenericMenuItem>>,
    filtered_items: Vec<FilteredMenuItem>,
    selected_index: usize,
    is_footer_selected: bool,
    fixed_footer: Option<FixedFooter>,
    search_input: Option<ViewHandle<EditorView>>,
    search_query: String,
    chip_menu_type: ChipMenuType,
    /// When set, the menu offers a synthetic "create from query" item whenever
    /// the user's query doesn't exactly match an existing item. See
    /// [`CreateItemFromQueryFn`].
    create_item_from_query: Option<Arc<CreateItemFromQueryFn>>,

    // Environment sidecar state
    window_id: WindowId,
    env_sidecar_copy_id_mouse_state: MouseStateHandle,
    env_sidecar_copy_image_mouse_state: MouseStateHandle,
    env_sidecar_copy_feedback_times: HashMap<String, Instant>,
    env_sidecar_scroll_state: ClippedScrollStateHandle,

    // Branch history panel state (for ChipMenuType::Branches)
    selected_branch_name: Option<String>,
    selected_branch_commits: Vec<crate::util::git::Commit>,
    current_branch_name: Option<String>,
    is_loading_branches: bool,
    is_loading_commits: bool,
    load_error: Option<String>,
    commit_list_state: UniformListState,
    commit_scroll_state: ScrollStateHandle,

    // Commit files panel state (third pane)
    selected_commit_hash: Option<String>,
    selected_commit_files: Vec<crate::util::git::FileChangeEntry>,
    is_loading_files: bool,
    file_list_state: UniformListState,
    file_scroll_state: ScrollStateHandle,

    // Branch context menu state (right-click)
    branch_context_menu: Option<BranchContextMenu>,
    branch_context_menu_position: Option<cuteui::geometry::vector::Vector2F>,
    branch_context_menu_mouse_states: Vec<MouseStateHandle>,
}

/// Context menu for branch operations (shown on right-click)
#[derive(Debug, Clone)]
pub struct BranchContextMenu {
    pub branch_name: String,
    pub is_remote: bool,
    pub is_current_branch: bool,
}

#[derive(Debug, Clone)]
pub enum BranchContextMenuAction {
    MergeIntoCurrent { branch_name: String },
    Checkout { branch_name: String },
    Delete { branch_name: String },
    Rename { branch_name: String },
    CopyBranchName { branch_name: String },
}

#[derive(Debug, Clone)]
pub enum DisplayChipMenuAction {
    SelectItem {
        index: usize,
    },
    Select {
        index: usize,
    },
    SelectUp,
    SelectDown,
    SelectEnter,
    SelectFixedFooterOption,
    CopyEnvironmentSidecarField {
        key: String,
        value: String,
    },
    Close,
    ShowBranchContextMenu {
        index: usize,
        position: cuteui::geometry::vector::Vector2F,
    },
    ExecuteBranchAction {
        action: BranchContextMenuAction,
    },
    CloseBranchContextMenu,
    SelectCommit {
        commit_hash: String,
    },
}

impl DisplayChipMenu {
    fn menu_width(&self) -> f32 {
        match self.chip_menu_type {
            ChipMenuType::Environments => ENV_MENU_WIDTH,
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                MENU_WIDTH
            }
        }
    }

    fn menu_item_horizontal_padding(&self) -> f32 {
        match self.chip_menu_type {
            ChipMenuType::Environments => ENV_MENU_ITEM_HORIZONTAL_PADDING,
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                LABEL_HORIZONTAL_PADDING
            }
        }
    }

    fn menu_item_vertical_padding(&self) -> f32 {
        match self.chip_menu_type {
            ChipMenuType::Environments => ENV_MENU_ITEM_VERTICAL_PADDING,
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                LABEL_VERTICAL_PADDING
            }
        }
    }

    fn menu_vertical_padding(&self) -> f32 {
        match self.chip_menu_type {
            ChipMenuType::Environments => ENV_MENU_VERTICAL_PADDING,
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                MENU_VERTICAL_PADDING
            }
        }
    }

    fn figma_menu_drop_shadow() -> DropShadow {
        DropShadow::default()
    }

    pub fn new<T: GenericMenuItem>(
        menu_items: Vec<T>,
        fixed_footer_option: Option<FixedFooter>,
        chip_menu_type: ChipMenuType,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let search_input = match chip_menu_type {
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::Environments => {
                Some(ctx.add_typed_action_view(|ctx| {
                    let appearance = Appearance::handle(ctx).as_ref(ctx);

                    let text_options = match chip_menu_type {
                        ChipMenuType::Environments => {
                            TextOptions::ui_text(Some(ENV_MENU_ITEM_FONT_SIZE), appearance)
                        }
                        ChipMenuType::Directories
                        | ChipMenuType::Branches
                        | ChipMenuType::CodeReview => {
                            let ui_font_family = appearance.ui_font_family();
                            let mut options = TextOptions::ui_font_size(appearance);
                            options.font_family_override = Some(ui_font_family);
                            options
                        }
                    };

                    let options = EditorOptions {
                        autogrow: false,
                        soft_wrap: false,
                        single_line: true,
                        text: text_options,
                        propagate_and_no_op_vertical_navigation_keys:
                            PropagateAndNoOpNavigationKeys::Always,
                        ..Default::default()
                    };
                    let mut editor = EditorView::new(options, ctx);
                    let placeholder_text = match chip_menu_type {
                        ChipMenuType::Directories => "Search directories...",
                        ChipMenuType::Branches => "Search branches...",
                        ChipMenuType::Environments => "Search environments...",
                        ChipMenuType::CodeReview => {
                            unreachable!("search input should not be constructed")
                        }
                    };
                    editor.set_placeholder_text(placeholder_text, ctx);
                    editor
                }))
            }
            ChipMenuType::CodeReview => None,
        };

        // Subscribe to editor changes to update search query (only if search input exists)
        if let Some(ref search_input_handle) = search_input {
            ctx.subscribe_to_view(
                search_input_handle,
                |menu, _editor, event, ctx| match event {
                    EditorEvent::Edited(_) => {
                        if let Some(ref search_input) = menu.search_input {
                            let new_query = search_input
                                .read(ctx, |editor, ctx| editor.buffer_text(ctx).to_string());
                            if new_query != menu.search_query {
                                menu.update_search_query(new_query, ctx);
                            }
                        }
                    }
                    EditorEvent::Escape => {
                        menu.close(ctx);
                    }
                    EditorEvent::Navigate(NavigationKey::Up) => {
                        menu.select_prev(ctx);
                    }
                    EditorEvent::Navigate(NavigationKey::Down) => {
                        menu.select_next(ctx);
                    }
                    EditorEvent::Enter => {
                        menu.select_enter(ctx);
                    }
                    _ => {}
                },
            );
        }

        let menu_items: Vec<Arc<dyn GenericMenuItem>> = menu_items
            .into_iter()
            .map(|value| {
                let arc: Arc<dyn GenericMenuItem> = Arc::new(value);
                arc
            })
            .collect();

        let filtered_items: Vec<FilteredMenuItem> = menu_items
            .iter()
            .map(|item| FilteredMenuItem {
                item: item.clone(),
                match_result: None,
            })
            .collect();

        // Always start selection at the top (first item) for consistent behavior
        let initial_selected_index = 0;

        Self {
            list_state: Default::default(),
            scroll_state: Default::default(),
            menu_items,
            filtered_items,
            selected_index: initial_selected_index,
            fixed_footer: fixed_footer_option,
            is_footer_selected: false,
            search_input,
            search_query: String::new(),
            chip_menu_type,
            create_item_from_query: None,

            window_id: ctx.window_id(),
            env_sidecar_copy_id_mouse_state: Default::default(),
            env_sidecar_copy_image_mouse_state: Default::default(),
            env_sidecar_copy_feedback_times: HashMap::new(),
            env_sidecar_scroll_state: Default::default(),

            selected_branch_name: None,
            selected_branch_commits: Vec::new(),
            current_branch_name: None,
            is_loading_branches: false,
            is_loading_commits: false,
            load_error: None,
            commit_list_state: Default::default(),
            commit_scroll_state: Default::default(),

            // Commit files panel state
            selected_commit_hash: None,
            selected_commit_files: Vec::new(),
            is_loading_files: false,
            file_list_state: Default::default(),
            file_scroll_state: Default::default(),

            branch_context_menu: None,
            branch_context_menu_position: None,
            branch_context_menu_mouse_states: Vec::new(),
        }
    }

    /// Set the current branch name for highlighting
    pub fn set_current_branch(&mut self, branch_name: Option<String>) {
        self.current_branch_name = branch_name;
    }

    /// Set loading state for branches
    pub fn set_loading_branches(&mut self, loading: bool) {
        self.is_loading_branches = loading;
    }

    /// Set loading state for commits
    pub fn set_loading_commits(&mut self, loading: bool) {
        self.is_loading_commits = loading;
    }

    /// Set error message
    pub fn set_error(&mut self, error: Option<String>) {
        self.load_error = error;
    }

    /// Register a builder that produces a synthetic top-of-list item for
    /// otherwise-unmatched search queries. See [`CreateItemFromQueryFn`].
    pub fn with_create_item_from_query(mut self, builder: Arc<CreateItemFromQueryFn>) -> Self {
        self.create_item_from_query = Some(builder);
        self
    }

    pub fn reset_selected_index(&mut self) {
        if self.filtered_items.is_empty() && self.fixed_footer.is_some() {
            self.is_footer_selected = true;
            return;
        }
        self.selected_index = 0;
        self.is_footer_selected = false;
        self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
    }

    /// Update the menu items and reset the selected index
    pub fn update_menu_items<T: GenericMenuItem>(
        &mut self,
        new_items: Vec<T>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.menu_items = new_items
            .into_iter()
            .map(|value| {
                let arc: Arc<dyn GenericMenuItem> = Arc::new(value);
                arc
            })
            .collect();
        self.update_filtered_items();
        self.reset_selected_index();

        // Scroll to the selected item
        if !self.filtered_items.is_empty() {
            self.list_state.scroll_to(self.selected_index);
        }

        ctx.notify();
    }

    /// Update the selected branch and its commit history
    pub fn update_selected_branch(
        &mut self,
        branch_name: String,
        commits: Vec<crate::util::git::Commit>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.selected_branch_name = Some(branch_name);
        self.selected_branch_commits = commits;
        // Reset commit selection when branch changes
        self.selected_commit_hash = None;
        self.selected_commit_files = Vec::new();
        ctx.notify();
    }

    /// Update the selected commit and its file changes
    pub fn update_selected_commit(
        &mut self,
        commit_hash: String,
        files: Vec<crate::util::git::FileChangeEntry>,
        ctx: &mut ViewContext<Self>,
    ) {
        self.selected_commit_hash = Some(commit_hash);
        self.selected_commit_files = files;
        ctx.notify();
    }

    /// Set loading state for commit files
    pub fn set_loading_files(&mut self, loading: bool) {
        self.is_loading_files = loading;
    }

    fn update_filtered_items(&mut self) {
        if self.search_query.is_empty() {
            // No search query - show all items
            self.filtered_items = self
                .menu_items
                .iter()
                .map(|item| FilteredMenuItem {
                    item: item.clone(),
                    match_result: None,
                })
                .collect();
            return;
        }

        // Filter items based on search query
        self.filtered_items = self
            .menu_items
            .iter()
            .filter_map(|item| {
                let item_name = item.name();
                match_indices_case_insensitive(&item_name, &self.search_query).map(|match_result| {
                    FilteredMenuItem {
                        item: item.clone(),
                        match_result: Some(match_result),
                    }
                })
            })
            .collect();

        // Sort by match score (higher scores first)
        self.filtered_items.sort_by(|a, b| {
            let score_a = a.match_result.as_ref().map(|r| r.score).unwrap_or(0);
            let score_b = b.match_result.as_ref().map(|r| r.score).unwrap_or(0);
            score_b.cmp(&score_a)
        });

        // Offer a synthetic top-of-list "create from query" item when the
        // current query has no exact match against an existing item. This is
        // what powers the "Create new branch …" affordance in the branch
        // switcher.
        if let Some(builder) = self.create_item_from_query.as_ref() {
            let trimmed = self.search_query.trim();
            let already_matches_existing = query_matches_existing_name(
                self.menu_items.iter().map(|item| item.name()),
                trimmed,
            );
            if !already_matches_existing {
                if let Some(synthetic) = builder(trimmed) {
                    self.filtered_items.insert(
                        0,
                        FilteredMenuItem {
                            item: synthetic,
                            match_result: None,
                        },
                    );
                }
            }
        }
    }

    pub fn update_search_query(&mut self, query: String, ctx: &mut ViewContext<Self>) {
        self.search_query = query;
        self.update_filtered_items();

        // Always start at the top after filtering for consistent behavior
        self.reset_selected_index();
        if !self.filtered_items.is_empty() {
            self.list_state.scroll_to(self.selected_index);
        }

        ctx.notify();
    }

    fn select_item(&mut self, item: Arc<dyn GenericMenuItem>, ctx: &mut ViewContext<Self>) {
        ctx.emit(PromptDisplayMenuEvent::MenuAction(GenericMenuEvent {
            action_item: item.clone(),
        }));
        ctx.notify();
    }

    fn select(&mut self, index: usize, ctx: &mut ViewContext<Self>) {
        if self.selected_index != index {
            self.selected_index = index;
            self.env_sidecar_scroll_state.scroll_to(Pixels::zero());

            // For branches, emit event to fetch commit history
            if self.chip_menu_type == ChipMenuType::Branches && index < self.filtered_items.len() {
                let item = self.filtered_items[index].item.clone();
                ctx.emit(PromptDisplayMenuEvent::BranchSelected {
                    branch_name: item.name(),
                });
            }
        }
        ctx.notify();
    }

    pub fn select_index(&mut self, index: usize, ctx: &mut ViewContext<Self>) {
        if index >= self.filtered_items.len() {
            return;
        }
        self.is_footer_selected = false;
        self.select(index, ctx);
        self.list_state.scroll_to(self.selected_index);
    }

    fn is_footer_selected(&self) -> bool {
        self.is_footer_selected
            || self
                .fixed_footer
                .as_ref()
                .is_some_and(|f| f.mouse_state.lock().is_ok_and(|state| state.is_hovered()))
    }

    fn select_prev(&mut self, ctx: &mut ViewContext<Self>) {
        if self.filtered_items.is_empty() {
            return;
        }
        let has_footer = self.fixed_footer.is_some();

        if self.selected_index == 0 {
            if has_footer && !self.is_footer_selected() {
                self.is_footer_selected = true;
            } else {
                self.is_footer_selected = false;
                self.selected_index = self.filtered_items.len() - 1;
                self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
            }
        } else {
            self.is_footer_selected = false;
            self.selected_index -= 1;
            self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
        }
        self.list_state.scroll_to(self.selected_index);
        ctx.notify();
    }

    fn select_next(&mut self, ctx: &mut ViewContext<Self>) {
        if self.filtered_items.is_empty() {
            return;
        }
        let has_footer = self.fixed_footer.is_some();

        self.selected_index += 1;
        if self.is_footer_selected() {
            self.is_footer_selected = false;
            self.selected_index = 0;
            self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
        } else if self.selected_index >= self.filtered_items.len() {
            self.selected_index = 0;
            self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
            if has_footer && !self.is_footer_selected() {
                self.is_footer_selected = true;
            }
        } else {
            self.env_sidecar_scroll_state.scroll_to(Pixels::zero());
        }
        self.list_state.scroll_to(self.selected_index);
        ctx.notify();
    }

    fn select_enter(&mut self, ctx: &mut ViewContext<Self>) {
        if self.is_footer_selected() {
            self.select_fixed_footer_option(ctx);
            return;
        }

        if self.selected_index < self.filtered_items.len() {
            let item = self.filtered_items[self.selected_index].item.clone();
            self.select_item(item, ctx);
        }
    }

    fn select_fixed_footer_option(&mut self, ctx: &mut ViewContext<Self>) {
        if let Some(footer_option) = &self.fixed_footer {
            ctx.emit(PromptDisplayMenuEvent::MenuAction(GenericMenuEvent {
                action_item: footer_option.action_item.clone(),
            }));
            ctx.notify();
        }
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(PromptDisplayMenuEvent::CloseMenu);
        ctx.notify();
    }

    fn should_show_environment_sidecar(&self) -> bool {
        self.chip_menu_type == ChipMenuType::Environments
            && !self.is_footer_selected()
            && self.selected_index < self.filtered_items.len()
    }

    fn parse_sync_id_lossy(s: &str) -> SyncId {
        if let Some(hashed) = ClientId::from_hash(s) {
            SyncId::ClientId(hashed)
        } else {
            SyncId::ServerId(ServerId::from_string_lossy(s))
        }
    }

    fn environment_sidecar_data(&self, app: &AppContext) -> Option<EnvironmentSidecarData> {
        if !self.should_show_environment_sidecar() {
            return None;
        }

        let item = self.filtered_items.get(self.selected_index)?.item.clone();
        let sync_id = Self::parse_sync_id_lossy(&item.action_data());
        let env = CloudAmbientAgentEnvironment::get_by_id(&sync_id, app)?;

        let repo_names = env
            .model()
            .string_model
            .github_repos
            .iter()
            .map(|repo| repo.repo.clone())
            .collect::<Vec<_>>();
        let repos_text = if repo_names.is_empty() {
            "(none)".to_string()
        } else {
            repo_names.join(", ")
        };

        Some(EnvironmentSidecarData {
            name: env.model().string_model.display_name(),
            id: env.id.to_string(),
            image: env.model().string_model.base_image.to_string(),
            repos_text,
        })
    }

    fn environment_sidecar_anchor_id(&self) -> Option<String> {
        if !self.should_show_environment_sidecar() {
            return None;
        }

        Some(format!("MenuPromptChip-{}", self.selected_index))
    }

    fn environment_sidecar_side(
        &self,
        position_id: &str,
        app: &AppContext,
    ) -> EnvironmentSidecarSide {
        let Some(window) = app.windows().platform_window(self.window_id) else {
            return EnvironmentSidecarSide::Left;
        };

        // Anchor is the currently selected/hovered row.
        let Some(anchor_rect) =
            app.element_position_by_id_at_last_frame(self.window_id, position_id)
        else {
            return EnvironmentSidecarSide::Left;
        };

        let gap = ENV_SIDE_CAR_HORIZONTAL_GAP;
        let window_width = window.size().x();

        // If sidecar is on the right of the anchor.
        let right_edge_if_on_right = anchor_rect.max_x() + gap + ENV_SIDE_CAR_WIDTH;
        let overflow_right = (right_edge_if_on_right - window_width).max(0.);

        // If sidecar is on the left of the anchor.
        let left_edge_if_on_left = anchor_rect.min_x() - gap - ENV_SIDE_CAR_WIDTH;
        let overflow_left = (0. - left_edge_if_on_left).max(0.);

        let would_overflow_right = overflow_right > 0.;
        let would_overflow_left = overflow_left > 0.;

        match (would_overflow_left, would_overflow_right) {
            (true, false) => EnvironmentSidecarSide::Right,
            (false, true) => EnvironmentSidecarSide::Left,
            (false, false) => EnvironmentSidecarSide::Left,
            (true, true) => {
                if overflow_left <= overflow_right {
                    EnvironmentSidecarSide::Left
                } else {
                    EnvironmentSidecarSide::Right
                }
            }
        }
    }

    fn environment_sidecar_positioning(
        &self,
        position_id: String,
        app: &AppContext,
    ) -> Option<OffsetPositioning> {
        // Ensure anchor rect exists in cache; otherwise positioning will be wrong.
        app.element_position_by_id_at_last_frame(self.window_id, &position_id)?;

        let side = self.environment_sidecar_side(&position_id, app);
        let offset_y = -ENV_MENU_VERTICAL_PADDING;

        Some(match side {
            EnvironmentSidecarSide::Right => OffsetPositioning::offset_from_save_position_element(
                position_id,
                vec2f(ENV_SIDE_CAR_HORIZONTAL_GAP, offset_y),
                PositionedElementOffsetBounds::WindowByPosition,
                PositionedElementAnchor::TopRight,
                ChildAnchor::TopLeft,
            ),
            EnvironmentSidecarSide::Left => OffsetPositioning::offset_from_save_position_element(
                position_id,
                vec2f(-ENV_SIDE_CAR_HORIZONTAL_GAP, offset_y),
                PositionedElementOffsetBounds::WindowByPosition,
                PositionedElementAnchor::TopLeft,
                ChildAnchor::TopRight,
            ),
        })
    }

    fn environment_sidecar_overlay(
        &self,
        app: &AppContext,
    ) -> Option<(Box<dyn Element>, OffsetPositioning)> {
        let data = self.environment_sidecar_data(app)?;
        let position_id = self.environment_sidecar_anchor_id()?;
        let positioning = self.environment_sidecar_positioning(position_id, app)?;
        Some((self.render_environment_sidecar(&data, app), positioning))
    }

    fn render_environment_sidecar(
        &self,
        data: &EnvironmentSidecarData,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        let background = theme.surface_2();
        let label_text_color = theme.sub_text_color(background).into_solid();
        let main_text_color = theme.main_text_color(background).into_solid();

        let label_font_size = 12.;
        let value_font_size = 12.;

        let id_key = format!("env-sidecar:{}:id", data.id);
        let image_key = format!("env-sidecar:{}:image", data.id);

        let icon = |icon: Icon| {
            ConstrainedBox::new(icon.to_cuteui_icon(Fill::Solid(label_text_color)).finish())
                .with_width(ENV_SIDE_CAR_ICON_SIZE)
                .with_height(ENV_SIDE_CAR_ICON_SIZE)
                .finish()
        };

        let label_text = |text: &str| {
            Text::new_inline(
                text.to_string(),
                appearance.ui_font_family(),
                label_font_size,
            )
            .with_color(label_text_color)
            .finish()
        };

        let value_text = |text: String| {
            Text::new(text, appearance.ui_font_family(), value_font_size)
                .with_color(main_text_color)
                .with_selectable(true)
                .finish()
        };

        let id_value = {
            let env_id = data.id.clone();
            render_copyable_text_field(
                CopyableTextFieldConfig::new(env_id.clone())
                    .with_font_size(value_font_size)
                    .with_text_color(main_text_color)
                    .with_icon_size(ENV_SIDE_CAR_COPY_BUTTON_SIZE)
                    .with_mouse_state(self.env_sidecar_copy_id_mouse_state.clone())
                    .with_last_copied_at(self.env_sidecar_copy_feedback_times.get(&id_key))
                    .with_wrap_text(true)
                    .with_cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_copy_button_placement(CopyButtonPlacement::NextToText),
                move |ctx| {
                    ctx.dispatch_typed_action(DisplayChipMenuAction::CopyEnvironmentSidecarField {
                        key: id_key.clone(),
                        value: env_id.clone(),
                    });
                },
                app,
            )
        };

        let image_value = {
            let docker_image = data.image.clone();
            render_copyable_text_field(
                CopyableTextFieldConfig::new(docker_image.clone())
                    .with_font_size(value_font_size)
                    .with_text_color(main_text_color)
                    .with_icon_size(ENV_SIDE_CAR_COPY_BUTTON_SIZE)
                    .with_mouse_state(self.env_sidecar_copy_image_mouse_state.clone())
                    .with_last_copied_at(self.env_sidecar_copy_feedback_times.get(&image_key))
                    .with_wrap_text(true)
                    .with_cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_copy_button_placement(CopyButtonPlacement::NextToText),
                move |ctx| {
                    ctx.dispatch_typed_action(DisplayChipMenuAction::CopyEnvironmentSidecarField {
                        key: image_key.clone(),
                        value: docker_image.clone(),
                    });
                },
                app,
            )
        };

        let row = |row_icon: Icon, label: &str, value: Box<dyn Element>, is_last: bool| {
            let label_cluster = Flex::row()
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(
                    Container::new(icon(row_icon))
                        .with_margin_right(ENV_SIDE_CAR_ICON_LABEL_GAP)
                        .finish(),
                )
                .with_child(label_text(label))
                .finish();

            let element = Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Container::new(label_cluster)
                        .with_margin_right(ENV_SIDE_CAR_ROW_GAP)
                        .finish(),
                )
                .with_child(Shrinkable::new(1., value).finish())
                .finish();

            if is_last {
                element
            } else {
                Container::new(element)
                    .with_margin_bottom(ENV_SIDE_CAR_ROW_GAP)
                    .finish()
            }
        };

        let content = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(row(
                Icon::Globe4,
                "Name:",
                value_text(data.name.clone()),
                false,
            ))
            .with_child(row(Icon::Hash, "ID:", id_value, false))
            .with_child(row(Icon::Docker, "Image:", image_value, false))
            .with_child(row(
                Icon::Github,
                "Repos:",
                value_text(data.repos_text.clone()),
                true,
            ))
            .finish();

        let scrollable_content = ClippedScrollable::vertical(
            self.env_sidecar_scroll_state.clone(),
            content,
            ScrollbarWidth::Auto,
            theme.nonactive_ui_detail().into(),
            theme.active_ui_detail().into(),
            // Leave the scrollbar gutter background transparent.
            cuteui::elements::Fill::None,
        )
        .with_padding_start(0.)
        .with_padding_end(0.)
        .with_overlayed_scrollbar()
        .finish();

        let inner = Container::new(scrollable_content)
            .with_uniform_padding(ENV_SIDE_CAR_PADDING)
            .with_border(
                Border::all(1.).with_border_fill(Fill::Solid(internal_colors::neutral_2(theme))),
            )
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(
                ENV_SIDE_CAR_INNER_RADIUS,
            )))
            .finish();

        let outer = Container::new(inner)
            .with_background(background)
            .with_border(
                Border::all(1.).with_border_fill(Fill::Solid(internal_colors::neutral_4(theme))),
            )
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(
                ENV_SIDE_CAR_OUTER_RADIUS,
            )))
            .with_drop_shadow(Self::figma_menu_drop_shadow())
            .finish();

        ConstrainedBox::new(outer)
            .with_width(ENV_SIDE_CAR_WIDTH)
            .with_min_height(ENV_SIDE_CAR_HEIGHT)
            .finish()
    }

    fn render_fixed_footer_option(
        &self,
        app: &AppContext,
        footer_option: &FixedFooter,
    ) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        let chip_menu_type = self.chip_menu_type;
        let (font_size, icon_size) = match chip_menu_type {
            ChipMenuType::Environments => (ENV_MENU_ITEM_FONT_SIZE, ENV_MENU_ICON_SIZE),
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                let font_size = appearance.ui_font_size();
                (font_size, font_size * 0.8)
            }
        };

        let item_horizontal_padding = self.menu_item_horizontal_padding();
        let item_vertical_padding = self.menu_item_vertical_padding();

        let is_footer_selected = self.is_footer_selected();
        ConstrainedBox::new(
            Hoverable::new(footer_option.mouse_state.clone(), move |mouse_state| {
                let is_active = mouse_state.is_hovered() || is_footer_selected;

                let background_color = if is_active {
                    match chip_menu_type {
                        ChipMenuType::Environments => Some(internal_colors::fg_overlay_4(theme)),
                        ChipMenuType::Directories
                        | ChipMenuType::Branches
                        | ChipMenuType::CodeReview => Some(theme.accent()),
                    }
                } else {
                    None
                };

                let text_color = if is_active {
                    match chip_menu_type {
                        ChipMenuType::Environments => {
                            theme.main_text_color(theme.surface_2()).into_solid()
                        }
                        ChipMenuType::Directories
                        | ChipMenuType::Branches
                        | ChipMenuType::CodeReview => {
                            theme.main_text_color(theme.accent()).into_solid()
                        }
                    }
                } else {
                    theme.sub_text_color(theme.surface_2()).into_solid()
                };

                // Update icon and text colors based on hover state
                let mut updated_text =
                    Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

                // Add icon if it exists
                if let Some(icon) = footer_option.action_item.icon(app) {
                    updated_text.add_child(
                        Container::new(
                            ConstrainedBox::new(
                                icon.to_cuteui_icon(Fill::Solid(text_color)).finish(),
                            )
                            .with_height(icon_size)
                            .with_width(icon_size)
                            .finish(),
                        )
                        .with_margin_right(8.)
                        .finish(),
                    );
                } else {
                    // Add spacing equivalent to icon + margin for alignment
                    updated_text.add_child(
                        ConstrainedBox::new(Empty::new().finish())
                            .with_width(icon_size + 8.)
                            .finish(),
                    );
                }

                // Add the text element
                updated_text.add_child(
                    Text::new_inline(
                        footer_option.action_item.name(),
                        appearance.ui_font_family(),
                        font_size,
                    )
                    .autosize_text(MIN_FONT_SIZE)
                    .with_color(text_color)
                    .finish(),
                );

                let mut container = Container::new(updated_text.finish())
                    .with_horizontal_padding(item_horizontal_padding)
                    .with_vertical_padding(item_vertical_padding)
                    .with_border(Border::top(1.0));

                if let Some(bg_color) = background_color {
                    container = container.with_background(bg_color);
                }

                container.finish()
            })
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(DisplayChipMenuAction::SelectFixedFooterOption);
            })
            .finish(),
        )
        .with_width(self.menu_width())
        .finish()
    }

    fn render_env_search_footer(
        &self,
        search_input: &ViewHandle<EditorView>,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let theme = Appearance::as_ref(app).theme();
        let divider_color = internal_colors::fg_overlay_2(theme);

        Container::new(ChildView::new(search_input).finish())
            .with_margin_top(ENV_MENU_SEARCH_FOOTER_TOP_MARGIN)
            .with_horizontal_padding(ENV_MENU_ITEM_HORIZONTAL_PADDING)
            .with_padding_top(ENV_MENU_SEARCH_VERTICAL_PADDING)
            .with_padding_bottom(ENV_MENU_SEARCH_BOTTOM_PADDING)
            .with_border(Border::top(1.).with_border_fill(divider_color))
            .finish()
    }

    fn render_items(&self, ctx: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(ctx);
        let theme = appearance.theme();

        // Show loading state for branches
        if self.is_loading_branches && self.chip_menu_type == ChipMenuType::Branches {
            return Container::new(
                Flex::column()
                    .with_main_axis_alignment(MainAxisAlignment::Center)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_child(
                        Text::new_inline(
                            "Loading branches...",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                        .finish(),
                    )
                    .finish(),
            )
            .with_horizontal_padding(LABEL_HORIZONTAL_PADDING)
            .with_vertical_padding(20.)
            .finish();
        }

        if self.filtered_items.is_empty() {
            // Show "No results" if search is active but no matches.
            if !self.search_query.is_empty() {
                // Match the model selector dropdown's no-results row exactly
                // for the environments menu (label, font size, paddings, and
                // text color); other chip menus keep their existing styling.
                let (label, font_size, horizontal_padding, vertical_padding, text_color) =
                    match self.chip_menu_type {
                        ChipMenuType::Environments => (
                            "No results",
                            ENV_MENU_ITEM_FONT_SIZE,
                            ENV_MENU_ITEM_HORIZONTAL_PADDING,
                            ENV_MENU_ITEM_VERTICAL_PADDING,
                            internal_colors::text_sub(theme, theme.surface_2()),
                        ),
                        ChipMenuType::Directories
                        | ChipMenuType::Branches
                        | ChipMenuType::CodeReview => (
                            "No results found",
                            appearance.ui_font_size(),
                            LABEL_HORIZONTAL_PADDING,
                            LABEL_VERTICAL_PADDING * 2.0,
                            theme.sub_text_color(theme.surface_2()).into_solid(),
                        ),
                    };
                return Container::new(
                    Text::new(label, appearance.ui_font_family(), font_size)
                        .with_color(text_color)
                        .finish(),
                )
                .with_horizontal_padding(horizontal_padding)
                .with_vertical_padding(vertical_padding)
                .finish();
            }
            // For branches with no items and no search query, return empty
            // The parent will show "No branches available" message
            if self.chip_menu_type == ChipMenuType::Branches {
                return Empty::new().finish();
            }
            return Empty::new().finish();
        }

        let selected_index = self.selected_index;
        let filtered_items_length = self.filtered_items.len();
        let filtered_items = self.filtered_items.clone();
        let is_footer_hovered = self.is_footer_selected();
        let menu_width = self.menu_width();
        let item_horizontal_padding = self.menu_item_horizontal_padding();
        let item_vertical_padding = self.menu_item_vertical_padding();
        let chip_menu_type = self.chip_menu_type;
        let current_branch_name = self.current_branch_name.clone();
        let list = UniformList::new(
            self.list_state.clone(),
            filtered_items.len(),
            move |mut range, app| {
                let appearance = Appearance::as_ref(app);
                let theme = appearance.theme();

                range.end = cmp::min(range.end, filtered_items.len());
                range
                    .map(|index| {
                        let filtered_item = &filtered_items[index];
                        let item = &filtered_item.item;
                        let display_text_str = item.name();
                        let display_text_str_for_indicator = display_text_str.clone();

                        let is_selected = index == selected_index && !is_footer_hovered;

                        let font_size = if matches!(chip_menu_type, ChipMenuType::Environments) {
                            ENV_MENU_ITEM_FONT_SIZE
                        } else {
                            appearance.ui_font_size()
                        };
                        let icon_size = font_size * 0.8; // Icon slightly smaller than text

                        let (main_text, selected_background) = match chip_menu_type {
                            ChipMenuType::Environments => (
                                theme.main_text_color(theme.surface_2()).into_solid(),
                                is_selected.then_some(internal_colors::fg_overlay_4(theme)),
                            ),
                            ChipMenuType::Directories
                            | ChipMenuType::Branches
                            | ChipMenuType::CodeReview => {
                                if is_selected {
                                    let bg = theme.accent();
                                    (theme.main_text_color(bg).into_solid(), Some(bg))
                                } else {
                                    (theme.main_text_color(theme.surface_2()).into_solid(), None)
                                }
                            }
                        };

                        // Create main container with SpaceBetween to float right elements to far right
                        let mut main_container = Flex::row()
                            .with_cross_axis_alignment(CrossAxisAlignment::Center)
                            .with_main_axis_size(MainAxisSize::Max);

                        // Create left side container with icon and main text
                        let mut left_side =
                            Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

                        let icon_gap = 8.;
                        if matches!(chip_menu_type, ChipMenuType::Environments) {
                            if let Some(icon) = item.icon(app) {
                                let icon_slot_size = ENV_MENU_ICON_SLOT_SIZE;
                                let glyph_size = ENV_MENU_ICON_SIZE;

                                let icon_glyph = ConstrainedBox::new(
                                    icon.to_cuteui_icon(Fill::Solid(main_text)).finish(),
                                )
                                .with_width(glyph_size)
                                .with_height(glyph_size)
                                .finish();

                                let icon_slot = ConstrainedBox::new(
                                    Flex::row()
                                        .with_main_axis_alignment(MainAxisAlignment::Center)
                                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                                        .with_child(icon_glyph)
                                        .finish(),
                                )
                                .with_width(icon_slot_size)
                                .with_height(icon_slot_size)
                                .finish();

                                left_side.add_child(
                                    Container::new(icon_slot)
                                        .with_margin_right(icon_gap)
                                        .finish(),
                                );
                            }
                        } else if let Some(icon) = item.icon(app) {
                            left_side.add_child(
                                Container::new(
                                    ConstrainedBox::new(
                                        icon.to_cuteui_icon(Fill::Solid(main_text)).finish(),
                                    )
                                    .with_height(icon_size)
                                    .with_width(icon_size)
                                    .finish(),
                                )
                                .with_margin_right(icon_gap)
                                .finish(),
                            );
                        } else {
                            // Add spacing equivalent to icon + margin for alignment
                            left_side.add_child(
                                ConstrainedBox::new(Empty::new().finish())
                                    .with_width(icon_size + icon_gap)
                                    .finish(),
                            );
                        }

                        // Create main text with highlighting if there's a match result
                        let display_text = if let Some(match_result) = &filtered_item.match_result {
                            Text::new_inline(
                                display_text_str,
                                appearance.ui_font_family(),
                                font_size,
                            )
                            .autosize_text(MIN_FONT_SIZE)
                            .with_color(main_text)
                            .with_single_highlight(
                                Highlight::new()
                                    .with_properties(Properties::default().weight(Weight::Bold))
                                    .with_foreground_color(main_text),
                                match_result.matched_indices.clone(),
                            )
                        } else {
                            Text::new_inline(
                                display_text_str,
                                appearance.ui_font_family(),
                                font_size,
                            )
                            .autosize_text(MIN_FONT_SIZE)
                            .with_color(main_text)
                        };

                        left_side.add_child(display_text.finish());

                        // Add current branch indicator
                        if chip_menu_type == ChipMenuType::Branches {
                            if let Some(ref current) = current_branch_name {
                                if display_text_str_for_indicator == *current {
                                    // Add a small indicator for current branch
                                    left_side.add_child(
                                        Container::new(
                                            Text::new_inline(
                                                " ●",
                                                appearance.ui_font_family(),
                                                font_size,
                                            )
                                            .with_color(theme.ansi_fg_green().into())
                                            .finish(),
                                        )
                                        .with_margin_left(4.)
                                        .finish(),
                                    );
                                }
                            }
                        }

                        // Add left side to main container
                        main_container.add_child(left_side.finish());

                        // Add right-side element if available, using SpaceBetween alignment
                        if let Some(right_element) = item.right_side_element(app) {
                            main_container = main_container
                                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween);
                            main_container.add_child(right_element);
                        }

                        let mut container = Container::new(main_container.finish())
                            .with_horizontal_padding(item_horizontal_padding)
                            .with_vertical_padding(item_vertical_padding);

                        if !matches!(chip_menu_type, ChipMenuType::Environments)
                            && (is_selected || index < filtered_items_length - 1)
                        {
                            container = container.with_border(Border::bottom(1.0));
                        }

                        if let Some(bg) = selected_background {
                            container = container.with_background(bg);
                        }

                        SavePosition::new(
                            EventHandler::new(container.finish())
                                .on_left_mouse_down(move |ctx, _, _| {
                                    ctx.dispatch_typed_action(DisplayChipMenuAction::SelectItem {
                                        index,
                                    });
                                    DispatchEventResult::StopPropagation
                                })
                                .on_right_mouse_down(move |ctx, _, pos| {
                                    // Show branch context menu on right-click
                                    ctx.dispatch_typed_action(
                                        DisplayChipMenuAction::ShowBranchContextMenu {
                                            index,
                                            position: pos,
                                        },
                                    );
                                    DispatchEventResult::StopPropagation
                                })
                                .on_mouse_in(
                                    move |ctx, _, _| {
                                        ctx.dispatch_typed_action(DisplayChipMenuAction::Select {
                                            index,
                                        });
                                        ctx.notify();
                                        DispatchEventResult::StopPropagation
                                    },
                                    Some(MouseInBehavior {
                                        fire_on_synthetic_events: false,
                                        fire_when_covered: false,
                                    }),
                                )
                                .finish(),
                            format!("MenuPromptChip-{index}").as_str(),
                        )
                        .finish()
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            },
        );

        let (scrollbar_width, max_height, overlayed_scrollbar) = match self.chip_menu_type {
            ChipMenuType::Environments => (
                ScrollbarWidth::Auto,
                ENV_MENU_MAX_HEIGHT - (ENV_MENU_VERTICAL_PADDING * 2.0),
                true,
            ),
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                // Use Auto scrollbar for branches to handle many branches
                let scrollbar = if self.chip_menu_type == ChipMenuType::Branches {
                    ScrollbarWidth::Auto
                } else {
                    ScrollbarWidth::None
                };
                // For branches, use larger height for bottom sheet
                let height = if self.chip_menu_type == ChipMenuType::Branches {
                    400.0 // Larger height for bottom sheet
                } else {
                    280.
                };
                (scrollbar, height, true)
            }
        };

        let mut scrollable = Scrollable::vertical(
            self.scroll_state.clone(),
            list.finish_scrollable(),
            scrollbar_width,
            theme.nonactive_ui_detail().into(),
            theme.active_ui_detail().into(),
            cuteui::elements::Fill::None,
        )
        .with_padding_end(0.)
        .with_padding_start(0.);

        if overlayed_scrollbar {
            scrollable = scrollable.with_overlayed_scrollbar();
        }

        // Return just the scrollable content area (no outer styling)
        // For branches in dual-pane mode, don't constrain width here
        if self.chip_menu_type == ChipMenuType::Branches {
            ConstrainedBox::new(scrollable.finish())
                .with_max_height(max_height)
                .finish()
        } else {
            ConstrainedBox::new(scrollable.finish())
                .with_width(menu_width)
                .with_max_height(max_height)
                .finish()
        }
    }

    fn render_dual_pane_layout(
        &self,
        left_pane: Box<dyn Element>,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        // Right pane - commit history for selected branch
        let middle_pane = self.render_commit_history_panel(app);
        let right_pane = self.render_file_changes_panel(app);

        // Vertical dividers

        let divider1 = ConstrainedBox::new(
            Container::new(Empty::new().finish())
                .with_background(Fill::Solid(internal_colors::neutral_2(theme)))
                .finish(),
        )
        .with_width(1.)
        .with_height(400.)
        .finish();

        let divider2 = ConstrainedBox::new(
            Container::new(Empty::new().finish())
                .with_background(Fill::Solid(internal_colors::neutral_2(theme)))
                .finish(),
        )
        .with_width(1.)
        .with_height(400.)
        .finish();

        // === THREE-COLUMN LAYOUT (matching IDEA screenshot) ===
        // Left: 25% (branches), Middle: 40% (commits, widest), Right: 35% (files)
        let left_with_width = ConstrainedBox::new(left_pane).with_width(280.).finish();

        let middle_with_width = ConstrainedBox::new(middle_pane).with_width(380.).finish();

        let right_with_width = ConstrainedBox::new(right_pane).with_width(320.).finish();

        // Three-column layout
        let content = Flex::row()
            .with_child(left_with_width)
            .with_child(divider1)
            .with_child(middle_with_width)
            .with_child(divider2)
            .with_child(right_with_width)
            .finish();

        // Bottom sheet style - half screen width, from bottom
        Container::new(content)
            .with_background(theme.surface_2())
            .with_corner_radius(CornerRadius::with_top(Radius::Pixels(12.)))
            .with_border(Border::top(1.0).with_border_color(internal_colors::neutral_3(theme)))
            .with_drop_shadow(DropShadow {
                blur_radius: 16.0,
                offset: pathfinder_geometry::vector::vec2f(0., -4.),
                color: ColorU::new(0, 0, 0, 80),
                spread_radius: 0.0,
            })
            .finish()
    }

    fn render_commit_history_panel(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let font_family = appearance.ui_font_family();
        let font_size = appearance.ui_font_size();
        let _mono_font_family = appearance.monospace_font_family();

        let mut container = Flex::column().with_main_axis_size(MainAxisSize::Max);

        // Show loading state for commits
        if self.is_loading_commits {
            container.add_child(
                Container::new(
                    Flex::column()
                        .with_main_axis_alignment(MainAxisAlignment::Center)
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Container::new(
                                Text::new_inline("⟳", font_family, font_size * 1.5)
                                    .with_color(theme.accent().into_solid())
                                    .finish(),
                            )
                            .with_margin_bottom(8.)
                            .finish(),
                        )
                        .with_child(
                            Text::new_inline("Loading commits...", font_family, font_size)
                                .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(20.)
                .with_vertical_padding(60.)
                .finish(),
            );
            return ConstrainedBox::new(container.finish())
                .with_width(COMMIT_PANEL_WIDTH)
                .with_max_height(MENU_MAX_HEIGHT)
                .finish();
        }

        // Show error if any
        if let Some(ref error) = self.load_error {
            container.add_child(
                Container::new(
                    Flex::column()
                        .with_main_axis_alignment(MainAxisAlignment::Center)
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Container::new(
                                Icon::Warning
                                    .to_cuteui_icon(Fill::Solid(theme.ansi_fg_yellow()))
                                    .finish(),
                            )
                            .with_margin_bottom(8.)
                            .finish(),
                        )
                        .with_child(
                            Text::new_inline(error.clone(), font_family, font_size)
                                .with_color(theme.ansi_fg_red())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(20.)
                .with_vertical_padding(60.)
                .finish(),
            );
            return ConstrainedBox::new(container.finish())
                .with_width(COMMIT_PANEL_WIDTH)
                .with_max_height(MENU_MAX_HEIGHT)
                .finish();
        }

        // Header with branch name
        if let Some(ref branch_name) = self.selected_branch_name {
            // Header section
            let mut header = Flex::column();

            // Top row: icon + branch name
            let mut header_top = Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

            // Determine if this is a remote branch
            let is_remote = branch_name.starts_with("origin/")
                || branch_name.starts_with("upstream/")
                || branch_name.starts_with("remote/");

            // Branch icon with colored background pill
            let branch_icon = if is_remote {
                Icon::Cloud
            } else {
                Icon::GitBranch
            };
            let icon_color = theme.ansi_fg_green();

            header_top.add_child(
                Container::new(
                    ConstrainedBox::new(
                        branch_icon.to_cuteui_icon(Fill::Solid(icon_color)).finish(),
                    )
                    .with_width(font_size)
                    .with_height(font_size)
                    .finish(),
                )
                .with_margin_right(8.)
                .finish(),
            );

            // Branch name with bold weight
            header_top.add_child(
                Text::new_inline(branch_name.clone(), font_family, font_size)
                    .with_color(theme.main_text_color(theme.surface_2()).into_solid())
                    .with_style(Properties::default().weight(Weight::Semibold))
                    .finish(),
            );

            header.add_child(
                Container::new(header_top.finish())
                    .with_horizontal_padding(16.)
                    .with_padding_top(12.)
                    .with_padding_bottom(8.)
                    .finish(),
            );

            // Stats row with subtle background
            let commit_count = self.selected_branch_commits.len();
            let stats_text = if commit_count == 0 {
                "No commits".to_string()
            } else if commit_count == 1 {
                "1 commit".to_string()
            } else {
                format!("{} commits", commit_count)
            };

            header.add_child(
                Container::new(
                    Flex::row()
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Container::new(
                                Icon::GitCommit
                                    .to_cuteui_icon(Fill::Solid(
                                        theme.sub_text_color(theme.surface_2()).into_solid(),
                                    ))
                                    .finish(),
                            )
                            .with_margin_right(6.)
                            .finish(),
                        )
                        .with_child(
                            Text::new_inline(stats_text, font_family, font_size - 1.)
                                .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(16.)
                .with_padding_top(4.)
                .with_padding_bottom(10.)
                .with_border(
                    Border::bottom(1.0).with_border_color(internal_colors::neutral_2(theme)),
                )
                .finish(),
            );

            container.add_child(header.finish());

            // Commit list
            if !self.selected_branch_commits.is_empty() {
                let commits = self.selected_branch_commits.clone();
                let selected_hash = self.selected_commit_hash.clone();
                let commit_list = UniformList::new(
                    self.commit_list_state.clone(),
                    commits.len(),
                    move |mut range, app| {
                        let appearance = Appearance::as_ref(app);
                        let theme = appearance.theme();
                        let font_family = appearance.ui_font_family();
                        let font_size = appearance.ui_font_size();
                        let mono_font_family = appearance.monospace_font_family();

                        range.end = std::cmp::min(range.end, commits.len());
                        range
                            .map(|index| {
                                let commit = &commits[index];
                                let is_selected = selected_hash.as_ref() == Some(&commit.hash);

                                let mut row = Flex::row()
                                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                                    .with_main_axis_size(MainAxisSize::Max);

                                // Commit hash (7 characters, monospace, cyan)
                                let hash_text = if commit.hash.len() >= 7 {
                                    &commit.hash[..7]
                                } else {
                                    &commit.hash
                                };
                                row.add_child(
                                    Container::new(
                                        Text::new_inline(
                                            hash_text.to_string(),
                                            mono_font_family,
                                            font_size - 1.,
                                        )
                                        .with_color(theme.ansi_fg_cyan())
                                        .finish(),
                                    )
                                    .with_margin_right(12.)
                                    .finish(),
                                );

                                // Commit message (truncate if too long)
                                let max_msg_len = 40;
                                let message = if commit.subject.len() > max_msg_len {
                                    format!("{}…", &commit.subject[..max_msg_len])
                                } else {
                                    commit.subject.clone()
                                };

                                // Message container (takes remaining space)
                                let mut msg_container =
                                    Flex::row().with_main_axis_size(MainAxisSize::Max);
                                msg_container.add_child(
                                    Text::new_inline(message, font_family, font_size - 1.)
                                        .with_color(
                                            theme.main_text_color(theme.surface_2()).into_solid(),
                                        )
                                        .finish(),
                                );
                                row.add_child(msg_container.finish());

                                // Stats: +additions -deletions with colors
                                if commit.additions > 0 || commit.deletions > 0 {
                                    row.add_child(
                                        Container::new(
                                            Flex::row()
                                                .with_cross_axis_alignment(
                                                    CrossAxisAlignment::Center,
                                                )
                                                .with_child(
                                                    Text::new_inline(
                                                        format!("+{}", commit.additions),
                                                        font_family,
                                                        font_size - 2.,
                                                    )
                                                    .with_color(theme.ansi_fg_green())
                                                    .finish(),
                                                )
                                                .with_child(
                                                    Container::new(Empty::new().finish())
                                                        .with_margin_left(4.)
                                                        .finish(),
                                                )
                                                .with_child(
                                                    Text::new_inline(
                                                        format!("-{}", commit.deletions),
                                                        font_family,
                                                        font_size - 2.,
                                                    )
                                                    .with_color(theme.ansi_fg_red())
                                                    .finish(),
                                                )
                                                .finish(),
                                        )
                                        .with_margin_left(12.)
                                        .finish(),
                                    );
                                }

                                let hash_for_click = commit.hash.clone();
                                let mut container = Container::new(row.finish())
                                    .with_horizontal_padding(16.)
                                    .with_vertical_padding(8.);

                                // Highlight selected commit
                                if is_selected {
                                    container = container.with_background(theme.accent());
                                }

                                EventHandler::new(container.finish())
                                    .on_left_mouse_down(move |ctx, _, _| {
                                        ctx.dispatch_typed_action(
                                            DisplayChipMenuAction::SelectCommit {
                                                commit_hash: hash_for_click.clone(),
                                            },
                                        );
                                        DispatchEventResult::StopPropagation
                                    })
                                    .finish()
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    },
                );

                let scrollable = Scrollable::vertical(
                    self.commit_scroll_state.clone(),
                    commit_list.finish_scrollable(),
                    ScrollbarWidth::Auto,
                    theme.nonactive_ui_detail().into(),
                    theme.active_ui_detail().into(),
                    cuteui::elements::Fill::None,
                )
                .with_overlayed_scrollbar();

                container.add_child(
                    ConstrainedBox::new(scrollable.finish())
                        .with_max_height(320.)
                        .finish(),
                );
            } else {
                // Empty state
                container.add_child(
                    Container::new(
                        Flex::column()
                            .with_main_axis_alignment(MainAxisAlignment::Center)
                            .with_cross_axis_alignment(CrossAxisAlignment::Center)
                            .with_child(
                                Container::new(
                                    Icon::GitCommit
                                        .to_cuteui_icon(Fill::Solid(
                                            theme.sub_text_color(theme.surface_2()).into_solid(),
                                        ))
                                        .finish(),
                                )
                                .with_margin_bottom(8.)
                                .finish(),
                            )
                            .with_child(
                                Text::new_inline("No commits found", font_family, font_size)
                                    .with_color(
                                        theme.sub_text_color(theme.surface_2()).into_solid(),
                                    )
                                    .finish(),
                            )
                            .finish(),
                    )
                    .with_horizontal_padding(16.)
                    .with_vertical_padding(40.)
                    .finish(),
                );
            }
        } else {
            // No branch selected - show helpful placeholder
            container.add_child(
                Container::new(
                    Flex::column()
                        .with_main_axis_alignment(MainAxisAlignment::Center)
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Container::new(
                                Icon::GitBranch
                                    .to_cuteui_icon(Fill::Solid(
                                        theme.sub_text_color(theme.surface_2()).into_solid(),
                                    ))
                                    .finish(),
                            )
                            .with_margin_bottom(12.)
                            .finish(),
                        )
                        .with_child(
                            Text::new_inline("Select a branch", font_family, font_size)
                                .with_color(theme.main_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .with_child(
                            Text::new_inline("to view commit history", font_family, font_size - 1.)
                                .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(20.)
                .with_vertical_padding(60.)
                .finish(),
            );
        }

        ConstrainedBox::new(container.finish())
            .with_width(COMMIT_PANEL_WIDTH)
            .with_max_height(MENU_MAX_HEIGHT)
            .finish()
    }

    /// Render the file changes panel (third pane) for the selected commit
    fn render_file_changes_panel(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let font_family = appearance.ui_font_family();
        let font_size = appearance.ui_font_size();

        let mut container = Flex::column().with_main_axis_size(MainAxisSize::Max);

        // Show loading state
        if self.is_loading_files {
            container.add_child(
                Container::new(
                    Flex::column()
                        .with_main_axis_alignment(MainAxisAlignment::Center)
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Text::new_inline("Loading files...", font_family, font_size)
                                .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(20.)
                .with_vertical_padding(60.)
                .finish(),
            );
            return ConstrainedBox::new(container.finish())
                .with_max_height(MENU_MAX_HEIGHT)
                .finish();
        }

        // Show files if we have a selected commit
        if let Some(ref commit_hash) = self.selected_commit_hash {
            // Header with commit hash
            let hash_text = if commit_hash.len() >= 7 {
                &commit_hash[..7]
            } else {
                commit_hash
            };

            container.add_child(
                Container::new(
                    Flex::row()
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Icon::File
                                .to_cuteui_icon(Fill::Solid(
                                    theme.sub_text_color(theme.surface_2()).into_solid(),
                                ))
                                .finish(),
                        )
                        .with_child(
                            Container::new(
                                Text::new_inline(
                                    format!("  Files in {}", hash_text),
                                    font_family,
                                    font_size,
                                )
                                .with_color(theme.main_text_color(theme.surface_2()).into_solid())
                                .finish(),
                            )
                            .with_margin_left(4.)
                            .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(16.)
                .with_padding_top(12.)
                .with_padding_bottom(8.)
                .with_border(
                    Border::bottom(1.0).with_border_color(internal_colors::neutral_2(theme)),
                )
                .finish(),
            );

            // File list
            if !self.selected_commit_files.is_empty() {
                let files = self.selected_commit_files.clone();
                let file_list = UniformList::new(
                    self.file_list_state.clone(),
                    files.len(),
                    move |mut range, app| {
                        let appearance = Appearance::as_ref(app);
                        let theme = appearance.theme();
                        let font_family = appearance.ui_font_family();
                        let font_size = appearance.ui_font_size();

                        range.end = std::cmp::min(range.end, files.len());
                        range
                            .map(|index| {
                                let file = &files[index];

                                let mut row = Flex::row()
                                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                                    .with_main_axis_size(MainAxisSize::Max);

                                // File path
                                row.add_child(
                                    Text::new_inline(
                                        file.path.clone(),
                                        font_family,
                                        font_size - 1.,
                                    )
                                    .with_color(
                                        theme.main_text_color(theme.surface_2()).into_solid(),
                                    )
                                    .finish(),
                                );

                                // Stats
                                if file.additions > 0 || file.deletions > 0 {
                                    row.add_child(
                                        Container::new(
                                            Flex::row()
                                                .with_cross_axis_alignment(
                                                    CrossAxisAlignment::Center,
                                                )
                                                .with_child(
                                                    Text::new_inline(
                                                        format!("+{}", file.additions),
                                                        font_family,
                                                        font_size - 2.,
                                                    )
                                                    .with_color(theme.ansi_fg_green())
                                                    .finish(),
                                                )
                                                .with_child(
                                                    Container::new(Empty::new().finish())
                                                        .with_margin_left(4.)
                                                        .finish(),
                                                )
                                                .with_child(
                                                    Text::new_inline(
                                                        format!("-{}", file.deletions),
                                                        font_family,
                                                        font_size - 2.,
                                                    )
                                                    .with_color(theme.ansi_fg_red())
                                                    .finish(),
                                                )
                                                .finish(),
                                        )
                                        .with_margin_left(12.)
                                        .finish(),
                                    );
                                }

                                Container::new(row.finish())
                                    .with_horizontal_padding(16.)
                                    .with_vertical_padding(6.)
                                    .finish()
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                    },
                );

                let scrollable = Scrollable::vertical(
                    self.file_scroll_state.clone(),
                    file_list.finish_scrollable(),
                    ScrollbarWidth::Auto,
                    theme.nonactive_ui_detail().into(),
                    theme.active_ui_detail().into(),
                    cuteui::elements::Fill::None,
                )
                .with_overlayed_scrollbar();

                container.add_child(
                    ConstrainedBox::new(scrollable.finish())
                        .with_max_height(280.)
                        .finish(),
                );
            } else {
                container.add_child(
                    Container::new(
                        Text::new_inline("No file changes", font_family, font_size)
                            .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                            .finish(),
                    )
                    .with_horizontal_padding(16.)
                    .with_vertical_padding(40.)
                    .finish(),
                );
            }
        } else {
            // No commit selected
            container.add_child(
                Container::new(
                    Flex::column()
                        .with_main_axis_alignment(MainAxisAlignment::Center)
                        .with_cross_axis_alignment(CrossAxisAlignment::Center)
                        .with_child(
                            Container::new(
                                Icon::File
                                    .to_cuteui_icon(Fill::Solid(
                                        theme.sub_text_color(theme.surface_2()).into_solid(),
                                    ))
                                    .finish(),
                            )
                            .with_margin_bottom(12.)
                            .finish(),
                        )
                        .with_child(
                            Text::new_inline("Select a commit", font_family, font_size)
                                .with_color(theme.main_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .with_child(
                            Text::new_inline("to view file changes", font_family, font_size - 1.)
                                .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                                .finish(),
                        )
                        .finish(),
                )
                .with_horizontal_padding(20.)
                .with_vertical_padding(60.)
                .finish(),
            );
        }

        ConstrainedBox::new(container.finish())
            .with_max_height(MENU_MAX_HEIGHT)
            .finish()
    }

    /// Show the branch context menu for the item at the given index
    fn show_branch_context_menu(
        &mut self,
        index: usize,
        position: cuteui::geometry::vector::Vector2F,
        ctx: &mut ViewContext<Self>,
    ) {
        if index >= self.filtered_items.len() {
            return;
        }

        let item = &self.filtered_items[index].item;
        let branch_name = item.name();

        // Determine if this is a remote branch by checking common remote prefixes
        // Local branches can contain "/" (e.g., "feature/branch"), so we only
        // check for known remote prefixes like "origin/", "upstream/", etc.
        let is_remote = branch_name.starts_with("origin/")
            || branch_name.starts_with("upstream/")
            || branch_name.starts_with("remote/");

        // Check if this is the current branch
        let is_current_branch = self
            .current_branch_name
            .as_ref()
            .map(|current| &branch_name == current)
            .unwrap_or(false);

        self.branch_context_menu = Some(BranchContextMenu {
            branch_name: branch_name.clone(),
            is_remote,
            is_current_branch,
        });
        self.branch_context_menu_position = Some(position);

        // Pre-allocate mouse states for context menu items
        // Maximum items: Checkout, Merge, Rename, Delete, Copy = 5
        self.branch_context_menu_mouse_states =
            (0..5).map(|_| MouseStateHandle::default()).collect();

        ctx.notify();
    }

    /// Execute a branch context menu action
    fn execute_branch_action(
        &mut self,
        action: &BranchContextMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        // Close the context menu first
        self.branch_context_menu = None;
        self.branch_context_menu_mouse_states.clear();

        match action {
            BranchContextMenuAction::MergeIntoCurrent { branch_name } => {
                ctx.emit(PromptDisplayMenuEvent::BranchAction {
                    action: BranchAction::MergeIntoCurrent {
                        branch_name: branch_name.clone(),
                    },
                });
            }
            BranchContextMenuAction::Checkout { branch_name } => {
                ctx.emit(PromptDisplayMenuEvent::BranchAction {
                    action: BranchAction::Checkout {
                        branch_name: branch_name.clone(),
                    },
                });
            }
            BranchContextMenuAction::Delete { branch_name } => {
                ctx.emit(PromptDisplayMenuEvent::BranchAction {
                    action: BranchAction::Delete {
                        branch_name: branch_name.clone(),
                    },
                });
            }
            BranchContextMenuAction::Rename { branch_name } => {
                ctx.emit(PromptDisplayMenuEvent::BranchAction {
                    action: BranchAction::Rename {
                        branch_name: branch_name.clone(),
                    },
                });
            }
            BranchContextMenuAction::CopyBranchName { branch_name } => {
                ctx.clipboard()
                    .write(cuteui::clipboard::ClipboardContent::plain_text(
                        branch_name.clone(),
                    ));
            }
        }

        ctx.notify();
    }

    /// Render the branch context menu popup
    fn render_branch_context_menu(&self, app: &AppContext) -> Option<Box<dyn Element>> {
        let context_menu = self.branch_context_menu.as_ref()?;
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();
        let font_family = appearance.ui_font_family();
        let font_size = appearance.ui_font_size();

        let mut items: Vec<(BranchContextMenuAction, String, Option<Icon>)> = vec![];

        // Add menu items based on branch type
        if !context_menu.is_remote {
            // Local branch options
            if !context_menu.is_current_branch {
                items.push((
                    BranchContextMenuAction::Checkout {
                        branch_name: context_menu.branch_name.clone(),
                    },
                    "Checkout".to_string(),
                    Some(Icon::GitBranch),
                ));
                items.push((
                    BranchContextMenuAction::MergeIntoCurrent {
                        branch_name: context_menu.branch_name.clone(),
                    },
                    "Merge into Current".to_string(),
                    Some(Icon::ArrowSplit),
                ));
            }
            items.push((
                BranchContextMenuAction::Rename {
                    branch_name: context_menu.branch_name.clone(),
                },
                "Rename".to_string(),
                Some(Icon::Edit),
            ));
            if !context_menu.is_current_branch {
                items.push((
                    BranchContextMenuAction::Delete {
                        branch_name: context_menu.branch_name.clone(),
                    },
                    "Delete".to_string(),
                    Some(Icon::Trash),
                ));
            }
        } else {
            // Remote branch options - can checkout to create local tracking branch
            items.push((
                BranchContextMenuAction::Checkout {
                    branch_name: context_menu.branch_name.clone(),
                },
                "Checkout".to_string(),
                Some(Icon::GitBranch),
            ));
        }
        items.push((
            BranchContextMenuAction::CopyBranchName {
                branch_name: context_menu.branch_name.clone(),
            },
            "Copy Branch Name".to_string(),
            Some(Icon::Copy),
        ));

        let menu_width = 180.0;
        let icon_size = font_size * 0.8;

        let mut column = Flex::column();

        for (i, (action, label, icon)) in items.iter().enumerate() {
            let action_clone = action.clone();
            let label_clone = label.clone();
            let icon_clone = *icon;
            let is_last = i == items.len() - 1;

            // Use pre-allocated mouse state if available, otherwise create a new one
            let mouse_state = self
                .branch_context_menu_mouse_states
                .get(i)
                .cloned()
                .unwrap_or_else(|| MouseStateHandle::default());

            let item = Hoverable::new(mouse_state, move |mouse_state| {
                let is_hovered = mouse_state.is_hovered();
                let text_color = if is_hovered {
                    theme.main_text_color(theme.accent()).into_solid()
                } else {
                    theme.main_text_color(theme.surface_2()).into_solid()
                };
                let icon_color = if is_hovered {
                    theme.main_text_color(theme.accent()).into_solid()
                } else {
                    theme.main_text_color(theme.surface_2()).into_solid()
                };

                let mut row = Flex::row()
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_main_axis_size(MainAxisSize::Max);

                if let Some(icon) = icon_clone {
                    row.add_child(
                        Container::new(
                            ConstrainedBox::new(
                                icon.to_cuteui_icon(Fill::Solid(icon_color)).finish(),
                            )
                            .with_width(icon_size)
                            .with_height(icon_size)
                            .finish(),
                        )
                        .with_margin_right(8.)
                        .finish(),
                    );
                } else {
                    row.add_child(
                        ConstrainedBox::new(Empty::new().finish())
                            .with_width(icon_size + 8.)
                            .finish(),
                    );
                }

                row.add_child(
                    Text::new_inline(label_clone, font_family, font_size)
                        .with_color(text_color)
                        .finish(),
                );

                let mut container = Container::new(row.finish())
                    .with_horizontal_padding(12.)
                    .with_vertical_padding(6.);

                if !is_last {
                    container = container.with_border(
                        Border::bottom(1.0).with_border_color(internal_colors::neutral_2(theme)),
                    );
                }

                if is_hovered {
                    container = container.with_background(theme.accent());
                }
                container.finish()
            })
            .on_click(move |ctx, _, _| {
                ctx.dispatch_typed_action(DisplayChipMenuAction::ExecuteBranchAction {
                    action: action_clone.clone(),
                });
            })
            .finish();

            column.add_child(item);
        }

        let menu = Container::new(column.finish())
            .with_background(theme.surface_2())
            .with_border(Border::all(1.0).with_border_color(internal_colors::neutral_4(theme)))
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.0)))
            .with_drop_shadow(DropShadow::default())
            .finish();

        Some(ConstrainedBox::new(menu).with_width(menu_width).finish())
    }
}

impl View for DisplayChipMenu {
    fn ui_name() -> &'static str {
        "DisplayMenu"
    }

    fn on_focus(&mut self, focus_ctx: &FocusContext, ctx: &mut ViewContext<Self>) {
        if focus_ctx.is_self_focused() {
            if let Some(ref search_input) = self.search_input {
                ctx.focus(search_input);
            }
        }
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        // Create vertical flex container for main content + search input + sticky fixed footer option
        let mut main_container = Flex::column();

        let border_radius = Radius::Pixels(6.);

        match self.chip_menu_type {
            ChipMenuType::Environments => {
                if !self.menu_items.is_empty() {
                    main_container.add_child(
                        Container::new(self.render_items(app))
                            .with_padding_top(self.menu_vertical_padding())
                            .with_padding_bottom(self.menu_vertical_padding())
                            .finish(),
                    );
                }
                if let Some(ref footer_option) = self.fixed_footer {
                    main_container.add_child(self.render_fixed_footer_option(app, footer_option));
                }
                if let Some(ref search_input_handle) = self.search_input {
                    main_container
                        .add_child(self.render_env_search_footer(search_input_handle, app));
                }
            }
            ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                if let Some(ref search_input_handle) = self.search_input {
                    let search_input = appearance
                        .ui_builder()
                        .text_input(search_input_handle.clone())
                        .with_style(UiComponentStyles {
                            background: Some(Fill::Solid(ColorU::new(0, 0, 0, 0)).into()),
                            border_color: None,
                            border_width: Some(0.),
                            border_radius: None,
                            width: Some(self.menu_width() - (SEARCH_INPUT_HORIZONTAL_PADDING * 2.)),
                            padding: Some(Coords::uniform(4.)),
                            ..Default::default()
                        })
                        .build()
                        .finish();

                    let search_input_container = Container::new(search_input)
                        .with_horizontal_padding(SEARCH_INPUT_HORIZONTAL_PADDING)
                        .with_vertical_padding(2.)
                        .with_background(theme.surface_1())
                        .with_border(Border::all(1.0).with_border_color(theme.surface_2().into()))
                        .with_corner_radius(CornerRadius::with_top(border_radius))
                        .finish();

                    main_container.add_child(search_input_container);
                }
                if let Some(ref footer_option) = self.fixed_footer {
                    main_container.add_child(self.render_fixed_footer_option(app, footer_option));
                }
                if !self.menu_items.is_empty() {
                    main_container.add_child(
                        Container::new(self.render_items(app))
                            .with_padding_bottom(self.menu_vertical_padding())
                            .finish(),
                    );
                }
            }
        }

        let menu_card = {
            let menu_container = Container::new(main_container.finish())
                .with_background(theme.surface_2())
                .with_corner_radius(CornerRadius::with_all(border_radius));

            let menu_container = match self.chip_menu_type {
                ChipMenuType::Environments => menu_container
                    .with_border(
                        Border::all(1.)
                            .with_border_fill(Fill::Solid(internal_colors::neutral_4(theme))),
                    )
                    .with_drop_shadow(Self::figma_menu_drop_shadow()),
                ChipMenuType::Directories | ChipMenuType::Branches | ChipMenuType::CodeReview => {
                    menu_container.with_drop_shadow(DropShadow::default())
                }
            };

            // For branches, use wider width for bottom sheet
            let width = if self.chip_menu_type == ChipMenuType::Branches {
                900.0 // Wider for bottom sheet style
            } else {
                self.menu_width()
            };

            // For branches, add max height constraint
            if self.chip_menu_type == ChipMenuType::Branches {
                ConstrainedBox::new(menu_container.finish())
                    .with_width(width)
                    .with_max_height(450.0) // Half screen height
                    .finish()
            } else {
                ConstrainedBox::new(menu_container.finish())
                    .with_width(width)
                    .finish()
            }
        };

        // For branches, render dual-pane layout as bottom sheet
        let final_element = if self.chip_menu_type == ChipMenuType::Branches {
            // Get window width from bounds
            let window_width = app
                .window_bounds(&self.window_id)
                .map(|bounds| bounds.size().x())
                .unwrap_or(900.0);

            // Calculate widths - left 40%, right 60%
            let left_width = (window_width * 0.40).min(400.0).max(280.0);
            let _right_width = window_width - left_width;

            // === HEADER with Close Button ===
            let close_button = Hoverable::new(MouseStateHandle::default(), move |state| {
                let hovered = state.is_hovered();
                let icon_color = if hovered {
                    theme.main_text_color(theme.surface_2()).into_solid()
                } else {
                    theme.sub_text_color(theme.surface_2()).into_solid()
                };
                Icon::X.to_cuteui_icon(Fill::Solid(icon_color)).finish()
            });
            let close_button = close_button
                .on_click(|ctx, _, _| {
                    ctx.dispatch_typed_action(DisplayChipMenuAction::Close);
                })
                .with_cursor(Cursor::PointingHand)
                .finish();

            let header = Container::new(
                Flex::row()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_main_axis_alignment(MainAxisAlignment::End)
                    .with_child(
                        Container::new(close_button)
                            .with_padding_top(8.)
                            .with_padding_right(12.)
                            .finish(),
                    )
                    .finish(),
            )
            .finish();

            // === LEFT PANE: Search + Branch List ===
            let mut left_column = Flex::column().with_main_axis_size(MainAxisSize::Max);

            // Search input
            if let Some(ref search_input_handle) = self.search_input {
                let search_input = appearance
                    .ui_builder()
                    .text_input(search_input_handle.clone())
                    .with_style(UiComponentStyles {
                        background: Some(Fill::Solid(ColorU::new(0, 0, 0, 0)).into()),
                        border_color: None,
                        border_width: Some(0.),
                        border_radius: None,
                        width: Some(left_width - 32.),
                        padding: Some(Coords::uniform(6.)),
                        ..Default::default()
                    })
                    .build()
                    .finish();

                left_column.add_child(
                    Container::new(search_input)
                        .with_horizontal_padding(12.)
                        .with_vertical_padding(8.)
                        .with_background(theme.surface_1())
                        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.)))
                        .finish(),
                );
            }

            // Branch list - directly render items
            if !self.filtered_items.is_empty() {
                left_column.add_child(
                    Container::new(self.render_items(app))
                        .with_padding_top(4.)
                        .finish(),
                );
            } else if !self.search_query.is_empty() {
                // No results
                left_column.add_child(
                    Container::new(
                        Text::new(
                            "No branches found",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                        .finish(),
                    )
                    .with_horizontal_padding(16.)
                    .with_vertical_padding(20.)
                    .finish(),
                );
            } else {
                // No branches at all - show message
                left_column.add_child(
                    Container::new(
                        Text::new(
                            "No branches available",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_color(theme.sub_text_color(theme.surface_2()).into_solid())
                        .finish(),
                    )
                    .with_horizontal_padding(16.)
                    .with_vertical_padding(20.)
                    .finish(),
                );
            }

            let left_pane = Container::new(left_column.finish())
                .with_background(theme.surface_2())
                .with_padding_top(8.)
                .with_padding_bottom(12.)
                .with_horizontal_padding(8.)
                .finish();

            // === MIDDLE PANE: Commit History ===
            let middle_pane = self.render_commit_history_panel(app);

            // === RIGHT PANE: File Changes ===
            let right_pane = self.render_file_changes_panel(app);

            // === DIVIDERS ===
            let divider1 = ConstrainedBox::new(
                Container::new(Empty::new().finish())
                    .with_background(Fill::Solid(internal_colors::neutral_2(theme)))
                    .finish(),
            )
            .with_width(1.)
            .finish();

            let divider2 = ConstrainedBox::new(
                Container::new(Empty::new().finish())
                    .with_background(Fill::Solid(internal_colors::neutral_2(theme)))
                    .finish(),
            )
            .with_width(1.)
            .finish();

            // === THREE-COLUMN LAYOUT ===
            // Left: 20%, Middle: 50% (widest), Right: 30%
            let left_pane_width = (window_width * 0.20).min(280.0).max(200.0);
            let middle_pane_width = (window_width * 0.50).min(500.0).max(350.0);
            let right_pane_width = window_width - left_pane_width - middle_pane_width;

            let left_constrained = ConstrainedBox::new(left_pane)
                .with_width(left_pane_width)
                .with_max_height(380.) // Leave space for bottom bar
                .finish();

            let middle_constrained = ConstrainedBox::new(middle_pane)
                .with_width(middle_pane_width)
                .with_max_height(380.)
                .finish();

            let right_constrained = ConstrainedBox::new(right_pane)
                .with_width(right_pane_width)
                .with_max_height(380.)
                .finish();

            let row = Flex::row()
                .with_child(left_constrained)
                .with_child(divider1)
                .with_child(middle_constrained)
                .with_child(divider2)
                .with_child(right_constrained)
                .finish();

            // Combine header + content
            let content = Flex::column().with_child(header).with_child(row).finish();

            // Bottom sheet container
            let dual_pane = Container::new(content)
                .with_background(theme.surface_2())
                .with_corner_radius(CornerRadius::with_top(Radius::Pixels(12.)))
                .with_border(Border::top(1.0).with_border_color(internal_colors::neutral_3(theme)))
                .with_drop_shadow(DropShadow {
                    blur_radius: 20.0,
                    offset: pathfinder_geometry::vector::vec2f(0., -6.),
                    color: ColorU::new(0, 0, 0, 100),
                    spread_radius: 0.0,
                })
                .finish();

            // Add context menu overlay if open
            if let Some(context_menu) = self.render_branch_context_menu(app) {
                let mut stack = Stack::new();
                stack.add_child(dual_pane);

                let position = self
                    .branch_context_menu_position
                    .unwrap_or(vec2f(100., 50.));
                let positioning = OffsetPositioning::offset_from_parent(
                    position,
                    ParentOffsetBounds::WindowByPosition,
                    ParentAnchor::TopLeft,
                    ChildAnchor::TopLeft,
                );
                stack.add_positioned_overlay_child(context_menu, positioning);

                stack.finish()
            } else {
                dual_pane
            }
        } else {
            let mut stack = Stack::new();
            stack.add_child(menu_card);

            if self.should_show_environment_sidecar() {
                if let Some((sidecar, positioning)) = self.environment_sidecar_overlay(app) {
                    stack.add_positioned_overlay_child(sidecar, positioning);
                }
            }

            stack.finish()
        };

        Dismiss::new(final_element)
            .on_dismiss(|ctx, _app| ctx.dispatch_typed_action(DisplayChipMenuAction::Close))
            .prevent_interaction_with_other_elements()
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct GenericMenuEvent {
    pub action_item: Arc<dyn GenericMenuItem>,
}

/// Branch actions that can be performed from the context menu
#[derive(Debug, Clone)]
pub enum BranchAction {
    MergeIntoCurrent { branch_name: String },
    Checkout { branch_name: String },
    Delete { branch_name: String },
    Rename { branch_name: String },
}

pub enum PromptDisplayMenuEvent {
    MenuAction(GenericMenuEvent),
    CloseMenu,
    BranchSelected { branch_name: String },
    BranchAction { action: BranchAction },
    CommitSelected { commit_hash: String },
}

impl Entity for DisplayChipMenu {
    type Event = PromptDisplayMenuEvent;
}

impl TypedActionView for DisplayChipMenu {
    type Action = DisplayChipMenuAction;

    fn handle_action(&mut self, action: &DisplayChipMenuAction, ctx: &mut ViewContext<Self>) {
        match action {
            DisplayChipMenuAction::SelectItem { index } => {
                if *index >= self.filtered_items.len() {
                    return;
                }
                let item = self.filtered_items[*index].item.clone();
                self.select_item(item, ctx)
            }
            DisplayChipMenuAction::Select { index } => self.select(*index, ctx),
            DisplayChipMenuAction::SelectUp => self.select_prev(ctx),
            DisplayChipMenuAction::SelectDown => self.select_next(ctx),
            DisplayChipMenuAction::SelectEnter => self.select_enter(ctx),
            DisplayChipMenuAction::SelectFixedFooterOption => self.select_fixed_footer_option(ctx),
            DisplayChipMenuAction::CopyEnvironmentSidecarField { key, value } => {
                ctx.clipboard()
                    .write(ClipboardContent::plain_text(value.clone()));

                self.env_sidecar_copy_feedback_times
                    .insert(key.clone(), Instant::now());

                let duration = COPY_FEEDBACK_DURATION;
                ctx.spawn(
                    async move {
                        Timer::after(duration).await;
                    },
                    move |me, _, ctx| {
                        // Clean up old entries.
                        me.env_sidecar_copy_feedback_times
                            .retain(|_, time| time.elapsed() < duration);
                        ctx.notify();
                    },
                );

                ctx.notify();
            }
            DisplayChipMenuAction::Close => self.close(ctx),
            DisplayChipMenuAction::ShowBranchContextMenu { index, position } => {
                self.show_branch_context_menu(*index, *position, ctx);
            }
            DisplayChipMenuAction::ExecuteBranchAction { action } => {
                self.execute_branch_action(action, ctx);
            }
            DisplayChipMenuAction::CloseBranchContextMenu => {
                self.branch_context_menu = None;
                self.branch_context_menu_mouse_states.clear();
                ctx.notify();
            }
            DisplayChipMenuAction::SelectCommit { commit_hash } => {
                // Emit event to fetch commit files
                ctx.emit(PromptDisplayMenuEvent::CommitSelected {
                    commit_hash: commit_hash.clone(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::query_matches_existing_name;

    #[test]
    fn query_matches_existing_name_is_ascii_case_insensitive() {
        let names = ["main", "feature/Foo"];
        assert!(query_matches_existing_name(names, "main"));
        assert!(query_matches_existing_name(names, "Main"));
        assert!(query_matches_existing_name(names, "MAIN"));
        assert!(query_matches_existing_name(names, "feature/foo"));
        assert!(query_matches_existing_name(names, "FEATURE/FOO"));
    }

    #[test]
    fn query_matches_existing_name_returns_false_when_no_overlap() {
        let names = ["main", "feature/foo"];
        assert!(!query_matches_existing_name(names, "develop"));
        assert!(!query_matches_existing_name(names, "feature/bar"));
    }

    #[test]
    fn query_matches_existing_name_returns_false_for_empty_input() {
        let names: [&str; 0] = [];
        assert!(!query_matches_existing_name(names, "main"));
    }

    #[test]
    fn query_matches_existing_name_works_with_owned_strings() {
        let names = [String::from("main"), String::from("Develop")];
        assert!(query_matches_existing_name(names.iter(), "Main"));
        assert!(query_matches_existing_name(names.iter(), "develop"));
    }
}

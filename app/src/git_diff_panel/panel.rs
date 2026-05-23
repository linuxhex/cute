use std::path::PathBuf;

use warpui::elements::{
    ChildView, Container, CrossAxisAlignment, Element, Flex, MainAxisAlignment, MainAxisSize,
    ParentElement, Shrinkable, Text,
};
use warpui::fonts::Weight;
use warpui::presenter::ChildView;
use warpui::{ModelHandle, View, ViewContext, ViewHandle};

use super::model::{GitDiffModel, GitDiffModelEvent, GitFileStatus};
use crate::appearance::Appearance;

pub const MIN_PANEL_WIDTH: f32 = 200.;
pub const MAX_PANEL_WIDTH_RATIO: f32 = 0.3;

pub struct GitDiffPanel {
    model: ModelHandle<GitDiffModel>,
    list_view: ViewHandle<GitFileListView>,
}

#[derive(Clone, Debug)]
pub enum GitDiffPanelAction {
    OpenFile(PathBuf),
    Refresh,
}

#[derive(Clone, Debug)]
pub enum GitDiffPanelEvent {
    OpenFile(PathBuf),
}

impl GitDiffPanel {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let model = ctx.add_model(|_| GitDiffModel::new());
        let list_view = ctx.add_typed_action_view(|ctx| GitFileListView::new(model.clone(), ctx));

        ctx.subscribe_to_model(&model, |_, _, event, _| {
            Some(GitDiffPanelAction::Refresh)
        });

        Self { model, list_view }
    }

    pub fn refresh(&mut self, working_dir: &PathBuf, ctx: &mut ViewContext<Self>) {
        self.model.update(ctx, |model, _| {
            model.refresh(working_dir);
        });
    }

    fn render_header(&self, ctx: &mut ViewContext<Self>) -> Element {
        let appearance = Appearance::as_ref(ctx);
        let file_count = self.model.read(ctx, |m, _| m.file_count());

        Flex::row()
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_children([
                Text::new("Git Changes")
                    .with_font_size(14.)
                    .with_font_weight(Weight::SEMIBOLD)
                    .with_text_color(appearance.colors().text)
                    .into(),
                Text::new(format!("({})", file_count))
                    .with_font_size(12.)
                    .with_text_color(appearance.colors().text_muted)
                    .into(),
            ])
            .into()
    }
}

impl View for GitDiffPanel {
    fn view(&mut self, ctx: &mut ViewContext<Self>) -> Element {
        let appearance = Appearance::as_ref(ctx);

        Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_size(MainAxisSize::Max)
            .with_children([
                Container::new(self.render_header(ctx))
                    .with_padding(warpui::elements::Padding::all(8.))
                    .into(),
                ChildView::new(&self.list_view).into(),
            ])
            .with_background_color(appearance.colors().surface)
            .into()
    }
}

pub struct GitFileListView {
    model: ModelHandle<GitDiffModel>,
}

impl GitFileListView {
    pub fn new(model: ModelHandle<GitDiffModel>, ctx: &mut ViewContext<Self>) -> Self {
        Self { model }
    }

    fn render_file_item(
        &self,
        path: &PathBuf,
        status: &GitFileStatus,
        ctx: &mut ViewContext<Self>,
    ) -> Element {
        let appearance = Appearance::as_ref(ctx);
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?");

        let status_color = match status {
            GitFileStatus::Added => appearance.colors().success,
            GitFileStatus::Modified => appearance.colors().warning,
            GitFileStatus::Deleted => appearance.colors().error,
            GitFileStatus::Renamed => appearance.colors().info,
            GitFileStatus::Untracked => appearance.colors().text_muted,
        };

        let status_text = match status {
            GitFileStatus::Added => "A",
            GitFileStatus::Modified => "M",
            GitFileStatus::Deleted => "D",
            GitFileStatus::Renamed => "R",
            GitFileStatus::Untracked => "?",
        };

        Flex::row()
            .with_main_axis_alignment(MainAxisAlignment::Start)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_gap(8.)
            .with_children([
                Text::new(status_text)
                    .with_font_size(12.)
                    .with_font_weight(Weight::BOLD)
                    .with_text_color(status_color)
                    .into(),
                Text::new(file_name)
                    .with_font_size(13.)
                    .with_text_color(appearance.colors().text)
                    .into(),
            ])
            .into()
    }
}

impl View for GitFileListView {
    fn view(&mut self, ctx: &mut ViewContext<Self>) -> Element {
        let appearance = Appearance::as_ref(ctx);

        let files = self.model.read(ctx, |m, _| m.files().to_vec());

        if files.is_empty() {
            return Flex::column()
                .with_main_axis_size(MainAxisSize::Max)
                .with_cross_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::Center)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(
                    Text::new("No changes")
                        .with_font_size(13.)
                        .with_text_color(appearance.colors().text_muted)
                        .into(),
                )
                .into();
        }

        let items: Vec<Element> = files
            .iter()
            .map(|file| self.render_file_item(&file.path, &file.status, ctx))
            .collect();

        Flex::column()
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_size(MainAxisSize::Max)
            .with_gap(4.)
            .with_children(items)
            .with_padding(warpui::elements::Padding::horizontal(8.))
            .into()
    }
}

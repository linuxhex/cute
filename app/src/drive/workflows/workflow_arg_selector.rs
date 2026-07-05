// Stub module for workflow argument selector

use std::collections::HashMap;

use cuteui::{AppContext, Element, Entity, View, ViewContext, ViewHandle};
use cuteui::ui_components::components::Coords;

use crate::drive::workflows::enum_creation_dialog::WorkflowEnumData;
use crate::editor::EditorView;
use crate::server::ids::SyncId;

/// Events emitted by the workflow argument selector
#[derive(Debug, Clone)]
pub enum WorkflowArgSelectorEvent {
    NewEnum,
    LoadEnum(usize),
    Edited,
    Close,
    ToggleExpanded,
    InputTab,
    InputShiftTab,
}

/// Styles for the workflow argument selector
pub struct WorkflowArgSelectorStyles {
    pub editor_padding: Coords,
    pub height: Option<f32>,
    pub width: Option<f32>,
    pub dropdown_background: fn(&crate::appearance::Appearance) -> pathfinder_color::ColorU,
    pub border_color: fn(&crate::appearance::Appearance) -> pathfinder_color::ColorU,
    pub border_radius: f32,
}

impl Default for WorkflowArgSelectorStyles {
    fn default() -> Self {
        Self {
            editor_padding: Coords::uniform(10.0),
            height: None,
            width: None,
            dropdown_background: |_| pathfinder_color::ColorU::transparent_black(),
            border_color: |_| pathfinder_color::ColorU::transparent_black(),
            border_radius: 4.0,
        }
    }
}

pub struct WorkflowArgSelector {
    pub text_editor: Option<ViewHandle<EditorView>>,
}

impl Entity for WorkflowArgSelector {
    type Event = WorkflowArgSelectorEvent;
}

impl View for WorkflowArgSelector {
    fn ui_name() -> &'static str {
        "WorkflowArgSelector"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl WorkflowArgSelector {
    pub fn new(
        _styles: WorkflowArgSelectorStyles,
        _workflow_enums: &HashMap<SyncId, WorkflowEnumData>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            // Stub - in real implementation this would be created via ctx.add_view
            text_editor: None,
        }
    }

    pub fn set_workflow_enums(
        &mut self,
        _workflow_enums: &HashMap<SyncId, WorkflowEnumData>,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }

    pub fn get_created_enums(&self) -> Vec<SyncId> {
        // Stub implementation
        Vec::new()
    }

    pub fn clear_created_enums(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn enable(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn disable(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn close(&mut self, _ctx: &mut ViewContext<Self>) {
        // Stub implementation
    }

    pub fn get_selected_enum(&self) -> Option<SyncId> {
        // Stub implementation
        None
    }

    pub fn is_dirty(&self, _app: &AppContext) -> bool {
        // Stub implementation
        false
    }

    pub fn set_argument(
        &mut self,
        _arg_type: &crate::workflows::workflow::ArgumentType,
        _value: Option<&String>,
        _workflow_enums: &HashMap<SyncId, WorkflowEnumData>,
        _ctx: &mut ViewContext<Self>,
    ) {
        // Stub implementation
    }
}

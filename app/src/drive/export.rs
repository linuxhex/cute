#![allow(dead_code)]

use std::path::Path;
use cuteui::{Entity, SingletonEntity};

pub struct ExportManager;

impl Entity for ExportManager {
    type Event = ExportManagerEvent;
}

impl SingletonEntity for ExportManager {}

#[derive(Debug, Clone)]
pub enum ExportManagerEvent {
    Export,
}

impl ExportManager {
    pub fn export_to_file(_content: &str, _path: &Path) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn export(
        &mut self,
        _window_id: cuteui::WindowId,
        _exportable_objects: &Vec<crate::drive::CloudObjectTypeAndId>,
        _ctx: &mut cuteui::ModelContext<Self>,
    ) {
        // Stub implementation
    }
}

pub fn safe_filename(name: &str) -> String {
    name.replace(|c: char| c.is_ascii_control() || c == '/' || c == '\\' || c == ':' || c == '*' || c == '?' || c == '"' || c == '<' || c == '>' || c == '|', "_")
}

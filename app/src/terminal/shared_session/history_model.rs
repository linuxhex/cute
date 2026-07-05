//! History model for shared sessions.

use cuteui::Entity;
use crate::terminal::history::HistoryEntry;

/// History model for shared session input.
#[derive(Debug, Clone, Default)]
pub struct SharedSessionHistoryModel {
    entries: Vec<HistoryEntry>,
}

impl SharedSessionHistoryModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(HistoryEntry::command_only(entry));
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Entity for SharedSessionHistoryModel {
    type Event = ();
}

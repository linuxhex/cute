#![allow(dead_code)]

// Stub module for drive import functionality

pub struct DriveImport;

impl DriveImport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DriveImport {
    fn default() -> Self {
        Self::new()
    }
}

// Stub module for import modal
pub mod modal {
    use cuteui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

    #[derive(Debug, Clone, PartialEq)]
    pub enum ImportModalEvent {
        Cancel,
        Import,
        OpenTargetWithHashedId(crate::server::ids::ObjectUid),
        Close,
    }

    pub struct ImportModal;

    impl Entity for ImportModal {
        type Event = ImportModalEvent;
    }

    impl View for ImportModal {
        fn ui_name() -> &'static str {
            "ImportModal"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            // Stub implementation
            cuteui::elements::Empty::new().finish()
        }
    }

    impl TypedActionView for ImportModal {
        type Action = ImportModalEvent;
    }

    impl ImportModal {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn open_with_target(
            &mut self,
            _owner: crate::cloud_object::Owner,
            _initial_folder_id: Option<crate::server::ids::SyncId>,
            _ctx: &mut ViewContext<Self>,
        ) {
            // Stub implementation
        }
    }
}

use cuteui::{AppContext, Element, Entity, View, ViewContext};

#[derive(Debug, Clone, Default)]
pub enum AuthOverrideWarningModalVariant {
    #[default]
    WorkspaceModal,
}

#[derive(Debug, Clone, Default)]
pub struct AuthOverrideWarningModal {
    // TODO: Add fields
}

impl Entity for AuthOverrideWarningModal {
    type Event = AuthOverrideWarningModalEvent;
}

impl View for AuthOverrideWarningModal {
    fn ui_name() -> &'static str {
        "AuthOverrideWarningModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl AuthOverrideWarningModal {
    pub fn new(_ctx: &mut ViewContext<Self>, _variant: AuthOverrideWarningModalVariant) -> Self {
        Self::default()
    }

    pub fn set_interrupted_auth_payload(&mut self, _payload: crate::auth::AuthRedirectPayload) {
        // Stub implementation
    }
}

#[derive(Debug, Clone)]
pub enum AuthOverrideWarningModalEvent {
    Close,
    BulkExport,
}

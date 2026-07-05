use cuteui::{AppContext, Element, Entity, View, ViewContext};

#[derive(Debug, Clone, Default)]
pub enum AuthViewVariant {
    #[default]
    RequireLogin,
    RequireLoginCloseable,
    ShareRequirementCloseable,
}

#[derive(Debug, Clone, Default)]
pub struct AuthView {
    // TODO: Add fields
}

impl Entity for AuthView {
    type Event = AuthViewEvent;
}

impl View for AuthView {
    fn ui_name() -> &'static str {
        "AuthView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        cuteui::elements::Empty::new().finish()
    }
}

impl AuthView {
    pub fn new(_variant: AuthViewVariant, _ctx: &mut ViewContext<Self>) -> Self {
        Self::default()
    }

    pub fn skip_to_browser_open_step(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_variant(&mut self, _variant: AuthViewVariant) {
        // Stub implementation
    }
}

#[derive(Debug, Clone)]
pub enum AuthViewEvent {
    Close,
}

#[derive(Debug, Clone, Default)]
pub struct AuthRedirectPayload {
    // TODO: Add fields
}

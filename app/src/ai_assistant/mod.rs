//! Stub implementation for ai_assistant module
//! This provides minimal types needed for the app to compile

pub mod panel {
    use cuteui::AppContext;

    pub fn init(_ctx: &mut AppContext) {
        // No-op stub
    }
}

pub mod model {
    #[derive(Debug, Clone)]
    pub struct AIAssistantModel;
}

pub mod panel_settings {
    #[derive(Debug, Clone, Default)]
    pub struct AIAssistantPanelSettings;
}

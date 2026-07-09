// Placeholder for Listener - actual implementation may not exist in this codebase
use cuteui::{Entity, ModelContext, SingletonEntity};
use std::sync::Arc;

pub struct Listener;

impl Entity for Listener {
    type Event = ();
}

impl SingletonEntity for Listener {}

impl Listener {
    pub fn new(
        _client: Arc<dyn crate::server::server_api::object::ObjectClient>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        Self
    }

    #[cfg(test)]
    pub fn mock(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn has_current_subscription_abort_handle(&self) -> bool {
        false
    }
}

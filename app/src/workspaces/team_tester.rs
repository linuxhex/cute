use warpui::{Entity, ModelContext, SingletonEntity};

/// Minimal TeamTesterStatus model for local version.
/// In the cloud version, this manages team tester specific polling and data refresh.
/// In the local version, it's a stub that provides the same interface.
#[derive(Clone)]
pub struct TeamTesterStatus {}

impl TeamTesterStatus {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {}
    }

    #[cfg(test)]
    pub fn mock(ctx: &mut ModelContext<Self>) -> Self {
        Self::new(ctx)
    }

    /// Emit an event to start or force-refresh the cloud object and workspace metadata pollers.
    /// In the local version, this is a no-op as there's no cloud to poll.
    pub fn initiate_data_pollers(&mut self, force_refresh: bool, ctx: &mut ModelContext<Self>) {
        ctx.emit(TeamTesterStatusEvent::InitiateDataPollers { force_refresh })
    }
}

pub enum TeamTesterStatusEvent {
    InitiateDataPollers {
        /// If true, the subscriber should attempt to refresh any state
        /// immediately rather than just wait for the next poll.
        force_refresh: bool,
    },
}

impl Entity for TeamTesterStatus {
    type Event = TeamTesterStatusEvent;
}

impl SingletonEntity for TeamTesterStatus {}

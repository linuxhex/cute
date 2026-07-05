use cuteui::{AppContext, ModelContext, SingletonEntity};
use cute_core::errors::{ErrorExt, RegisteredError};

use std::fmt;
use std::error::Error;
use std::ops::Deref;

pub struct WarpDriveSettings {
    pub enable_warp_drive: WarpDriveSettingValue<bool>,
    pub sharing_onboarding_block_shown: WarpDriveSettingValue<bool>,
}

#[derive(Debug, Clone)]
pub struct WarpDriveSettingError;

impl fmt::Display for WarpDriveSettingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WarpDriveSettingError")
    }
}

impl Error for WarpDriveSettingError {}

impl RegisteredError for WarpDriveSettingError {}

impl ErrorExt for WarpDriveSettingError {
    fn is_actionable(&self) -> bool {
        false
    }

    fn report_error(&self) {}
}

pub struct WarpDriveSettingValue<T> {
    value: T,
}

impl<T> WarpDriveSettingValue<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self { value: T::default() }
    }
}

impl WarpDriveSettingValue<bool> {
    pub fn set_value(&mut self, _value: bool, _ctx: &mut ModelContext<WarpDriveSettings>) -> Result<(), WarpDriveSettingError> {
        // Stub implementation - no-op for local version
        Ok(())
    }

    pub fn get_value(&self) -> bool {
        self.value
    }

    pub fn toggle_and_save_value(
        &mut self,
        _ctx: &mut ModelContext<WarpDriveSettings>,
    ) -> Result<bool, WarpDriveSettingError> {
        // Stub implementation - just toggle the in-memory value
        self.value = !self.value;
        Ok(self.value)
    }
}

impl Deref for WarpDriveSettingValue<bool> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl WarpDriveSettings {
    pub fn register(ctx: &mut (impl cuteui::GetSingletonModelHandle + cuteui::AddSingletonModel + cuteui::UpdateModel)) -> cuteui::ModelHandle<Self> {
        ctx.add_singleton_model(|_| Self::default())
    }

    pub fn is_warp_drive_enabled(_app: &AppContext) -> bool {
        false
    }
}

pub enum WarpDriveSettingsChangedEvent {
    EnableWarpDrive { enabled: bool },
}

impl cuteui::Entity for WarpDriveSettingsChangedEvent {
    type Event = ();
}

impl SingletonEntity for WarpDriveSettingsChangedEvent {}

impl cuteui::Entity for WarpDriveSettings {
    type Event = WarpDriveSettingsChangedEvent;
}

impl SingletonEntity for WarpDriveSettings {}

impl Default for WarpDriveSettings {
    fn default() -> Self {
        Self {
            enable_warp_drive: WarpDriveSettingValue::new(),
            sharing_onboarding_block_shown: WarpDriveSettingValue::new(),
        }
    }
}

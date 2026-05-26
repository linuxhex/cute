use cuteui::AppContext;

pub fn init(_ctx: &mut AppContext) {}

#[cfg(target_os = "macos")]
pub mod mac;

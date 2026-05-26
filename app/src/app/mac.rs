//! macOS platform callbacks for app
//! These are stub implementations to satisfy linker requirements

#[no_mangle]
pub extern "C-unwind" fn warp_app_will_finish_launching() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_did_become_active() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_did_resign_active() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_will_terminate() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_should_terminate_app() -> bool { false }

#[no_mangle]
pub extern "C-unwind" fn warp_app_new_window() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_active_window_changed(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_window_did_resize(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_window_did_move(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_window_moved(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_window_will_close(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_should_close_window(_window_id: u64) -> bool { true }

#[no_mangle]
pub extern "C-unwind" fn warp_app_screen_did_change(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_os_appearance_changed() {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_internet_reachability_changed(_reachable: bool) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_open_urls(_urls: *const *const i8, _count: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_open_files(_files: *const *const i8, _count: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_notification_clicked(_notification_id: *const i8, _len: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_process_modal_response(_modal_id: u64, _response: i32) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_send_global_keybinding(_keybinding_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_app_has_binding_for_keystroke(_keycode: u16, _modifiers: u32) -> bool { false }

#[no_mangle]
pub extern "C-unwind" fn warp_app_has_custom_action_for_keystroke(_keycode: u16, _modifiers: u32) -> bool { false }

#[no_mangle]
pub extern "C-unwind" fn warp_app_are_key_bindings_disabled_for_window(_window_id: u64) -> bool { false }

#[no_mangle]
pub extern "C-unwind" fn warp_dealloc_window(_window_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_dispatch_standard_action(_action: *const i8, _len: usize) {}

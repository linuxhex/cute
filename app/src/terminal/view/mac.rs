//! macOS platform callbacks for terminal view
//! These are stub implementations to satisfy linker requirements

// Notification callbacks
#[no_mangle]
pub extern "C" fn warp_on_notification_send_error(_error: i32) {}

#[no_mangle]
pub extern "C" fn warp_on_request_notification_permissions_completed(_result: i32) {}

// File picker callbacks
#[no_mangle]
pub extern "C" fn warp_open_panel_file_selected(_urls: *const std::ffi::c_void, _callback: *mut std::ffi::c_void) {}

#[no_mangle]
pub extern "C" fn warp_save_panel_file_selected(_url: *const std::ffi::c_void, _callback: *mut std::ffi::c_void) {}

// View event handlers
#[no_mangle]
pub extern "C-unwind" fn warp_handle_view_event(_view_id: u64, _event_type: u32, _data: *const u8, _len: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_handle_drag_and_drop(_view_id: u64, _paths: *const *const i8, _count: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_handle_file_drag(_view_id: u64, _paths: *const *const i8, _count: usize) -> bool { false }

#[no_mangle]
pub extern "C-unwind" fn warp_handle_file_drag_exit(_view_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_handle_first_mouse_event(_view_id: u64) -> bool { true }

#[no_mangle]
pub extern "C-unwind" fn warp_handle_insert_text(_view_id: u64, _text: *const i8, _len: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_update_layer(_view_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_view_set_frame_size(_view_id: u64, _width: f64, _height: f64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_view_did_change_backing_properties(_view_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_update_ime_state(_view_id: u64, _has_marked_text: bool) {}

#[no_mangle]
pub extern "C-unwind" fn warp_marked_text_updated(_view_id: u64, _text: *const i8, _len: usize, _selected_range_start: usize, _selected_range_len: usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_marked_text_cleared(_view_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_ime_position(_view_id: u64, _x: *mut f64, _y: *mut f64, _width: *mut f64, _height: *mut f64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_get_accessibility_contents(_view_id: u64, _out: *mut *mut u8, _out_len: *mut usize) {}

#[no_mangle]
pub extern "C-unwind" fn warp_menu_item_triggered(_menu_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_menu_item_needs_update(_menu_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_menu_item_deallocated(_menu_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_close_panel(_panel_id: u64) {}

#[no_mangle]
pub extern "C-unwind" fn warp_toggle_panel(_panel_id: u64) {}

#![cfg(not(target_arch = "wasm32"))]

#[no_mangle]
pub extern "C" fn host_run_plugin_command(_ptr: i32, _len: i32) {
    // no-op stub
}

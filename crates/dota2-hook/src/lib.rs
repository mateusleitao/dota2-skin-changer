pub mod config;
pub mod inventory;

#[cfg(target_os = "windows")]
pub mod gc_hook;
#[cfg(target_os = "windows")]
pub mod proxy;

#[cfg(target_os = "windows")]
use std::ffi::c_void;

#[cfg(target_os = "windows")]
const DLL_PROCESS_ATTACH: u32 = 1;
#[cfg(target_os = "windows")]
const DLL_PROCESS_DETACH: u32 = 0;

/// DLL entry point — only compiled on Windows
#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "system" fn DllMain(
    _dll_module: *mut c_void,
    call_reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            if let Err(e) = initialize() {
                eprintln!("[dota2-hook] Failed to initialize: {e}");
                return 0;
            }
            1
        }
        DLL_PROCESS_DETACH => {
            cleanup();
            1
        }
        _ => 1,
    }
}

#[cfg(target_os = "windows")]
fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    config::load_item_database()?;
    proxy::load_original_dll()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn cleanup() {
    proxy::unload_original_dll();
}

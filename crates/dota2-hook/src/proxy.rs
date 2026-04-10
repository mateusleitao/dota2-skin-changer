//! DLL proxy module — loads the original steam_api64_o.dll and forwards calls.
//! This module is Windows-only.

use windows::core::PCSTR;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{FreeLibrary, GetProcAddress, LoadLibraryA};

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

static ORIGINAL_DLL: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

const ORIGINAL_DLL_NAME: &str = "steam_api64_o.dll\0";

/// Load the original steam_api64_o.dll
pub fn load_original_dll() -> Result<(), Box<dyn std::error::Error>> {
    let name = PCSTR::from_raw(ORIGINAL_DLL_NAME.as_ptr());
    let handle = unsafe { LoadLibraryA(name) }?;

    ORIGINAL_DLL.store(handle.0 as *mut c_void, Ordering::SeqCst);
    Ok(())
}

/// Unload the original DLL
pub fn unload_original_dll() {
    let ptr = ORIGINAL_DLL.swap(std::ptr::null_mut(), Ordering::SeqCst);
    if !ptr.is_null() {
        unsafe {
            let _ = FreeLibrary(HMODULE(ptr));
        }
    }
}

/// Get a function pointer from the original DLL by name
pub fn get_original_proc(name: &str) -> Option<*const c_void> {
    let handle = ORIGINAL_DLL.load(Ordering::SeqCst);
    if handle.is_null() {
        return None;
    }

    let name_cstr = std::ffi::CString::new(name).ok()?;
    let proc = unsafe {
        GetProcAddress(
            HMODULE(handle),
            PCSTR::from_raw(name_cstr.as_ptr() as *const u8),
        )
    };

    proc.map(|f| f as *const c_void)
}

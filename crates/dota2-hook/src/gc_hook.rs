//! ISteamGameCoordinator vtable hook.
//! Intercepts RetrieveMessage to inject inventory items.
//! This module is Windows-only.

use crate::config;
use crate::inventory;
use dota2_proto::{gc_msg, GC_MSG_PROTO_FLAG, GC_MSG_TYPE_MASK};
use std::ffi::c_void;

/// ISteamGameCoordinator vtable layout:
/// [0] SendMessage
/// [1] IsMessageAvailable
/// [2] RetrieveMessage
const VTABLE_RETRIEVE_MESSAGE: usize = 2;

type RetrieveMessageFn = unsafe extern "thiscall" fn(
    this: *mut c_void,
    msg_type: *mut u32,
    dest: *mut u8,
    dest_size: u32,
    msg_size: *mut u32,
) -> i32;

static mut ORIGINAL_RETRIEVE_MESSAGE: Option<RetrieveMessageFn> = None;

/// Hook the ISteamGameCoordinator vtable
///
/// # Safety
/// This modifies vtable pointers in memory. Must only be called once
/// with a valid ISteamGameCoordinator interface pointer.
pub unsafe fn hook_game_coordinator(gc_interface: *mut c_void) -> Result<(), String> {
    if gc_interface.is_null() {
        return Err("Null GC interface pointer".to_string());
    }

    let vtable_ptr = *(gc_interface as *const *const *const c_void);
    let retrieve_msg_ptr = *vtable_ptr.add(VTABLE_RETRIEVE_MESSAGE);

    ORIGINAL_RETRIEVE_MESSAGE = Some(std::mem::transmute::<*const c_void, RetrieveMessageFn>(
        retrieve_msg_ptr,
    ));

    // In a full implementation, we would modify the vtable here using
    // VirtualProtect to make it writable, then replace the pointer.
    // For now, this serves as the architectural skeleton.

    Ok(())
}

/// Hooked RetrieveMessage — calls original then modifies GC responses
///
/// # Safety
/// Called as a vtable replacement. Must maintain exact calling convention.
unsafe extern "thiscall" fn hooked_retrieve_message(
    this: *mut c_void,
    msg_type: *mut u32,
    dest: *mut u8,
    dest_size: u32,
    msg_size: *mut u32,
) -> i32 {
    let original = match ORIGINAL_RETRIEVE_MESSAGE {
        Some(f) => f,
        None => return 0,
    };

    let result = original(this, msg_type, dest, dest_size, msg_size);

    if result != 1 || msg_type.is_null() || msg_size.is_null() {
        return result;
    }

    let raw_type = *msg_type;
    let actual_type = raw_type & GC_MSG_TYPE_MASK;
    let is_protobuf = (raw_type & GC_MSG_PROTO_FLAG) != 0;

    if !is_protobuf {
        return result;
    }

    if actual_type == gc_msg::SO_CACHE_SUBSCRIBED || actual_type == gc_msg::CLIENT_WELCOME {
        if let Some(db) = config::get_item_database() {
            let size = *msg_size as usize;
            let data = std::slice::from_raw_parts(dest, size);

            let account_id = inventory::extract_account_id(data).unwrap_or(0);

            if let Ok(modified) = inventory::inject_items_into_cache(data, &db, account_id) {
                if modified.len() <= dest_size as usize {
                    std::ptr::copy_nonoverlapping(modified.as_ptr(), dest, modified.len());
                    *msg_size = modified.len() as u32;
                }
            }
        }
    }

    result
}

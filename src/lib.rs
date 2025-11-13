mod core;
pub mod modules;
mod state;

pub use core::Core;
pub use modules::components::Pos;

use std::ffi::CString;
use std::os::raw::c_char;

/// Creates a new Core instance and returns a pointer to it.
///
/// # Safety
/// The returned pointer must be passed to `destroy_core` to avoid memory leaks.
/// The caller is responsible for ensuring the pointer is not used after destruction.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_core() -> *mut Core {
    Box::into_raw(Box::new(Core::new()))
}

/// Destroys a Core instance created by `create_core`.
///
/// # Safety
/// The `core` pointer must be obtained from `create_core` and not already destroyed.
/// After calling this function, the pointer becomes invalid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_core(core: *mut Core) {
    if !core.is_null() {
        unsafe {
            drop(Box::from_raw(core));
        }
    }
}

/// Updates the Core instance with the given delta time.
///
/// # Safety
/// The `core` pointer must be valid and obtained from `create_core`.
/// Returns 0 on success, -1 on error (e.g., null pointer).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn update_core(core: *mut Core, delta: f64) -> i32 {
    if core.is_null() {
        return -1;
    }
    unsafe {
        match (*core).update(delta) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

/// Exports the world state as a JSON string.
///
/// # Safety
/// The `core` pointer must be valid and obtained from `create_core`.
/// The returned string must be freed with `free_string` to avoid memory leaks.
/// Returns null pointer on error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn export_world(core: *const Core) -> *mut c_char {
    if core.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let json = (*core).export_world();
        match CString::new(json) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Frees a string allocated by `export_world`.
///
/// # Safety
/// The `s` pointer must be obtained from `export_world` and not already freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

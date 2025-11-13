mod core;
pub mod modules;

pub use core::Core;
pub use modules::components::Pos;

use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_core() -> *mut Core {
    Box::into_raw(Box::new(Core::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_core(core: *mut Core) {
    if !core.is_null() {
        unsafe {
            drop(Box::from_raw(core));
        }
    }
}

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

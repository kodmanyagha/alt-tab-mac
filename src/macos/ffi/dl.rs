// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::{CStr, c_char, c_void};

pub const SKYLIGHT: &CStr = c"/System/Library/PrivateFrameworks/SkyLight.framework/SkyLight";
pub const APPLICATION_SERVICES: &CStr = c"/System/Library/Frameworks/ApplicationServices.framework/ApplicationServices";

const RTLD_LAZY: i32 = 1;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

pub fn open(path: &CStr) -> Option<*mut c_void> {
    // SAFETY: `path` is a NUL-terminated C string.
    let handle = unsafe { dlopen(path.as_ptr(), RTLD_LAZY) };
    (!handle.is_null()).then_some(handle)
}

/// Loads a symbol as a function pointer of type `F`.
///
/// # Safety
/// `F` must be an `extern "C" fn` matching the symbol's real C signature.
pub unsafe fn function<F: Copy>(handle: *mut c_void, name: &CStr) -> Option<F> {
    assert_eq!(std::mem::size_of::<F>(), std::mem::size_of::<*mut c_void>(), "F must be a function pointer");
    let ptr = symbol(handle, name)?;
    // SAFETY: `F` is pointer sized (asserted above) and the caller guarantees it is the symbol's real signature.
    Some(unsafe { std::mem::transmute_copy::<*mut c_void, F>(&ptr) })
}

pub fn symbol(handle: *mut c_void, name: &CStr) -> Option<*mut c_void> {
    // SAFETY: `handle` comes from `dlopen` and `name` is a NUL-terminated C string.
    let ptr = unsafe { dlsym(handle, name.as_ptr()) };
    if ptr.is_null() {
        tracing::warn!(symbol = ?name, "symbol not found");
        None
    } else {
        Some(ptr)
    }
}

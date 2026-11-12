// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;
use std::ptr::NonNull;

use objc2_core_foundation::{CFBoolean, CFNumber, CFRetained, CFString, CFType, Type};

/// Wraps a pointer returned under the "Create" rule (ownership passes to us).
///
/// # Safety
/// `ptr` must be null or a CF object of type `T` with a +1 retain count.
pub unsafe fn owned<T: Type>(ptr: *const c_void) -> Option<CFRetained<T>> {
    // SAFETY: The caller guarantees `ptr` is a `T` with a +1 retain count, which `CFRetained` takes over.
    NonNull::new(ptr as *mut T).map(|p| unsafe { CFRetained::from_raw(p) })
}

pub fn cfstring(s: &str) -> CFRetained<CFString> {
    CFString::from_str(s)
}

/// Raw `CFString` pointer (for constants).
pub fn cfstring_ptr(s: &CFString) -> *const c_void {
    s as *const CFString as *const c_void
}

pub fn to_string(value: &CFType) -> Option<String> {
    value.downcast_ref::<CFString>().map(|s| s.to_string())
}

pub fn to_i64(value: &CFType) -> Option<i64> {
    value.downcast_ref::<CFNumber>().and_then(|n| n.as_i64())
}

pub fn to_f64(value: &CFType) -> Option<f64> {
    value.downcast_ref::<CFNumber>().and_then(|n| n.as_f64())
}

pub fn to_bool(value: &CFType) -> Option<bool> {
    value.downcast_ref::<CFBoolean>().map(|b| b.as_bool())
}

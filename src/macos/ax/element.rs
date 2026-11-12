// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;
use std::ptr::NonNull;

use objc2_core_foundation::{CFRetained, CFType};

use crate::macos::cf;
use crate::macos::ffi::ax::{
    AX_SUCCESS, AXUIElementCopyAttributeValue, AXUIElementCreateApplication, AXUIElementPerformAction, AXUIElementRef,
    AXUIElementSetAttributeValue, AXUIElementSetMessagingTimeout,
};
use crate::macos::ffi::cf::{CFEqual, CFRelease, CFRetain, CFTypeRef};
use std::fmt;

pub struct AxElement(NonNull<c_void>);

// SAFETY: AXUIElement is a CF type whose functions may be called from any thread.
unsafe impl Send for AxElement {}
// SAFETY: AXUIElement is a CF type whose functions may be called from any thread.
unsafe impl Sync for AxElement {}

impl AxElement {
    /// Takes ownership of a pointer with a +1 retain count.
    pub fn from_owned(ptr: AXUIElementRef) -> Option<Self> {
        NonNull::new(ptr as *mut c_void).map(Self)
    }

    /// Wraps a pointer owned by someone else by retaining it.
    pub fn from_borrowed(ptr: AXUIElementRef) -> Option<Self> {
        let element = NonNull::new(ptr as *mut c_void)?;
        // SAFETY: `element` is a non-null AXUIElementRef; the extra retain is balanced by `Drop`.
        unsafe { CFRetain(element.as_ptr()) };
        Some(Self(element))
    }

    pub fn application(pid: i32) -> Option<Self> {
        // SAFETY: `AXUIElementCreateApplication` returns a +1 element or null, which `from_owned` takes over.
        Self::from_owned(unsafe { AXUIElementCreateApplication(pid) })
    }

    pub fn as_ptr(&self) -> AXUIElementRef {
        self.0.as_ptr()
    }

    /// Upper bound so that unresponsive apps cannot block us.
    pub fn set_timeout(&self, seconds: f32) {
        // SAFETY: `self` holds a valid, retained AXUIElementRef.
        unsafe { AXUIElementSetMessagingTimeout(self.as_ptr(), seconds) };
    }

    pub fn attribute(&self, name: &str) -> Option<CFRetained<CFType>> {
        let name = cf::cfstring(name);
        let mut value: CFTypeRef = std::ptr::null();
        // SAFETY: `self` and `name` are valid for the call and `value` is a live local to write into.
        let err = unsafe { AXUIElementCopyAttributeValue(self.as_ptr(), cf::cfstring_ptr(&name), &mut value) };
        if err != AX_SUCCESS {
            return None;
        }
        // SAFETY: `AXUIElementCopyAttributeValue` succeeded, so `value` is a +1 CF object owned by us.
        unsafe { cf::owned(value) }
    }

    pub fn set_attribute(&self, name: &str, value: &CFType) -> bool {
        let name = cf::cfstring(name);
        let value = value as *const CFType as CFTypeRef;
        // SAFETY: `self`, `name` and `value` are valid CF objects for the duration of the call.
        unsafe { AXUIElementSetAttributeValue(self.as_ptr(), cf::cfstring_ptr(&name), value) == AX_SUCCESS }
    }

    pub fn perform(&self, action: &str) -> bool {
        let action = cf::cfstring(action);
        // SAFETY: `self` and `action` are valid CF objects for the duration of the call.
        unsafe { AXUIElementPerformAction(self.as_ptr(), cf::cfstring_ptr(&action)) == AX_SUCCESS }
    }
}

impl Clone for AxElement {
    fn clone(&self) -> Self {
        // SAFETY: `self` holds a valid element; the extra retain belongs to the clone and is balanced by its `Drop`.
        unsafe { CFRetain(self.as_ptr()) };
        Self(self.0)
    }
}

impl Drop for AxElement {
    fn drop(&mut self) {
        // SAFETY: `self` owns exactly one retain on the element, released here once.
        unsafe { CFRelease(self.as_ptr()) };
    }
}

impl PartialEq for AxElement {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: Both elements hold valid, retained CF objects.
        unsafe { CFEqual(self.as_ptr(), other.as_ptr()) != 0 }
    }
}

impl fmt::Debug for AxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AxElement({:p})", self.0)
    }
}

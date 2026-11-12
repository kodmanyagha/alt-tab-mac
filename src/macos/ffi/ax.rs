// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use super::cf::{CFRunLoopSourceRef, CFStringRef, CFTypeRef};

pub type AXUIElementRef = *const c_void;
pub type AXObserverRef = *mut c_void;
pub type AXError = i32;
pub type AXObserverCallback =
    extern "C" fn(observer: AXObserverRef, element: AXUIElementRef, notification: CFStringRef, refcon: *mut c_void);

pub const AX_SUCCESS: AXError = 0;

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    pub static kAXTrustedCheckOptionPrompt: CFStringRef;

    pub fn AXIsProcessTrusted() -> bool;
    pub fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;

    pub fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    pub fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> AXError;
    pub fn AXUIElementSetAttributeValue(element: AXUIElementRef, attribute: CFStringRef, value: CFTypeRef) -> AXError;
    pub fn AXUIElementPerformAction(element: AXUIElementRef, action: CFStringRef) -> AXError;
    pub fn AXUIElementSetMessagingTimeout(element: AXUIElementRef, seconds: f32) -> AXError;

    pub fn AXObserverCreate(pid: i32, callback: AXObserverCallback, observer: *mut AXObserverRef) -> AXError;
    pub fn AXObserverAddNotification(
        observer: AXObserverRef,
        element: AXUIElementRef,
        notification: CFStringRef,
        refcon: *mut c_void,
    ) -> AXError;
    pub fn AXObserverGetRunLoopSource(observer: AXObserverRef) -> CFRunLoopSourceRef;

    /// Private but stable for years: gets the `CGWindowID` of an AX element.
    pub fn _AXUIElementGetWindow(element: AXUIElementRef, window_id: *mut u32) -> AXError;
}

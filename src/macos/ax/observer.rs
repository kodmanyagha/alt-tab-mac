// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use super::element::AxElement;
use crate::macos::cf;
use crate::macos::ffi::ax::{
    AX_SUCCESS, AXObserverAddNotification, AXObserverCallback, AXObserverCreate, AXObserverGetRunLoopSource,
    AXObserverRef,
};
use crate::macos::ffi::cf::{CFRelease, CFRunLoopAddSource, CFRunLoopGetCurrent, kCFRunLoopDefaultMode};

pub mod notification {
    pub const WINDOW_CREATED: &str = "AXWindowCreated";
    pub const FOCUSED_WINDOW_CHANGED: &str = "AXFocusedWindowChanged";
    pub const ELEMENT_DESTROYED: &str = "AXUIElementDestroyed";
    pub const TITLE_CHANGED: &str = "AXTitleChanged";
    pub const WINDOW_MINIATURIZED: &str = "AXWindowMiniaturized";
    pub const WINDOW_DEMINIATURIZED: &str = "AXWindowDeminiaturized";
    pub const WINDOW_MOVED: &str = "AXWindowMoved";
    pub const WINDOW_RESIZED: &str = "AXWindowResized";
}

pub struct AxObserver(AXObserverRef);

// SAFETY: The observer is created, used and released on the tracker thread; `Send` only allows moving the owner there.
unsafe impl Send for AxObserver {}

impl AxObserver {
    /// Creates the observer and adds it to the current thread's run loop.
    pub fn new(pid: i32, callback: AXObserverCallback) -> Option<Self> {
        let mut raw: AXObserverRef = std::ptr::null_mut();
        // SAFETY: `callback` has the AXObserverCallback signature and `raw` is a live local to write into.
        if unsafe { AXObserverCreate(pid, callback, &mut raw) } != AX_SUCCESS || raw.is_null() {
            return None;
        }
        let observer = Self(raw);
        observer.attach_to_current_run_loop();
        Some(observer)
    }

    fn attach_to_current_run_loop(&self) {
        // SAFETY: `self.0` is a valid observer and its source is added to the current thread's run loop.
        unsafe {
            let source = AXObserverGetRunLoopSource(self.0);
            CFRunLoopAddSource(CFRunLoopGetCurrent(), source, kCFRunLoopDefaultMode);
        }
    }

    /// `refcon` is passed back to the callback unchanged (we pass the pid).
    pub fn subscribe(&self, element: &AxElement, name: &str, refcon: usize) -> bool {
        let name = cf::cfstring(name);
        // SAFETY: `self.0`, `element` and `name` are valid; `refcon` is an opaque integer the callback only reads back.
        let err = unsafe {
            AXObserverAddNotification(self.0, element.as_ptr(), cf::cfstring_ptr(&name), refcon as *mut c_void)
        };
        err == AX_SUCCESS
    }
}

impl Drop for AxObserver {
    fn drop(&mut self) {
        // SAFETY: `self` owns exactly one retain on the observer, released here once.
        unsafe { CFRelease(self.0 as *const c_void) };
    }
}

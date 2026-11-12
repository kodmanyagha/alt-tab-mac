// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::{CStr, c_void};
use std::sync::mpsc;
use std::time::Duration;

use block2::RcBlock;
use objc2::msg_send;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Bool};
use objc2_foundation::NSArray;

use crate::core::model::WindowId;
use crate::macos::ffi::dl;
use crate::macos::image::CgImage;

const FRAMEWORK: &CStr = c"/System/Library/Frameworks/ScreenCaptureKit.framework/ScreenCaptureKit";
const TIMEOUT: Duration = Duration::from_secs(2);

struct Sendable(Retained<AnyObject>);
// SAFETY: ScreenCaptureKit content objects are immutable snapshots that may move between threads.
unsafe impl Send for Sendable {}

pub struct Content(Retained<AnyObject>);

pub fn load_content() -> Option<Content> {
    dl::open(FRAMEWORK)?;
    let class = AnyClass::get(c"SCShareableContent")?;
    let (tx, rx) = mpsc::channel();
    let block = RcBlock::new(move |content: *mut AnyObject, _error: *mut AnyObject| {
        // SAFETY: ScreenCaptureKit passes a valid `SCShareableContent` or null; `Retained::retain` handles both.
        let _ = tx.send(unsafe { Retained::retain(content) }.map(Sendable));
    });
    // SAFETY: The selector and argument types match `SCShareableContent`'s class method; the block is copied by the callee.
    let _: () = unsafe {
        msg_send![class, getShareableContentExcludingDesktopWindows: Bool::YES,
                  onScreenWindowsOnly: Bool::NO, completionHandler: &*block]
    };
    rx.recv_timeout(TIMEOUT).ok().flatten().map(|s| Content(s.0))
}

impl Content {
    fn window(&self, id: WindowId) -> Option<Retained<AnyObject>> {
        // SAFETY: `self.0` is an `SCShareableContent`, which responds to `windows`.
        let windows: Retained<NSArray<AnyObject>> = unsafe { msg_send![&*self.0, windows] };
        windows.iter().find(|w| {
            // SAFETY: Each element of `windows` is an `SCWindow`, which responds to `windowID`.
            let wid: u32 = unsafe { msg_send![&**w, windowID] };
            wid == id.0
        })
    }

    /// A `width`×`height` pixel image of the window.
    pub fn capture(&self, id: WindowId, width: usize, height: usize) -> Option<CgImage> {
        let window = self.window(id)?;
        let filter = make_filter(&window)?;
        let config = make_config(width, height)?;
        screenshot(&filter, &config)
    }
}

fn make_filter(window: &AnyObject) -> Option<Retained<AnyObject>> {
    let class = AnyClass::get(c"SCContentFilter")?;
    // SAFETY: `class` is `SCContentFilter`, an NSObject subclass.
    let alloc: Allocated<AnyObject> = unsafe { msg_send![class, alloc] };
    // SAFETY: `alloc` is an uninitialized `SCContentFilter` and `window` is an `SCWindow`.
    unsafe { msg_send![alloc, initWithDesktopIndependentWindow: window] }
}

fn make_config(width: usize, height: usize) -> Option<Retained<AnyObject>> {
    let class = AnyClass::get(c"SCStreamConfiguration")?;
    // SAFETY: `class` is `SCStreamConfiguration`, an NSObject subclass with a plain `new`.
    let config: Retained<AnyObject> = unsafe { msg_send![class, new] };
    // SAFETY: These setters exist on `SCStreamConfiguration` with exactly these argument types.
    unsafe {
        let _: () = msg_send![&*config, setWidth: width];
        let _: () = msg_send![&*config, setHeight: height];
        let _: () = msg_send![&*config, setShowsCursor: Bool::NO];
        let _: () = msg_send![&*config, setIgnoreShadowsSingleWindow: Bool::YES];
    }
    Some(config)
}

fn screenshot(filter: &AnyObject, config: &AnyObject) -> Option<CgImage> {
    let class = AnyClass::get(c"SCScreenshotManager")?;
    let (tx, rx) = mpsc::channel();
    let block = RcBlock::new(move |image: *const c_void, _error: *mut AnyObject| {
        let _ = tx.send(CgImage::from_borrowed(image));
    });
    // SAFETY: The selector and argument types match `SCScreenshotManager`'s class method; the block is copied by the callee.
    let _: () =
        unsafe { msg_send![class, captureImageWithFilter: filter, configuration: config, completionHandler: &*block] };
    rx.recv_timeout(TIMEOUT).ok().flatten()
}

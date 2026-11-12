// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use super::cf::CFStringRef;

pub const LIST_OPTION_ALL: u32 = 0;
pub const LIST_EXCLUDE_DESKTOP_ELEMENTS: u32 = 1 << 4;
pub const NULL_WINDOW_ID: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    pub static kCGWindowNumber: CFStringRef;
    pub static kCGWindowOwnerPID: CFStringRef;
    pub static kCGWindowLayer: CFStringRef;
    pub static kCGWindowBounds: CFStringRef;
    pub static kCGWindowIsOnscreen: CFStringRef;
    pub static kCGWindowAlpha: CFStringRef;

    pub fn CGWindowListCopyWindowInfo(option: u32, relative_to: u32) -> *const c_void;
    pub fn CGRectMakeWithDictionaryRepresentation(dict: *const c_void, rect: *mut CGRect) -> bool;
}

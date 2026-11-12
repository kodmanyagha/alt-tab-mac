// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_core_foundation::{CFArray, CFDictionary, CFRetained, CFString, CFType};

use super::cf;
use super::ffi::cf::CFStringRef;
use super::ffi::window_list::{
    CGRect, CGRectMakeWithDictionaryRepresentation, CGWindowListCopyWindowInfo, LIST_EXCLUDE_DESKTOP_ELEMENTS,
    LIST_OPTION_ALL, NULL_WINDOW_ID, kCGWindowAlpha, kCGWindowBounds, kCGWindowIsOnscreen, kCGWindowLayer,
    kCGWindowNumber, kCGWindowOwnerPID,
};
use crate::core::geometry::Rect;
use crate::core::model::{Pid, WindowId};

const NORMAL_WINDOW_LAYER: i64 = 0;

#[derive(Clone, Debug)]
pub struct CgWindow {
    pub id: WindowId,
    pub pid: Pid,
    pub frame: Rect,
    pub is_onscreen: bool,
}

/// Normal windows on all desktops, front to back (z-order).
pub fn list_windows() -> Vec<CgWindow> {
    let Some(array) = copy_window_info() else {
        return Vec::new();
    };
    // SAFETY: CGWindowListCopyWindowInfo documents each element as a CFDictionary with CFString keys.
    let array: &CFArray<CFDictionary<CFString, CFType>> = unsafe { array.cast_unchecked() };
    array.iter().filter_map(|dict| parse_window(&dict)).collect()
}

fn copy_window_info() -> Option<CFRetained<CFArray>> {
    let options = LIST_OPTION_ALL | LIST_EXCLUDE_DESKTOP_ELEMENTS;
    // SAFETY: `CGWindowListCopyWindowInfo` returns a +1 CFArray or null, which is what `cf::owned` expects.
    unsafe { cf::owned(CGWindowListCopyWindowInfo(options, NULL_WINDOW_ID)) }
}

fn parse_window(dict: &CFDictionary<CFString, CFType>) -> Option<CgWindow> {
    // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
    if get_i64(dict, unsafe { kCGWindowLayer })? != NORMAL_WINDOW_LAYER || is_transparent(dict) {
        return None;
    }
    Some(CgWindow {
        // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
        id: WindowId(get_i64(dict, unsafe { kCGWindowNumber })? as u32),
        // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
        pid: Pid(get_i64(dict, unsafe { kCGWindowOwnerPID })? as i32),
        frame: get_bounds(dict)?,
        // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
        is_onscreen: get(dict, unsafe { kCGWindowIsOnscreen }).and_then(|v| cf::to_bool(&v)).unwrap_or(false),
    })
}

fn is_transparent(dict: &CFDictionary<CFString, CFType>) -> bool {
    // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
    get(dict, unsafe { kCGWindowAlpha }).and_then(|v| cf::to_f64(&v)) == Some(0.0)
}

fn get_bounds(dict: &CFDictionary<CFString, CFType>) -> Option<Rect> {
    // SAFETY: Window info key constants are immutable statics exported by CoreGraphics.
    let value = get(dict, unsafe { kCGWindowBounds })?;
    let mut rect = CGRect::default();
    // SAFETY: `value` is the window's bounds dictionary and `rect` is a live local to write into.
    let ok = unsafe { CGRectMakeWithDictionaryRepresentation(&*value as *const CFType as _, &mut rect) };
    ok.then(|| Rect::new(rect.x, rect.y, rect.width, rect.height))
}

fn get_i64(dict: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<i64> {
    get(dict, key).and_then(|v| cf::to_i64(&v))
}

fn get(dict: &CFDictionary<CFString, CFType>, key: CFStringRef) -> Option<CFRetained<CFType>> {
    // SAFETY: Every caller passes a non-null CFStringRef constant, which has the same layout as `CFString`.
    let key = unsafe { &*(key as *const CFString) };
    dict.get(key)
}

// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use super::cf::CFMachPortRef;

pub type CGEventRef = *mut c_void;
pub type CGEventTapProxy = *mut c_void;
pub type CGEventTapCallBack =
    extern "C" fn(proxy: CGEventTapProxy, kind: u32, event: CGEventRef, user_info: *mut c_void) -> CGEventRef;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

pub const SESSION_EVENT_TAP: u32 = 1;
pub const HEAD_INSERT_EVENT_TAP: u32 = 0;
pub const TAP_OPTION_DEFAULT: u32 = 0;

pub const EVENT_KEY_DOWN: u32 = 10;
pub const EVENT_KEY_UP: u32 = 11;
pub const EVENT_FLAGS_CHANGED: u32 = 12;
pub const EVENT_TAP_DISABLED_BY_TIMEOUT: u32 = 0xFFFF_FFFE;
pub const EVENT_TAP_DISABLED_BY_USER_INPUT: u32 = 0xFFFF_FFFF;

pub const FIELD_KEYBOARD_KEYCODE: u32 = 9;

pub const FLAG_SHIFT: u64 = 0x0002_0000;
pub const FLAG_CONTROL: u64 = 0x0004_0000;
pub const FLAG_ALTERNATE: u64 = 0x0008_0000;
pub const FLAG_COMMAND: u64 = 0x0010_0000;

pub fn event_mask(kinds: &[u32]) -> u64 {
    kinds.iter().fold(0, |mask, &k| mask | (1u64 << k))
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    pub fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: u64,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    pub fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    pub fn CGEventTapIsEnabled(tap: CFMachPortRef) -> bool;
    pub fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
    pub fn CGEventGetFlags(event: CGEventRef) -> u64;
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
}

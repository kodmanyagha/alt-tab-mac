// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

use super::ffi::cf::{
    CFMachPortCreateRunLoopSource, CFMachPortRef, CFRunLoopAddSource, CFRunLoopGetCurrent, kCFRunLoopCommonModes,
};
use super::ffi::event_tap::{
    CGEventGetFlags, CGEventGetIntegerValueField, CGEventGetLocation, CGEventRef, CGEventTapCreate, CGEventTapEnable,
    CGEventTapIsEnabled, CGEventTapProxy, EVENT_FLAGS_CHANGED, EVENT_KEY_DOWN, EVENT_KEY_UP,
    EVENT_TAP_DISABLED_BY_TIMEOUT, EVENT_TAP_DISABLED_BY_USER_INPUT, FIELD_KEYBOARD_KEYCODE, FLAG_ALTERNATE,
    FLAG_COMMAND, FLAG_CONTROL, FLAG_SHIFT, HEAD_INSERT_EVENT_TAP, SESSION_EVENT_TAP, TAP_OPTION_DEFAULT, event_mask,
};
use crate::core::geometry::Point;
use crate::core::hotkey::{Modifier, Modifiers};
use crate::core::input::KeyEvent;

pub type KeyHandler = Box<dyn FnMut(KeyEvent, Point) -> bool>;

struct TapState {
    port: CFMachPortRef,
    handler: KeyHandler,
}

static TAP_PORT: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Re-enables the tap if it was disabled (sleep, timeout). Returns `true` if it had to.
pub fn ensure_enabled() -> bool {
    let port = TAP_PORT.load(Ordering::Relaxed);
    // SAFETY: `port` is non-null here and the tap is never released while the app runs.
    if port.is_null() || unsafe { CGEventTapIsEnabled(port) } {
        return false;
    }
    // SAFETY: `port` is non-null here and the tap is never released while the app runs.
    unsafe { CGEventTapEnable(port, true) };
    true
}

/// The tap lives as long as the app, so its state is leaked on purpose.
pub fn install(handler: KeyHandler) -> bool {
    let state = Box::into_raw(Box::new(TapState { port: std::ptr::null_mut(), handler }));
    let port = create_tap(state as *mut c_void);
    if port.is_null() {
        // SAFETY: `state` came from `Box::into_raw` above and the failed tap never saw it.
        drop(unsafe { Box::from_raw(state) });
        return false;
    }
    // SAFETY: `state` came from `Box::into_raw` above and nothing else accesses it yet.
    unsafe { (*state).port = port };
    TAP_PORT.store(port, Ordering::Relaxed);
    add_to_main_run_loop(port);
    true
}

fn create_tap(user_info: *mut c_void) -> CFMachPortRef {
    let mask = event_mask(&[EVENT_KEY_DOWN, EVENT_KEY_UP, EVENT_FLAGS_CHANGED]);
    // SAFETY: `on_event` matches the CGEventTapCallBack signature and `user_info` is the leaked `TapState`.
    unsafe { CGEventTapCreate(SESSION_EVENT_TAP, HEAD_INSERT_EVENT_TAP, TAP_OPTION_DEFAULT, mask, on_event, user_info) }
}

fn add_to_main_run_loop(port: CFMachPortRef) {
    // SAFETY: `port` is a valid tap; the run loop source is added to the main thread's run loop.
    unsafe {
        let source = CFMachPortCreateRunLoopSource(std::ptr::null(), port, 0);
        CFRunLoopAddSource(CFRunLoopGetCurrent(), source, kCFRunLoopCommonModes);
        CGEventTapEnable(port, true);
    }
}

extern "C" fn on_event(_proxy: CGEventTapProxy, kind: u32, event: CGEventRef, user_info: *mut c_void) -> CGEventRef {
    // SAFETY: `user_info` is the leaked `TapState`; the callback only runs on the main run loop, so access is exclusive.
    let state = unsafe { &mut *(user_info as *mut TapState) };
    if kind == EVENT_TAP_DISABLED_BY_TIMEOUT || kind == EVENT_TAP_DISABLED_BY_USER_INPUT {
        tracing::warn!("event tap was disabled, re-enabling");
        // SAFETY: `state.port` was set before the tap was enabled and is never released.
        unsafe { CGEventTapEnable(state.port, true) };
        return event;
    }
    let Some(key_event) = to_key_event(kind, event) else {
        return event;
    };
    // SAFETY: `event` is the valid event passed to the tap callback.
    let location = unsafe { CGEventGetLocation(event) };
    let swallow = (state.handler)(key_event, Point { x: location.x, y: location.y });
    if swallow { std::ptr::null_mut() } else { event }
}

fn to_key_event(kind: u32, event: CGEventRef) -> Option<KeyEvent> {
    // SAFETY: `event` is the valid event passed to the tap callback.
    let code = unsafe { CGEventGetIntegerValueField(event, FIELD_KEYBOARD_KEYCODE) } as u16;
    // SAFETY: `event` is the valid event passed to the tap callback.
    let mods = to_modifiers(unsafe { CGEventGetFlags(event) });
    match kind {
        EVENT_KEY_DOWN => Some(KeyEvent::Down { code, mods }),
        EVENT_KEY_UP => Some(KeyEvent::Up { code }),
        EVENT_FLAGS_CHANGED => Some(KeyEvent::FlagsChanged { mods }),
        _ => None,
    }
}

fn to_modifiers(flags: u64) -> Modifiers {
    [
        (FLAG_COMMAND, Modifier::Cmd),
        (FLAG_ALTERNATE, Modifier::Alt),
        (FLAG_CONTROL, Modifier::Ctrl),
        (FLAG_SHIFT, Modifier::Shift),
    ]
    .into_iter()
    .filter(|(flag, _)| flags & flag != 0)
    .fold(Modifiers::NONE, |acc, (_, m)| acc.with(m))
}

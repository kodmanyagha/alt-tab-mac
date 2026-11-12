// SPDX-License-Identifier: GPL-3.0-or-later

use super::ax::AxElement;
use super::ffi::skylight::{CPS_USER_GENERATED, ProcessSerialNumber, SkyLight, skylight};
use crate::core::model::{Pid, WindowId};

pub struct FocusTarget {
    pub pid: Pid,
    pub window_id: WindowId,
    pub window: AxElement,
    pub app: AxElement,
    pub is_minimized: bool,
    pub is_app_hidden: bool,
}

/// Performs AX IPC; do not call it on the main thread.
pub fn focus(target: &FocusTarget) {
    restore_visibility(target);
    let focused_by_skylight = skylight().is_some_and(|sl| focus_with_skylight(sl, target));
    if !focused_by_skylight {
        activate_with_appkit(target.pid);
    }
    target.window.raise();
}

fn restore_visibility(target: &FocusTarget) {
    if target.is_app_hidden {
        target.app.set_hidden(false);
    }
    if target.is_minimized {
        target.window.set_minimized(false);
    }
}

fn focus_with_skylight(sl: &SkyLight, target: &FocusTarget) -> bool {
    let mut psn = ProcessSerialNumber::default();
    // SAFETY: `psn` is a live local the function writes into; the pointer is resolved with this signature.
    if unsafe { (sl.get_process_for_pid)(target.pid.0, &mut psn) } != 0 {
        return false;
    }
    // SAFETY: `psn` was filled in by `GetProcessForPID` and the function pointer has this signature.
    let ok = unsafe { (sl.set_front_process)(&psn, target.window_id.0, CPS_USER_GENERATED) } == 0;
    if ok {
        make_key_window(sl, &psn, target.window_id);
    }
    ok
}

/// Sends WindowServer a "make this the key window" event. The byte layout is WindowServer's internal event record
/// format.
fn make_key_window(sl: &SkyLight, psn: &ProcessSerialNumber, window_id: WindowId) {
    let mut record = [0u8; 0xf8];
    record[0x04] = 0xf8;
    record[0x3a] = 0x10;
    record[0x3c..0x40].copy_from_slice(&window_id.0.to_ne_bytes());
    record[0x20..0x30].fill(0xff);
    for phase in [0x01, 0x02] {
        record[0x08] = phase;
        // SAFETY: `record` is a 0xf8-byte buffer, the size the private event record API reads.
        unsafe { (sl.post_event_record)(psn, record.as_ptr()) };
    }
}

fn activate_with_appkit(pid: Pid) {
    use objc2_app_kit::{NSApplicationActivationOptions, NSRunningApplication};
    let app = NSRunningApplication::runningApplicationWithProcessIdentifier(pid.0);
    if let Some(app) = app {
        #[allow(deprecated)]
        app.activateWithOptions(NSApplicationActivationOptions::ActivateIgnoringOtherApps);
    }
}

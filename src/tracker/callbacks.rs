// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use objc2_core_foundation::CFString;

use crate::core::model::Pid;
use crate::macos::ax::{AxElement, notification as n};

use super::state::Tracker;
use super::tracker_handle::with_tracker;

/// C callback of the AX observer. `refcon` is the app's pid.
pub extern "C" fn on_ax_notification(
    _observer: *mut c_void,
    element: *const c_void,
    name: *const c_void,
    refcon: *mut c_void,
) {
    let Some(element) = AxElement::from_borrowed(element) else {
        return;
    };
    // SAFETY: The AX observer callback always passes a non-null CFStringRef notification name.
    let name = unsafe { &*(name as *const CFString) }.to_string();
    let pid = Pid(refcon as usize as i32);
    with_tracker(|t| {
        t.on_notification(pid, element, &name);
        t.publish();
    });
}

impl Tracker {
    fn on_notification(&mut self, pid: Pid, element: AxElement, name: &str) {
        match name {
            n::WINDOW_CREATED => self.on_window_created(pid, element),
            n::FOCUSED_WINDOW_CHANGED => self.on_focus_changed(pid, element),
            n::ELEMENT_DESTROYED => self.on_destroyed(pid, &element),
            n::TITLE_CHANGED | n::WINDOW_MINIATURIZED | n::WINDOW_DEMINIATURIZED => {
                self.on_window_changed(pid, &element)
            }
            n::WINDOW_MOVED | n::WINDOW_RESIZED => self.schedule_reconcile(),
            _ => {}
        }
    }

    fn on_window_created(&mut self, pid: Pid, element: AxElement) {
        self.add_or_update_window(pid, element);
        self.schedule_reconcile();
    }

    fn on_focus_changed(&mut self, pid: Pid, element: AxElement) {
        let id = self.window_by_element(pid, &element).or_else(|| self.add_or_update_window(pid, element));
        if let (Some(id), true) = (id, self.frontmost == Some(pid)) {
            self.touch(id);
        }
    }

    fn on_destroyed(&mut self, pid: Pid, element: &AxElement) {
        if let Some(id) = self.window_by_element(pid, element) {
            self.remove_window(id);
        }
    }

    fn on_window_changed(&mut self, pid: Pid, element: &AxElement) {
        if let Some(id) = self.window_by_element(pid, element) {
            self.refresh_window(id);
        }
    }
}

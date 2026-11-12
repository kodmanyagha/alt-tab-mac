// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::{Pid, WindowInfo};
use crate::core::spaces::space_state;
use crate::macos::spaces;

use super::state::Tracker;

pub fn is_self(pid: Pid) -> bool {
    pid.0 as u32 == std::process::id()
}

impl Tracker {
    /// `None` means the settings window was closed.
    pub fn set_self_window(&mut self, window: Option<WindowInfo>) {
        if let Some(old) = self.self_window.take() {
            self.mru.remove(old.id);
        }
        if let Some(window) = window {
            self.touch(window.id);
            self.self_window = Some(window);
            self.refresh_self_window_space();
        }
    }

    /// When our app comes to the front, the focus is on the settings window.
    pub fn activate_self(&mut self) -> bool {
        let Some(id) = self.self_window.as_ref().map(|w| w.id) else {
            return false;
        };
        self.touch(id);
        true
    }

    pub fn refresh_self_window_space(&mut self) {
        let active = self.active_spaces.clone();
        let Some(window) = self.self_window.as_mut() else {
            return;
        };
        let window_spaces = spaces::spaces_of_window(window.id);
        let state = space_state(window_spaces.as_deref(), active.as_deref(), true, window.is_minimized, false);
        window.is_on_active_space = state.on_active_space;
    }
}

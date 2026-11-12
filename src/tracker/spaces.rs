// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::WindowId;
use crate::core::spaces::space_state;
use crate::macos::spaces;

use super::state::Tracker;

impl Tracker {
    pub fn refresh_spaces(&mut self) {
        self.active_spaces = spaces::active_spaces();
        let ids: Vec<WindowId> = self.windows.keys().copied().collect();
        for id in ids {
            self.refresh_window_space(id);
        }
        self.refresh_self_window_space();
    }

    pub fn refresh_window_space(&mut self, id: WindowId) {
        let window_spaces = spaces::spaces_of_window(id);
        let onscreen = self.cg.get(&id).is_some_and(|w| w.is_onscreen);
        let Some(window) = self.windows.get_mut(&id) else {
            return;
        };
        let app_hidden = self.apps.get(&window.info.pid).is_some_and(|a| a.is_hidden);
        let state = space_state(
            window_spaces.as_deref(),
            self.active_spaces.as_deref(),
            onscreen,
            window.info.is_minimized,
            app_hidden,
        );
        window.info.is_on_active_space = state.on_active_space;
        window.is_orphan = state.orphan;
    }
}

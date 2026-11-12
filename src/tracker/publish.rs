// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use crate::core::model::{Snapshot, WindowId, WindowInfo};

use super::state::Tracker;
use super::store::FocusHandles;

impl Tracker {
    pub fn publish(&mut self) {
        self.store.publish(self.build_snapshot());
        self.registry.replace(self.focus_handles());
        (self.on_publish)();
    }

    fn build_snapshot(&self) -> Snapshot {
        Snapshot {
            windows: self.windows_in_mru_order(),
            frontmost_pid: self.frontmost,
            focused_window: self.focused,
            screens: self.screens.clone(),
            screen_names: self.screen_names.clone(),
        }
    }

    fn windows_in_mru_order(&self) -> Vec<WindowInfo> {
        let tracked = self.windows.values().filter(|w| !w.is_orphan).map(|w| self.decorate(&w.info));
        let mut list: Vec<WindowInfo> = tracked.chain(self.self_window.iter().cloned()).collect();
        list.sort_by_key(|w| (self.mru.rank(w.id), w.id));
        list
    }

    /// Copies app information (name, hidden) onto the window.
    fn decorate(&self, info: &WindowInfo) -> WindowInfo {
        let mut info = info.clone();
        if let Some(app) = self.apps.get(&info.pid) {
            info.app_name.clone_from(&app.name);
            info.bundle_id.clone_from(&app.bundle_id);
            info.is_app_hidden = app.is_hidden;
        }
        info
    }

    fn focus_handles(&self) -> HashMap<WindowId, FocusHandles> {
        self.windows
            .iter()
            .filter_map(|(&id, w)| {
                let app = self.apps.get(&w.info.pid)?;
                Some((id, FocusHandles { pid: w.info.pid, window: w.element.clone(), app: app.element.clone() }))
            })
            .collect()
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;

use crate::core::model::{Pid, WindowId};
use crate::macos::run_loop::RunLoopHandle;
use crate::macos::window_list::{CgWindow, list_windows};

use super::state::Tracker;
use super::tracker_handle::with_tracker;
use crate::core::mru::MruList;

impl Tracker {
    pub fn reconcile(&mut self) {
        let list = list_windows();
        self.cg = list.iter().map(|w| (w.id, w.clone())).collect();
        self.remove_closed_windows();
        self.update_geometry();
        self.scan_unknown_windows(&list);
        self.refresh_spaces();
        self.retry_unobserved_apps();
    }

    /// On startup, builds the MRU order from the windows' z-order (front to back).
    pub fn seed_mru_from_z_order(&mut self, list: &[CgWindow]) {
        let known: HashSet<WindowId> = self.windows.keys().copied().collect();
        let mut mru = MruList::default();
        for w in list.iter().filter(|w| known.contains(&w.id)) {
            mru.add_if_missing(w.id);
        }
        for &id in &known {
            mru.add_if_missing(id);
        }
        self.mru = mru;
    }

    /// Coalesces several events into a single reconciliation.
    pub fn schedule_reconcile(&mut self) {
        if self.reconcile_scheduled {
            return;
        }
        self.reconcile_scheduled = true;
        RunLoopHandle::current().perform(|| {
            with_tracker(|t| {
                t.reconcile_scheduled = false;
                t.reconcile();
                t.publish();
            })
        });
    }

    fn remove_closed_windows(&mut self) {
        let closed: Vec<WindowId> = self.windows.keys().filter(|id| !self.cg.contains_key(id)).copied().collect();
        for id in closed {
            self.remove_window(id);
        }
        self.ignored.retain(|id| self.cg.contains_key(id));
    }

    fn update_geometry(&mut self) {
        for (id, window) in self.windows.iter_mut() {
            if let Some(cg) = self.cg.get(id) {
                window.info.frame = cg.frame;
            }
        }
    }

    /// Rescans the apps of windows that CG knows about but we do not.
    fn scan_unknown_windows(&mut self, list: &[CgWindow]) {
        let pids = self.pids_with_unknown_windows(list);
        for pid in pids {
            self.scan_app_windows(pid);
        }
        let unknown: Vec<WindowId> = list.iter().map(|w| w.id).filter(|id| self.is_unknown(*id)).collect();
        self.ignored.extend(unknown);
    }

    fn pids_with_unknown_windows(&self, list: &[CgWindow]) -> HashSet<Pid> {
        list.iter().filter(|w| self.apps.contains_key(&w.pid) && self.is_unknown(w.id)).map(|w| w.pid).collect()
    }

    fn is_unknown(&self, id: WindowId) -> bool {
        !self.windows.contains_key(&id) && !self.ignored.contains(&id)
    }
}

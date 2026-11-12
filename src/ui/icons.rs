// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use objc2::rc::Retained;
use objc2_app_kit::{NSImage, NSRunningApplication};

use crate::core::model::Pid;

#[derive(Default)]
pub struct IconCache {
    icons: HashMap<Pid, Retained<NSImage>>,
}

impl IconCache {
    pub fn get(&self, pid: Pid) -> Option<&NSImage> {
        self.icons.get(&pid).map(|i| &**i)
    }

    /// Loads icons for pids not seen yet and drops the ones that are gone.
    pub fn sync(&mut self, pids: impl Iterator<Item = Pid>) {
        let wanted: Vec<Pid> = pids.collect();
        self.icons.retain(|pid, _| wanted.contains(pid));
        for pid in wanted {
            if !self.icons.contains_key(&pid) {
                self.load(pid);
            }
        }
    }

    fn load(&mut self, pid: Pid) {
        let icon = NSRunningApplication::runningApplicationWithProcessIdentifier(pid.0).and_then(|app| app.icon());
        if let Some(icon) = icon {
            self.icons.insert(pid, icon);
        }
    }
}

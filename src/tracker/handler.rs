// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::{Pid, ScreenInfo};
use crate::macos::window_list::list_windows;
use crate::macos::workspace::AppDesc;

use super::messages::Msg;
use super::state::Tracker;
use std::time::Instant;

impl Tracker {
    pub fn handle(&mut self, msg: Msg) {
        match msg {
            Msg::Init { apps, frontmost, screens } => self.init(apps, frontmost, screens),
            Msg::AppLaunched(desc) => self.add_app(desc),
            Msg::AppTerminated(pid) => self.remove_app(pid),
            Msg::AppActivated(pid) => self.activate_app(pid),
            Msg::AppHidden(pid, hidden) => self.set_app_hidden(pid, hidden),
            Msg::SpaceChanged => self.reconcile(),
            Msg::ScreensChanged(screens) => self.on_screens_changed(screens),
            Msg::Touch(id) => self.touch(id),
            Msg::SelfWindow(window) => self.set_self_window(window),
            Msg::Reconcile => self.reconcile(),
        }
        self.publish();
    }

    fn init(&mut self, apps: Vec<AppDesc>, frontmost: Option<Pid>, screens: Vec<ScreenInfo>) {
        let started = Instant::now();
        self.set_screens(screens);
        let list = list_windows();
        self.cg = list.iter().map(|w| (w.id, w.clone())).collect();
        for app in apps {
            self.add_app(app);
        }
        self.seed_mru_from_z_order(&list);
        if let Some(pid) = frontmost {
            self.activate_app(pid);
        }
        tracing::info!(window_count = self.windows.len(), duration = ?started.elapsed(), "initial scan");
    }

    fn on_screens_changed(&mut self, screens: Vec<ScreenInfo>) {
        self.set_screens(screens);
        self.reconcile();
    }

    fn set_screens(&mut self, screens: Vec<ScreenInfo>) {
        self.screens = screens.iter().map(|s| s.frame).collect();
        self.screen_names = screens.into_iter().map(|s| s.name).collect();
    }
}

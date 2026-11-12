// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::{Pid, WindowId};
use crate::macos::ax::{AxElement, AxObserver, notification as n};
use crate::macos::workspace::AppDesc;

use super::callbacks::on_ax_notification;
use super::self_window::is_self;
use super::state::{TrackedApp, Tracker};

const AX_TIMEOUT_SECONDS: f32 = 0.25;

const MAX_OBSERVE_ATTEMPTS: u8 = 20;

const APP_NOTIFICATIONS: [&str; 2] = [n::WINDOW_CREATED, n::FOCUSED_WINDOW_CHANGED];

impl Tracker {
    pub fn add_app(&mut self, desc: AppDesc) {
        if is_self(desc.pid) {
            return;
        }
        let Some(element) = AxElement::application(desc.pid.0) else {
            return;
        };
        element.set_timeout(AX_TIMEOUT_SECONDS);
        let app = TrackedApp {
            name: desc.name,
            bundle_id: desc.bundle_id,
            is_hidden: desc.is_hidden,
            element,
            observer: None,
            observe_attempts: 0,
        };
        self.apps.insert(desc.pid, app);
        self.try_observe_app(desc.pid);
    }

    /// AX is not ready for a while in newly launched apps, so on failure this is retried a limited number of times
    /// during reconciliation. Windows are scanned even without notifications (periodic reconciliation keeps the list up
    /// to date).
    pub fn try_observe_app(&mut self, pid: Pid) {
        let Some(app) = self.apps.get_mut(&pid) else {
            return;
        };
        app.observe_attempts += 1;
        app.observer = subscribe_app(pid, &app.element);
        if app.observer.is_some() {
            self.subscribe_known_windows(pid);
        }
        self.scan_app_windows(pid);
    }

    pub fn retry_unobserved_apps(&mut self) {
        let pending: Vec<Pid> = self
            .apps
            .iter()
            .filter(|(_, a)| a.observer.is_none() && a.observe_attempts < MAX_OBSERVE_ATTEMPTS)
            .map(|(&p, _)| p)
            .collect();
        for pid in pending {
            self.try_observe_app(pid);
        }
    }

    pub fn remove_app(&mut self, pid: Pid) {
        self.apps.remove(&pid);
        let ids: Vec<_> = self.windows.iter().filter(|(_, w)| w.info.pid == pid).map(|(&id, _)| id).collect();
        for id in ids {
            self.remove_window(id);
        }
    }

    pub fn set_app_hidden(&mut self, pid: Pid, hidden: bool) {
        if let Some(app) = self.apps.get_mut(&pid) {
            app.is_hidden = hidden;
        }
        let ids: Vec<_> = self.windows.iter().filter(|(_, w)| w.info.pid == pid).map(|(&id, _)| id).collect();
        for id in ids {
            self.refresh_window_space(id);
        }
    }

    /// The focused window of the app coming to the front moves to the head of the MRU list.
    pub fn activate_app(&mut self, pid: Pid) {
        self.frontmost = Some(pid);
        if is_self(pid) && self.activate_self() {
            return;
        }
        let id = self.focused_window_of(pid).or_else(|| {
            self.scan_app_windows(pid);
            self.focused_window_of(pid)
        });
        match id {
            Some(id) => self.touch(id),
            None => self.focused = None,
        }
    }

    fn focused_window_of(&self, pid: Pid) -> Option<WindowId> {
        let window = self.apps.get(&pid)?.element.focused_window()?;
        self.window_by_element(pid, &window)
    }

    pub fn touch(&mut self, id: WindowId) {
        self.mru.touch(id);
        self.focused = Some(id);
    }
}

/// Returns the observer if at least one notification could be subscribed.
fn subscribe_app(pid: Pid, element: &AxElement) -> Option<AxObserver> {
    let observer = AxObserver::new(pid.0, on_ax_notification)?;
    let results = APP_NOTIFICATIONS.map(|name| observer.subscribe(element, name, pid.0 as usize));
    results.contains(&true).then_some(observer)
}

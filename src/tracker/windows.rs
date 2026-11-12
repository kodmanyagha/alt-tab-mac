// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::{Pid, WindowId, WindowInfo};
use crate::core::window_kind::{WindowKind, classify};
use crate::macos::ax::{AxElement, notification as n};
use crate::macos::window_list::list_windows;

use super::state::{TrackedWindow, Tracker};

const WINDOW_NOTIFICATIONS: [&str; 6] = [
    n::ELEMENT_DESTROYED,
    n::TITLE_CHANGED,
    n::WINDOW_MINIATURIZED,
    n::WINDOW_DEMINIATURIZED,
    n::WINDOW_MOVED,
    n::WINDOW_RESIZED,
];

impl Tracker {
    /// Reads the app's AX windows: adds new ones and updates known ones.
    pub fn scan_app_windows(&mut self, pid: Pid) {
        let Some(app) = self.apps.get(&pid) else {
            return;
        };
        for element in app.element.windows() {
            self.add_or_update_window(pid, element);
        }
    }

    pub fn add_or_update_window(&mut self, pid: Pid, element: AxElement) -> Option<WindowId> {
        let kind = kind_of(&element);
        if kind == WindowKind::Other {
            return None;
        }
        let id = WindowId(element.window_id()?);
        if !self.is_normal_layer(id) {
            return None;
        }
        if let Some(existing) = self.windows.get_mut(&id) {
            refresh_from_ax(&mut existing.info, &existing.element);
            existing.info.kind = kind;
        } else {
            self.insert_window(pid, id, element, kind);
        }
        Some(id)
    }

    /// Whether the window is on the normal application layer. Menu bar items, popovers and our switcher panel are on
    /// other layers. If a newly opened window is not in the list yet, the list is refreshed once (< 1 ms).
    fn is_normal_layer(&mut self, id: WindowId) -> bool {
        if !self.cg.contains_key(&id) {
            self.cg = list_windows().into_iter().map(|w| (w.id, w)).collect();
        }
        self.cg.contains_key(&id)
    }

    fn insert_window(&mut self, pid: Pid, id: WindowId, element: AxElement, kind: WindowKind) {
        let mut info = self.new_window_info(pid, id, kind);
        refresh_from_ax(&mut info, &element);
        self.subscribe_window(pid, &element);
        self.ignored.remove(&id);
        self.mru.add_if_missing(id);
        self.windows.insert(id, TrackedWindow { info, element, is_orphan: false });
        self.refresh_window_space(id);
    }

    fn new_window_info(&self, pid: Pid, id: WindowId, kind: WindowKind) -> WindowInfo {
        let cg = self.cg.get(&id);
        WindowInfo {
            id,
            pid,
            app_name: String::new(),
            bundle_id: String::new(),
            title: String::new(),
            frame: cg.map(|w| w.frame).unwrap_or_default(),
            is_minimized: false,
            is_app_hidden: false,
            is_on_active_space: cg.is_some_and(|w| w.is_onscreen),
            kind,
        }
    }

    fn subscribe_window(&self, pid: Pid, element: &AxElement) {
        let Some(observer) = self.apps.get(&pid).and_then(|a| a.observer.as_ref()) else {
            return;
        };
        for name in WINDOW_NOTIFICATIONS {
            observer.subscribe(element, name, pid.0 as usize);
        }
    }

    /// If the observer was created later, subscribes to the windows that were already known.
    pub fn subscribe_known_windows(&self, pid: Pid) {
        for window in self.windows.values().filter(|w| w.info.pid == pid) {
            self.subscribe_window(pid, &window.element);
        }
    }

    pub fn remove_window(&mut self, id: WindowId) {
        self.windows.remove(&id);
        self.mru.remove(id);
        if self.focused == Some(id) {
            self.focused = None;
        }
    }

    pub fn refresh_window(&mut self, id: WindowId) {
        if let Some(w) = self.windows.get_mut(&id) {
            refresh_from_ax(&mut w.info, &w.element);
        }
        self.refresh_window_space(id);
    }
}

/// Window kind (standard, dialog, panel…). Unknown kinds are never tracked; the others are tracked and the filter
/// decides whether they are shown.
fn kind_of(element: &AxElement) -> WindowKind {
    classify(element.subrole().as_deref(), element.is_modal())
}

/// Title and minimized state are read from AX (CG titles need Screen Recording permission, AX does not).
fn refresh_from_ax(info: &mut WindowInfo, element: &AxElement) {
    info.title = element.title().unwrap_or_default();
    info.is_minimized = element.is_minimized();
}

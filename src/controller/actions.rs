// SPDX-License-Identifier: GPL-3.0-or-later

use super::app_controller::AppController;
use crate::activator::Command;
use crate::core::actions::WindowAction;
use crate::core::model::WindowId;

impl AppController {
    pub(super) fn act(&mut self, action: WindowAction) {
        let Some(id) = self.switcher.selected_id() else {
            return;
        };
        let _ = self.activator.send(Command::Act(id, action));
        if action.removes_window() {
            self.remove_from_session(&self.affected_windows(id, action));
        } else {
            self.mark_in_session(id, action);
        }
    }

    /// Quitting an app removes all of that app's windows from the list.
    fn affected_windows(&self, id: WindowId, action: WindowAction) -> Vec<WindowId> {
        let Some(pid) = self.session_windows.iter().find(|w| w.id == id).map(|w| w.pid) else {
            return vec![id];
        };
        match action {
            WindowAction::QuitApp => self.session_windows.iter().filter(|w| w.pid == pid).map(|w| w.id).collect(),
            _ => vec![id],
        }
    }

    fn remove_from_session(&mut self, ids: &[WindowId]) {
        self.session_windows.retain(|w| !ids.contains(&w.id));
        match self.switcher.remove(ids) {
            Some(selected) if !self.session_windows.is_empty() => self.refresh_panel(selected),
            _ => self.cancel(),
        }
    }

    fn mark_in_session(&mut self, id: WindowId, action: WindowAction) {
        let pid = self.session_windows.iter().find(|w| w.id == id).map(|w| w.pid);
        for w in self.session_windows.iter_mut() {
            match action {
                WindowAction::Minimize if w.id == id => w.is_minimized = true,
                WindowAction::HideApp if Some(w.pid) == pid => w.is_app_hidden = true,
                _ => {}
            }
        }
    }
}

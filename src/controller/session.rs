// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::{Duration, Instant};

use dispatch2::{DispatchQueue, DispatchTime};

use super::app_controller::{AppController, with_controller};
use super::context::{filter_context, panel_screens};
use crate::activator::Command;
use crate::core::filter::filter_windows;
use crate::core::geometry::Point;
use crate::core::grid_nav::{self, Direction, Grid};
use crate::core::sort::sort_windows;
use crate::tracker::messages::Msg;
use crate::ui::Look;
use crate::ui::panel::Images;

impl AppController {
    pub(super) fn start_session(&mut self, hotkey: usize, reverse: bool, cursor: Point, started: Instant) {
        let snapshot = self.store.current();
        let ctx = filter_context(&snapshot, &self.config, cursor);
        let mut windows = filter_windows(&snapshot, &self.bindings.scopes[hotkey], &ctx);
        sort_windows(&mut windows, self.config.appearance.sort);
        let ids = windows.iter().map(|w| w.id).collect();
        self.session_windows = windows.into_iter().cloned().collect();
        let selected = self.switcher.start(hotkey, ids, snapshot.focused_window, reverse);
        self.session_token += 1;
        self.session_screens = panel_screens(&snapshot, &self.config.general.panel_screen, cursor);
        self.session_look = Look::new(self.bindings.styles[hotkey], self.config.appearance.clone());
        self.show_now_or_later(selected);
        tracing::debug!(latency = ?started.elapsed(), window_count = self.session_windows.len(), "shortcut to panel");
        if self.session_look.style.uses_previews() {
            self.request_thumbnails(self.session_windows.iter());
        }
    }

    fn show_now_or_later(&mut self, selected: usize) {
        match self.config.general.show_delay_ms {
            0 => self.show_panel(selected),
            delay => schedule_delayed_show(self.session_token, delay),
        }
    }

    fn show_panel(&mut self, selected: usize) {
        if self.session_windows.is_empty() {
            return;
        }
        let windows: Vec<_> = self.session_windows.iter().collect();
        let images = Images { icons: &self.icons, thumbnails: &self.thumbnails };
        self.panel.show(&windows, selected, &images, &self.session_look, &self.session_screens);
    }

    /// Redraws the panel when the list changes (window closed, etc.).
    pub(super) fn refresh_panel(&mut self, selected: usize) {
        if self.panel.is_visible() {
            self.show_panel(selected);
        }
    }

    pub(super) fn step(&mut self, delta: isize) {
        if let Some(index) = self.switcher.step(delta) {
            self.panel.select(index);
        }
    }

    /// Arrow keys: jumps rows or columns according to the panel's grid shape.
    pub(super) fn move_selection(&mut self, direction: Direction) {
        let (Some(current), Some(shape)) = (self.switcher.selected(), self.panel.grid_shape()) else { return };
        let grid = Grid {
            columns: shape.0,
            rows: shape.1,
            count: self.session_windows.len(),
            orientation: self.session_look.appearance.orientation,
        };
        if let Some(index) = self.switcher.select(grid_nav::step(current, direction, grid)) {
            self.panel.select(index);
        }
    }

    pub(super) fn commit(&mut self) {
        let chosen = self.switcher.commit();
        self.end_session();
        if let Some(id) = chosen {
            self.tracker.send(Msg::Touch(id));
            let _ = self.activator.send(Command::Focus(id));
        }
    }

    pub(super) fn cancel(&mut self) {
        self.switcher.cancel();
        self.end_session();
    }

    fn end_session(&mut self) {
        self.panel.hide();
        self.session_windows.clear();
        self.session_token += 1;
    }
}

/// Shows the panel after `delay_ms` if the shortcut is still held. If it was released in the meantime (quick switch),
/// the panel never appears.
fn schedule_delayed_show(token: u64, delay_ms: u64) {
    let Ok(when) = DispatchTime::try_from(Duration::from_millis(delay_ms)) else {
        return;
    };
    let _ = DispatchQueue::main().after(when, move || {
        with_controller(|c| {
            if c.session_token == token
                && let Some(selected) = c.switcher.selected()
            {
                c.show_panel(selected);
            }
        });
    });
}

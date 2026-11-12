// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Instant;

use super::app_controller::{AppController, with_controller};
use crate::core::geometry::Point;
use crate::core::input::{Intent, KeyEvent, should_swallow};
use crate::ui::settings::recorder;

/// Event tap callback. Returns `true` to swallow the event.
pub fn on_key(event: KeyEvent, cursor: Point) -> bool {
    if recorder::is_recording() {
        return recorder::capture(event);
    }
    let started = Instant::now();
    with_controller(|c| c.handle_key(event, cursor, started)).unwrap_or(false)
}

impl AppController {
    fn handle_key(&mut self, event: KeyEvent, cursor: Point, started: Instant) -> bool {
        let intent = self.router.route(event, &self.bindings.hotkeys, self.switcher.active_hotkey());
        match intent {
            Intent::Trigger { hotkey, reverse } => self.start_session(hotkey, reverse, cursor, started),
            Intent::Step(delta) => self.step(delta),
            Intent::Move(direction) => self.move_selection(direction),
            Intent::Commit => self.commit(),
            Intent::Cancel => self.cancel(),
            Intent::Act(action) => self.act(action),
            Intent::Swallow | Intent::PassThrough => {}
        }
        should_swallow(event, intent)
    }
}

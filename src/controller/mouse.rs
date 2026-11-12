// SPDX-License-Identifier: GPL-3.0-or-later

use super::app_controller::{AppController, with_controller};
use crate::core::geometry::Point;

/// `point` is relative to the top-left corner of the panel.
pub fn on_mouse_move(point: Point) {
    with_controller(|c| c.hover(point));
}

pub fn on_mouse_click(point: Point) {
    with_controller(|c| {
        if c.hover(point) {
            c.commit();
        }
    });
}

impl AppController {
    /// Selects the item under the point. Returns `true` if an item was found.
    fn hover(&mut self, point: Point) -> bool {
        if !self.config.general.mouse_selection {
            return false;
        }
        let Some(index) = self.panel.item_at(point) else {
            return false;
        };
        if self.switcher.select(index).is_some() {
            self.panel.select(index);
            return true;
        }
        false
    }
}

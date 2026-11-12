// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::Config;
use crate::core::filter::FilterContext;
use crate::core::geometry::{Point, Rect};
use crate::core::model::Snapshot;
use crate::core::panel_screen::{PanelScreen, ScreenRefs, resolve};
use crate::core::scope::ScreenReference;
use crate::core::screens::{screen_of_point, screen_of_rect};

pub fn filter_context<'a>(snapshot: &Snapshot, config: &'a Config, cursor: Point) -> FilterContext<'a> {
    FilterContext {
        frontmost_pid: snapshot.frontmost_pid,
        current_screen: current_screen(snapshot, config.general.screen_reference, cursor),
        kinds: config.window_kinds,
        blacklist: &config.blacklist,
    }
}

fn current_screen(snapshot: &Snapshot, reference: ScreenReference, cursor: Point) -> Option<usize> {
    let by_cursor = || screen_of_point(cursor, &snapshot.screens);
    match reference {
        ScreenReference::Cursor => by_cursor(),
        ScreenReference::ActiveWindow => focused_window_screen(snapshot).or_else(by_cursor),
    }
}

fn focused_window_screen(snapshot: &Snapshot) -> Option<usize> {
    let window = snapshot.window(snapshot.focused_window?)?;
    screen_of_rect(&window.frame, &snapshot.screens)
}

/// Screens the panel is drawn on (one or all, depending on the setting).
pub fn panel_screens(snapshot: &Snapshot, choice: &PanelScreen, cursor: Point) -> Vec<Rect> {
    let refs = ScreenRefs {
        names: &snapshot.screen_names,
        active_window: focused_window_screen(snapshot),
        cursor: screen_of_point(cursor, &snapshot.screens),
    };
    let screens: Vec<Rect> =
        resolve(choice, &refs).into_iter().filter_map(|i| snapshot.screens.get(i).copied()).collect();
    if screens.is_empty() { vec![FALLBACK_SCREEN] } else { screens }
}

const FALLBACK_SCREEN: Rect = Rect { x: 0.0, y: 0.0, width: 1440.0, height: 900.0 };

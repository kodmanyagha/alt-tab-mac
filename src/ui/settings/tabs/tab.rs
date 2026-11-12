// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use super::{about, appearance, general, shortcuts, windows};
use crate::config::Config;
use crate::i18n::Key;

pub struct Tab {
    pub title: Key,
    pub build: fn(MainThreadMarker, &Config) -> Retained<NSView>,
}

pub const TABS: [Tab; 5] = [
    Tab { title: Key::TabGeneral, build: general::build },
    Tab { title: Key::TabShortcuts, build: shortcuts::build },
    Tab { title: Key::TabAppearance, build: appearance::build },
    Tab { title: Key::TabWindows, build: windows::build },
    Tab { title: Key::TabAbout, build: |mtm, _| about::build(mtm) },
];

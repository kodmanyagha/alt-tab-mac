// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use objc2::MainThreadMarker;
use objc2_app_kit::{NSAppearance, NSAppearanceNameAqua, NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::NSPoint;

use super::settings_window;
use super::tabs::TABS;
use super::window::make_window;
use crate::ui::png::{pump_run_loop, save_png};

pub fn render_all(mtm: MainThreadMarker, dir: &Path) {
    use objc2_app_kit::NSAppearanceCustomization;

    crate::i18n::apply_for_tools(crate::config::load_or_create().general.language);
    NSApplication::sharedApplication(mtm).setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    let (window, tabs) = make_window(mtm);
    // SAFETY: Appearance name constants are immutable statics exported by AppKit.
    window.setAppearance(NSAppearance::appearanceNamed(unsafe { NSAppearanceNameAqua }).as_deref());
    settings_window::rebuild_into(mtm, &tabs);
    window.setFrameOrigin(NSPoint::new(-20_000.0, -20_000.0));
    window.orderFront(None);
    for (index, tab) in TABS.iter().enumerate() {
        tabs.selectTabViewItemAtIndex(index as isize);
        pump_run_loop();
        let Some(view) = window.contentView() else { continue };
        let path = dir.join(format!("{index}-{:?}.png", tab.title));
        match save_png(&view, &path) {
            Ok(()) => println!("{}", path.display()),
            Err(err) => eprintln!("{}: {err}", path.display()),
        }
    }
}

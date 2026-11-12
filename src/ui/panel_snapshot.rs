// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

use super::icons::IconCache;
use super::panel::{Images, SwitcherPanel};
use super::png::{pump_run_loop, save_png};
use super::theme::Look;
use super::thumbnails::ThumbnailCache;
use crate::config;
use crate::core::appearance::Appearance;
use crate::core::appearance::Orientation;
use crate::core::geometry::Rect;
use crate::core::model::{Pid, WindowId, WindowInfo};
use crate::core::style::Style;
use crate::core::window_kind::WindowKind;
use crate::macos::workspace;

const STYLES: [Style; 4] = [Style::Icons, Style::Previews, Style::PreviewsWithIcon, Style::List];
const OFFSCREEN: Rect = Rect { x: -20_000.0, y: -20_000.0, width: 1440.0, height: 900.0 };

pub fn render_all(mtm: MainThreadMarker, dir: &Path) {
    NSApplication::sharedApplication(mtm).setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    let windows = sample_windows();
    let mut icons = IconCache::default();
    icons.sync(windows.iter().map(|w| w.pid));
    let images = Images { icons: &icons, thumbnails: &ThumbnailCache::default() };
    let mut panel = SwitcherPanel::new(mtm);
    let appearance = config::load_or_create().appearance;
    for orientation in [Orientation::Horizontal, Orientation::Vertical] {
        for style in STYLES {
            let look = Look::new(style, Appearance { orientation, ..appearance.clone() });
            render_one(&mut panel, &windows, &images, &look, &dir.join(format!("{orientation:?}-{style:?}.png")));
        }
    }
}

fn render_one(panel: &mut SwitcherPanel, windows: &[WindowInfo], images: &Images, look: &Look, path: &Path) {
    let refs: Vec<&WindowInfo> = windows.iter().collect();
    panel.show(&refs, 1, images, look, &OFFSCREEN);
    pump_run_loop();
    match panel.content_view().map(|v| save_png(&v, path)) {
        Some(Ok(())) => println!("{}", path.display()),
        Some(Err(err)) => eprintln!("{}: {err}", path.display()),
        None => eprintln!("{}: no content view", path.display()),
    }
    panel.hide();
}

/// Sample windows from the running apps (at most 6).
fn sample_windows() -> Vec<WindowInfo> {
    workspace::running_apps().into_iter().take(6).enumerate().map(|(i, app)| sample(i, app.pid, &app.name)).collect()
}

fn sample(i: usize, pid: Pid, app: &str) -> WindowInfo {
    WindowInfo {
        id: WindowId(i as u32 + 1),
        pid,
        app_name: app.into(),
        bundle_id: String::new(),
        title: format!("{app} — Belge {}", i + 1),
        frame: Rect::default(),
        is_minimized: false,
        is_app_hidden: false,
        is_on_active_space: true,
        kind: WindowKind::Standard,
    }
}

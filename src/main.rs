// SPDX-License-Identifier: GPL-3.0-or-later

mod activator;
mod app;
mod cli;
mod config;
mod controller;
mod core;
mod diagnose;
mod i18n;
mod logging;
mod macos;
mod thumbnails;
mod tracker;
mod ui;

use objc2::MainThreadMarker;

use cli::Mode;

fn main() {
    let mode = cli::parse();
    let _log_guard = logging::init(mode.log_target());
    match mode {
        Mode::App => app::run(),
        Mode::Diagnose => diagnose::run(),
        Mode::RenderSettings(dir) => ui::settings::snapshot::render_all(main_thread(), &dir),
        Mode::RenderPanel(dir) => ui::panel_snapshot::render_all(main_thread(), &dir),
    }
}

/// Startup only: `main` always runs on the main thread.
fn main_thread() -> MainThreadMarker {
    MainThreadMarker::new().expect("main() runs on the main thread")
}

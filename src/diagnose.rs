// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Instant;

use crate::core::window_kind::classify;
use crate::macos::ax::AxElement;
use crate::macos::capture::{self, CaptureTarget};
use crate::macos::window_list::{CgWindow, list_windows};
use crate::macos::{os_version, permissions, spaces, workspace};

pub fn run() {
    println!("macOS major version     : {}", os_version::major());
    println!("Accessibility permission: {}", permissions::is_trusted());
    println!("Screen Recording        : {}", capture::has_permission());
    println!("Active desktops         : {:?}", spaces::active_spaces());
    let windows = list_windows();
    print_windows(&windows);
    print_ax_windows();
    try_capture(&windows);
}

fn print_windows(windows: &[CgWindow]) {
    println!("Windows ({}):", windows.len());
    for w in windows {
        let spaces = spaces::spaces_of_window(w.id);
        println!("  {:>6} pid {:>6} onscreen={:<5} desktops={:?}", w.id.0, w.pid.0, w.is_onscreen, spaces);
    }
}

/// AX windows of the apps and their kinds (for window classification).
fn print_ax_windows() {
    println!("AX windows:");
    for app in workspace::running_apps() {
        let Some(element) = AxElement::application(app.pid.0) else { continue };
        element.set_timeout(0.5);
        for w in element.windows() {
            print_ax_window(&app.name, &w);
        }
    }
}

fn print_ax_window(app: &str, w: &AxElement) {
    let subrole = w.subrole();
    let modal = w.is_modal();
    println!(
        "  {:<18} id={:<6} role={:<10} subrole={:<24} modal={:<5} kind={:<13?} {:?}",
        app,
        w.window_id().map_or("-".into(), |id| id.to_string()),
        w.role().unwrap_or_default(),
        subrole.clone().unwrap_or_default(),
        modal,
        classify(subrole.as_deref(), modal),
        w.title().unwrap_or_default(),
    );
}

fn try_capture(windows: &[CgWindow]) {
    let Some(w) = windows.iter().find(|w| w.is_onscreen) else {
        println!("Capture: no window on screen");
        return;
    };
    let started = Instant::now();
    let result = capture::capture_all(&[CaptureTarget { id: w.id, frame: w.frame }]);
    match result.first() {
        Some((_, img)) => println!("Capture: {}x{} pixels, {:?}", img.width(), img.height(), started.elapsed()),
        None => println!("Capture: failed ({:?})", started.elapsed()),
    }
}

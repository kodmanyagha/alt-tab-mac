// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use objc2_foundation::NSBundle;

/// Path of the `.app` bundle; `None` when running outside a bundle (`cargo run`).
pub fn app_path() -> Option<PathBuf> {
    let path = PathBuf::from(NSBundle::mainBundle().bundlePath().to_string());
    (path.extension()? == "app").then_some(path)
}

pub fn executable_path() -> Option<PathBuf> {
    NSBundle::mainBundle().executablePath().map(|p| PathBuf::from(p.to_string()))
}

pub fn identifier() -> String {
    NSBundle::mainBundle().bundleIdentifier().map(|s| s.to_string()).unwrap_or_else(|| "emirbugra.alttabmac.app".into())
}

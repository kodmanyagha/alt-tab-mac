// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use objc2_app_kit::{NSBitmapImageFileType, NSView};
use objc2_foundation::{NSDate, NSDictionary, NSRunLoop};

/// Spins the run loop briefly so drawing can finish.
pub fn pump_run_loop() {
    let until = NSDate::dateWithTimeIntervalSinceNow(0.3);
    NSRunLoop::currentRunLoop().runUntilDate(&until);
}

pub fn save_png(view: &NSView, path: &Path) -> Result<(), String> {
    view.layoutSubtreeIfNeeded();
    let bounds = view.bounds();
    let rep = view.bitmapImageRepForCachingDisplayInRect(bounds).ok_or("could not create a bitmap")?;
    view.cacheDisplayInRect_toBitmapImageRep(bounds, &rep);
    // SAFETY: An empty properties dictionary is a valid argument for PNG encoding.
    let data = unsafe { rep.representationUsingType_properties(NSBitmapImageFileType::PNG, &NSDictionary::new()) }
        .ok_or("could not encode PNG")?;
    std::fs::write(path, data.to_vec()).map_err(|e| e.to_string())
}

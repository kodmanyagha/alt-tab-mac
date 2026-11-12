// SPDX-License-Identifier: GPL-3.0-or-later

use super::{legacy, sck};
use crate::core::geometry::Rect;
use crate::core::model::WindowId;
use crate::macos::ffi::cg_image::{CGPreflightScreenCaptureAccess, CGRequestScreenCaptureAccess};
use crate::macos::image::CgImage;
use crate::macos::os_version;

pub const MAX_WIDTH_PX: usize = 480;

pub struct CaptureTarget {
    pub id: WindowId,
    pub frame: Rect,
}

pub fn has_permission() -> bool {
    // SAFETY: `CGPreflightScreenCaptureAccess` has no preconditions.
    unsafe { CGPreflightScreenCaptureAccess() }
}

/// Asks for Screen Recording permission (opens the system prompt).
pub fn request_permission() -> bool {
    // SAFETY: `CGRequestScreenCaptureAccess` has no preconditions.
    unsafe { CGRequestScreenCaptureAccess() }
}

/// Captures the windows. Windows that cannot be captured (other desktop, minimized) are left out of the result.
pub fn capture_all(targets: &[CaptureTarget]) -> Vec<(WindowId, CgImage)> {
    if os_version::at_least(14) { capture_with_sck(targets) } else { capture_with_legacy(targets) }
}

fn capture_with_sck(targets: &[CaptureTarget]) -> Vec<(WindowId, CgImage)> {
    let Some(content) = sck::load_content() else {
        return Vec::new();
    };
    let mut images = Vec::new();
    for target in targets {
        let (width, height) = pixel_size(&target.frame);
        if let Some(image) = content.capture(target.id, width, height) {
            images.push((target.id, image));
        }
    }
    images
}

fn capture_with_legacy(targets: &[CaptureTarget]) -> Vec<(WindowId, CgImage)> {
    let mut images = Vec::new();
    for target in targets {
        if let Some(image) = legacy::capture(target.id) {
            images.push((target.id, image.scaled_to_width(MAX_WIDTH_PX)));
        }
    }
    images
}

/// 2x on Retina but at most `MAX_WIDTH_PX` wide; the aspect ratio is kept.
fn pixel_size(frame: &Rect) -> (usize, usize) {
    let width = (frame.width * 2.0).min(MAX_WIDTH_PX as f64).max(1.0);
    let height = (width * frame.height / frame.width.max(1.0)).max(1.0);
    (width as usize, height as usize)
}

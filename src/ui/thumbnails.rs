// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

use objc2::rc::Retained;
use objc2_app_kit::NSImage;
use objc2_foundation::NSSize;

use crate::core::model::WindowId;
use crate::macos::image::CgImage;
use objc2_core_graphics::CGImage;

#[derive(Default)]
pub struct ThumbnailCache {
    images: HashMap<WindowId, Retained<NSImage>>,
}

impl ThumbnailCache {
    pub fn get(&self, id: WindowId) -> Option<&NSImage> {
        self.images.get(&id).map(|i| &**i)
    }

    pub fn insert(&mut self, id: WindowId, image: &CgImage) -> &NSImage {
        use objc2::AllocAnyThread;

        // SAFETY: `CgImage` always holds a valid, retained CGImageRef, which has the same layout as `CGImage`; the borrow lives no longer than `image`.
        let cg = unsafe { &*(image.as_ptr() as *const CGImage) };
        let ns = NSImage::initWithCGImage_size(NSImage::alloc(), cg, NSSize::ZERO);
        self.images.insert(id, ns);
        &self.images[&id]
    }

    /// Drops the images of windows that no longer exist.
    pub fn retain_only(&mut self, alive: impl Iterator<Item = WindowId>) {
        let alive: Vec<WindowId> = alive.collect();
        self.images.retain(|id, _| alive.contains(id));
    }
}

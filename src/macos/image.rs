// SPDX-License-Identifier: GPL-3.0-or-later

use std::ptr::NonNull;

use super::ffi::cg_image::{
    BITMAP_INFO_BGRA_PREMULTIPLIED, CGBitmapContextCreate, CGBitmapContextCreateImage, CGColorSpaceCreateDeviceRGB,
    CGColorSpaceRelease, CGContextDrawImage, CGContextRelease, CGContextSetInterpolationQuality, CGImageGetHeight,
    CGImageGetWidth, CGImageRef, CGImageRelease, CGImageRetain, CGRect, INTERPOLATION_HIGH,
};

pub struct CgImage(NonNull<std::ffi::c_void>);

// SAFETY: CGImage is immutable and documented by Apple as thread-safe.
unsafe impl Send for CgImage {}
// SAFETY: CGImage is immutable and documented by Apple as thread-safe.
unsafe impl Sync for CgImage {}

impl CgImage {
    /// Takes ownership of a pointer with a +1 retain count.
    pub fn from_owned(ptr: CGImageRef) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(Self)
    }

    /// Wraps a pointer owned by someone else by retaining it.
    pub fn from_borrowed(ptr: CGImageRef) -> Option<Self> {
        let image = Self::from_owned(ptr)?;
        // SAFETY: `ptr` is non-null (checked by `from_owned`) and the extra retain is balanced by `Drop`.
        unsafe { CGImageRetain(ptr) };
        Some(image)
    }

    pub fn as_ptr(&self) -> CGImageRef {
        self.0.as_ptr()
    }

    pub fn width(&self) -> usize {
        // SAFETY: `self` always holds a valid, retained CGImageRef.
        unsafe { CGImageGetWidth(self.as_ptr()) }
    }

    pub fn height(&self) -> usize {
        // SAFETY: `self` always holds a valid, retained CGImageRef.
        unsafe { CGImageGetHeight(self.as_ptr()) }
    }

    /// Scales the image down, keeping the aspect ratio, if it is wider than `max_width` pixels.
    pub fn scaled_to_width(self, max_width: usize) -> Self {
        if self.width() <= max_width || self.width() == 0 {
            return self;
        }
        let height = (self.height() * max_width / self.width()).max(1);
        draw_scaled(&self, max_width, height).unwrap_or(self)
    }
}

impl Drop for CgImage {
    fn drop(&mut self) {
        // SAFETY: `self` owns exactly one retain on the image, released here once.
        unsafe { CGImageRelease(self.as_ptr()) };
    }
}

fn draw_scaled(image: &CgImage, width: usize, height: usize) -> Option<CgImage> {
    // SAFETY: Every created CG object is checked for null and released exactly once; `image` is a valid CGImageRef.
    unsafe {
        let space = CGColorSpaceCreateDeviceRGB();
        let ctx =
            CGBitmapContextCreate(std::ptr::null_mut(), width, height, 8, 0, space, BITMAP_INFO_BGRA_PREMULTIPLIED);
        CGColorSpaceRelease(space);
        if ctx.is_null() {
            return None;
        }
        CGContextSetInterpolationQuality(ctx, INTERPOLATION_HIGH);
        let rect = CGRect { x: 0.0, y: 0.0, width: width as f64, height: height as f64 };
        CGContextDrawImage(ctx, rect, image.as_ptr());
        let scaled = CgImage::from_owned(CGBitmapContextCreateImage(ctx));
        CGContextRelease(ctx);
        scaled
    }
}

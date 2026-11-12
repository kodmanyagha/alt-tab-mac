// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

pub type CGImageRef = *const c_void;
pub type CGContextRef = *mut c_void;
pub type CGColorSpaceRef = *const c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub const BITMAP_INFO_BGRA_PREMULTIPLIED: u32 = 2 | (2 << 12);
pub const INTERPOLATION_HIGH: i32 = 3;

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    pub fn CGImageGetWidth(image: CGImageRef) -> usize;
    pub fn CGImageGetHeight(image: CGImageRef) -> usize;
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);

    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    pub fn CGColorSpaceRelease(space: CGColorSpaceRef);
    pub fn CGBitmapContextCreate(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: u32,
    ) -> CGContextRef;
    pub fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;
    pub fn CGContextSetInterpolationQuality(context: CGContextRef, quality: i32);
    pub fn CGContextDrawImage(context: CGContextRef, rect: CGRect, image: CGImageRef);
    pub fn CGContextRelease(context: CGContextRef);

    pub fn CGPreflightScreenCaptureAccess() -> bool;
    pub fn CGRequestScreenCaptureAccess() -> bool;
}

// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::OnceLock;

use crate::core::model::WindowId;
use crate::macos::ffi::cg_image::{CGImageRef, CGRect};
use crate::macos::ffi::dl;
use crate::macos::image::CgImage;
use std::ffi::CStr;

type CreateImageFn = unsafe extern "C" fn(bounds: CGRect, options: u32, window: u32, image_options: u32) -> CGImageRef;

const LIST_OPTION_INCLUDING_WINDOW: u32 = 1 << 3;
const IMAGE_BOUNDS_IGNORE_FRAMING: u32 = 1 << 0;
const IMAGE_NOMINAL_RESOLUTION: u32 = 1 << 4;
const RECT_NULL: CGRect = CGRect { x: f64::INFINITY, y: f64::INFINITY, width: 0.0, height: 0.0 };

const CORE_GRAPHICS: &CStr = c"/System/Library/Frameworks/CoreGraphics.framework/CoreGraphics";

pub fn capture(id: WindowId) -> Option<CgImage> {
    let create = create_image_fn()?;
    let options = IMAGE_BOUNDS_IGNORE_FRAMING | IMAGE_NOMINAL_RESOLUTION;
    // SAFETY: `create` was resolved with this exact signature and all arguments are plain values.
    let raw = unsafe { create(RECT_NULL, LIST_OPTION_INCLUDING_WINDOW, id.0, options) };
    CgImage::from_owned(raw)
}

fn create_image_fn() -> Option<CreateImageFn> {
    static CELL: OnceLock<Option<CreateImageFn>> = OnceLock::new();
    *CELL.get_or_init(|| {
        let cg = dl::open(CORE_GRAPHICS)?;
        // SAFETY: `CreateImageFn` matches the C signature of `CGWindowListCreateImage`.
        unsafe { dl::function::<CreateImageFn>(cg, c"CGWindowListCreateImage") }
    })
}

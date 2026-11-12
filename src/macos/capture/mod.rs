// SPDX-License-Identifier: GPL-3.0-or-later

mod legacy;
mod sck;
mod window_capture;

pub use window_capture::{CaptureTarget, capture_all, has_permission, request_permission};

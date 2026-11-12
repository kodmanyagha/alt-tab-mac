// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::CStr;
use std::ptr;

use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Bool};

use crate::macos::ffi::dl;
use objc2_foundation::NSString;

const FRAMEWORK: &CStr = c"/System/Library/Frameworks/ServiceManagement.framework/ServiceManagement";
const STATUS_ENABLED: isize = 1;

fn main_app_service() -> Option<Retained<AnyObject>> {
    dl::open(FRAMEWORK)?;
    let class = AnyClass::get(c"SMAppService")?;
    // SAFETY: `class` is `SMAppService`, whose `mainAppService` class method returns an object.
    unsafe { msg_send![class, mainAppService] }
}

pub fn is_enabled() -> bool {
    let Some(service) = main_app_service() else {
        return false;
    };
    // SAFETY: `service` is an `SMAppService`, whose `status` returns an NSInteger.
    let status: isize = unsafe { msg_send![&*service, status] };
    status == STATUS_ENABLED
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let service = main_app_service().ok_or("SMAppService is not available")?;
    let mut error: *mut AnyObject = ptr::null_mut();
    // SAFETY: `service` is an `SMAppService`; `error` is a live local the method may write an NSError into.
    let ok: Bool = unsafe {
        if enabled {
            msg_send![&*service, registerAndReturnError: &mut error]
        } else {
            msg_send![&*service, unregisterAndReturnError: &mut error]
        }
    };
    if ok.as_bool() { Ok(()) } else { Err(describe(error)) }
}

fn describe(error: *mut AnyObject) -> String {
    if error.is_null() {
        return "unknown error".into();
    }
    // SAFETY: `error` is non-null and points to the NSError written by `SMAppService`.
    let description: Retained<NSString> = unsafe { msg_send![error, localizedDescription] };
    description.to_string()
}

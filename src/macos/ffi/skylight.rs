// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::OnceLock;

use super::dl;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessSerialNumber {
    pub high: u32,
    pub low: u32,
}

pub type SetFrontProcessFn = unsafe extern "C" fn(psn: *const ProcessSerialNumber, wid: u32, mode: u32) -> i32;
pub type PostEventRecordFn = unsafe extern "C" fn(psn: *const ProcessSerialNumber, bytes: *const u8) -> i32;
pub type GetProcessForPidFn = unsafe extern "C" fn(pid: i32, psn: *mut ProcessSerialNumber) -> i32;

pub const CPS_USER_GENERATED: u32 = 0x200;

pub struct SkyLight {
    pub set_front_process: SetFrontProcessFn,
    pub post_event_record: PostEventRecordFn,
    pub get_process_for_pid: GetProcessForPidFn,
}

/// Loads the functions once, then returns them from the cache.
pub fn skylight() -> Option<&'static SkyLight> {
    static CELL: OnceLock<Option<SkyLight>> = OnceLock::new();
    CELL.get_or_init(load).as_ref()
}

fn load() -> Option<SkyLight> {
    let skylight = dl::open(dl::SKYLIGHT)?;
    let services = dl::open(dl::APPLICATION_SERVICES)?;
    // SAFETY: Each function type matches the real C signature of the symbol it is loaded from.
    unsafe {
        Some(SkyLight {
            set_front_process: dl::function(skylight, c"_SLPSSetFrontProcessWithOptions")?,
            post_event_record: dl::function(skylight, c"SLPSPostEventRecordTo")?,
            get_process_for_pid: dl::function(services, c"GetProcessForPID")?,
        })
    }
}

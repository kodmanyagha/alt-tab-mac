// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;
use std::sync::OnceLock;

use super::dl;

pub type ConnectionId = i32;

pub const ALL_SPACES_MASK: i32 = 0x7;

pub struct SpacesApi {
    pub main_connection_id: unsafe extern "C" fn() -> ConnectionId,
    pub copy_managed_display_spaces: unsafe extern "C" fn(cid: ConnectionId) -> *const c_void,
    pub copy_spaces_for_windows:
        unsafe extern "C" fn(cid: ConnectionId, mask: i32, window_ids: *const c_void) -> *const c_void,
}

pub fn spaces_api() -> Option<&'static SpacesApi> {
    static CELL: OnceLock<Option<SpacesApi>> = OnceLock::new();
    CELL.get_or_init(load).as_ref()
}

fn load() -> Option<SpacesApi> {
    let sl = dl::open(dl::SKYLIGHT)?;
    // SAFETY: Each function type matches the real C signature of the SkyLight symbol it is loaded from.
    unsafe {
        Some(SpacesApi {
            main_connection_id: dl::function(sl, c"SLSMainConnectionID")?,
            copy_managed_display_spaces: dl::function(sl, c"SLSCopyManagedDisplaySpaces")?,
            copy_spaces_for_windows: dl::function(sl, c"SLSCopySpacesForWindows")?,
        })
    }
}

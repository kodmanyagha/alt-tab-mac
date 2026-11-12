// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_core_foundation::{CFArray, CFDictionary, CFNumber, CFRetained, CFString, CFType};

use super::cf;
use super::ffi::spaces::{ALL_SPACES_MASK, SpacesApi, spaces_api};
use crate::core::model::{SpaceId, WindowId};

/// The desktop currently shown on each screen.
pub fn active_spaces() -> Option<Vec<SpaceId>> {
    let api = spaces_api()?;
    // SAFETY: The SkyLight function returns a +1 CFArray or null, which is what `cf::owned` expects.
    let displays = unsafe { cf::owned::<CFArray>((api.copy_managed_display_spaces)(connection(api))) }?;
    // SAFETY: SkyLight documents each element of the managed display spaces array as a CFDictionary with CFString keys.
    let displays: &CFArray<CFDictionary<CFString, CFType>> = unsafe { displays.cast_unchecked() };
    Some(displays.iter().filter_map(|d| current_space_of_display(&d)).collect())
}

/// Desktops the window is on. An empty list means it is on no desktop (closed but not destroyed, or minimized).
pub fn spaces_of_window(id: WindowId) -> Option<Vec<SpaceId>> {
    let api = spaces_api()?;
    let number = CFNumber::new_i64(id.0 as i64);
    let ids = CFArray::from_retained_objects(&[number]);
    let ids_ptr = &*ids as *const CFArray<CFNumber> as *const std::ffi::c_void;
    // SAFETY: `ids_ptr` points to a live CFArray of CFNumber window ids for the duration of the call.
    let raw = unsafe { (api.copy_spaces_for_windows)(connection(api), ALL_SPACES_MASK, ids_ptr) };
    // SAFETY: `SLSCopySpacesForWindows` returns a +1 CFArray or null, which is what `cf::owned` expects.
    let spaces = unsafe { cf::owned::<CFArray>(raw) }?;
    Some(parse_space_ids(&spaces))
}

fn connection(api: &SpacesApi) -> i32 {
    // SAFETY: `SLSMainConnectionID` takes no arguments and only returns this process's connection id.
    unsafe { (api.main_connection_id)() }
}

fn current_space_of_display(display: &CFDictionary<CFString, CFType>) -> Option<SpaceId> {
    let current = display.get(&CFString::from_static_str("Current Space"))?;
    let current = current.downcast_ref::<CFDictionary>()?;
    // SAFETY: The "Current Space" entry is a CFDictionary with CFString keys.
    let current: &CFDictionary<CFString, CFType> = unsafe { current.cast_unchecked() };
    let id = current.get(&CFString::from_static_str("id64"))?;
    cf::to_i64(&id).map(|v| SpaceId(v as u64))
}

fn parse_space_ids(array: &CFRetained<CFArray>) -> Vec<SpaceId> {
    // SAFETY: The array only holds CFType values, which is the most general element type.
    let array: &CFArray<CFType> = unsafe { array.cast_unchecked() };
    array.iter().filter_map(|v| cf::to_i64(&v)).map(|v| SpaceId(v as u64)).collect()
}

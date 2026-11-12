// SPDX-License-Identifier: GPL-3.0-or-later

use super::model::SpaceId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpaceState {
    pub on_active_space: bool,
    pub orphan: bool,
}

/// `None` for `window_spaces` / `active_spaces` means SkyLight is unavailable; the "is the window on screen?" flag is
/// used instead.
pub fn space_state(
    window_spaces: Option<&[SpaceId]>,
    active_spaces: Option<&[SpaceId]>,
    is_onscreen: bool,
    is_minimized: bool,
    is_app_hidden: bool,
) -> SpaceState {
    let (Some(window), Some(active)) = (window_spaces, active_spaces) else {
        return SpaceState { on_active_space: is_onscreen, orphan: false };
    };
    SpaceState {
        on_active_space: window.iter().any(|s| active.contains(s)),
        orphan: window.is_empty() && !is_minimized && !is_app_hidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: SpaceId = SpaceId(1);
    const B: SpaceId = SpaceId(2);

    #[test]
    fn window_on_active_space() {
        let s = space_state(Some(&[A]), Some(&[A, B]), false, false, false);
        assert!(s.on_active_space && !s.orphan, "a window on the active space is visible there");
    }

    #[test]
    fn window_on_other_space() {
        let s = space_state(Some(&[B]), Some(&[A]), false, false, false);
        assert!(!s.on_active_space && !s.orphan, "a window on another space is neither active nor orphaned");
    }

    #[test]
    fn window_without_space_is_orphan() {
        let s = space_state(Some(&[]), Some(&[A]), false, false, false);
        assert!(s.orphan, "a visible window that belongs to no space is orphaned");
    }

    #[test]
    fn minimized_window_without_space_is_not_orphan() {
        let s = space_state(Some(&[]), Some(&[A]), false, true, false);
        assert!(!s.orphan, "a minimized window has no space but is not orphaned");
    }

    #[test]
    fn falls_back_to_onscreen_without_skylight() {
        let s = space_state(None, None, true, false, false);
        assert!(s.on_active_space && !s.orphan, "without SkyLight an onscreen window counts as on the active space");
    }
}

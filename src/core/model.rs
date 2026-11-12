// SPDX-License-Identifier: GPL-3.0-or-later

use super::geometry::Rect;
use super::window_kind::WindowKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WindowId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpaceId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pid(pub i32);

#[derive(Clone, Debug, PartialEq)]
pub struct WindowInfo {
    pub id: WindowId,
    pub pid: Pid,
    pub app_name: String,
    pub bundle_id: String,
    pub title: String,
    pub frame: Rect,
    pub is_minimized: bool,
    pub is_app_hidden: bool,
    pub is_on_active_space: bool,
    pub kind: WindowKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScreenInfo {
    pub frame: Rect,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub windows: Vec<WindowInfo>,
    pub frontmost_pid: Option<Pid>,
    pub focused_window: Option<WindowId>,
    pub screens: Vec<Rect>,
    pub screen_names: Vec<String>,
}

impl Snapshot {
    pub fn window(&self, id: WindowId) -> Option<&WindowInfo> {
        self.windows.iter().find(|w| w.id == id)
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::model::{Pid, ScreenInfo, WindowId, WindowInfo};
use crate::macos::workspace::AppDesc;

#[derive(Debug)]
pub enum Msg {
    Init { apps: Vec<AppDesc>, frontmost: Option<Pid>, screens: Vec<ScreenInfo> },
    AppLaunched(AppDesc),
    AppTerminated(Pid),
    AppActivated(Pid),
    AppHidden(Pid, bool),
    SpaceChanged,
    ScreensChanged(Vec<ScreenInfo>),
    Touch(WindowId),
    SelfWindow(Option<WindowInfo>),
    Reconcile,
}

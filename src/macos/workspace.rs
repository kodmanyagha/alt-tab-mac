// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2_app_kit::{
    NSApplicationActivationPolicy, NSApplicationDidChangeScreenParametersNotification, NSRunningApplication,
    NSWorkspace, NSWorkspaceActiveSpaceDidChangeNotification, NSWorkspaceApplicationKey,
    NSWorkspaceDidActivateApplicationNotification, NSWorkspaceDidHideApplicationNotification,
    NSWorkspaceDidLaunchApplicationNotification, NSWorkspaceDidTerminateApplicationNotification,
    NSWorkspaceDidUnhideApplicationNotification, NSWorkspaceDidWakeNotification,
};
use objc2_foundation::{NSNotification, NSNotificationCenter, NSNotificationName};

use super::notifications::add_observer;

use crate::core::model::Pid;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AppDesc {
    pub pid: Pid,
    pub name: String,
    pub bundle_id: String,
    pub is_hidden: bool,
}

#[derive(Clone, Debug)]
pub enum WorkspaceEvent {
    Launched(AppDesc),
    Terminated(Pid),
    Activated(Pid),
    Hidden(Pid),
    Unhidden(Pid),
    SpaceChanged,
    ScreensChanged,
    Woke,
}

/// Apps shown in the Dock and menu bar apps.
pub fn running_apps() -> Vec<AppDesc> {
    let apps = NSWorkspace::sharedWorkspace().runningApplications();
    apps.iter().filter_map(|app| describe(&app)).collect()
}

pub fn frontmost_pid() -> Option<Pid> {
    let app = NSWorkspace::sharedWorkspace().frontmostApplication()?;
    Some(Pid(app.processIdentifier()))
}

/// Dock apps and menu bar apps (so that their settings windows appear in the list). Background processes (`Prohibited`)
/// are skipped.
fn describe(app: &NSRunningApplication) -> Option<AppDesc> {
    if app.activationPolicy() == NSApplicationActivationPolicy::Prohibited {
        return None;
    }
    Some(AppDesc {
        pid: Pid(app.processIdentifier()),
        name: app.localizedName().map(|n| n.to_string()).unwrap_or_default(),
        bundle_id: app.bundleIdentifier().map(|b| b.to_string()).unwrap_or_default(),
        is_hidden: app.isHidden(),
    })
}

/// Starts listening to notifications. The observers live as long as the app.
pub fn observe(handler: impl Fn(WorkspaceEvent) + 'static) {
    let handler = Rc::new(handler);
    let center = NSWorkspace::sharedWorkspace().notificationCenter();
    for (name, kind) in workspace_notifications() {
        let handler = handler.clone();
        add_observer(&center, name, move |n| {
            if let Some(event) = to_event(kind, n) {
                handler(event);
            }
        });
    }
    let handler = handler.clone();
    // SAFETY: Notification name constants are immutable statics exported by AppKit.
    let screens = unsafe { NSApplicationDidChangeScreenParametersNotification };
    add_observer(&NSNotificationCenter::defaultCenter(), screens, move |_| handler(WorkspaceEvent::ScreensChanged));
}

#[derive(Clone, Copy)]
enum Kind {
    Launched,
    Terminated,
    Activated,
    Hidden,
    Unhidden,
    SpaceChanged,
    Woke,
}

fn workspace_notifications() -> [(&'static NSNotificationName, Kind); 7] {
    // SAFETY: Notification name constants are immutable statics exported by AppKit.
    unsafe {
        [
            (NSWorkspaceDidLaunchApplicationNotification, Kind::Launched),
            (NSWorkspaceDidTerminateApplicationNotification, Kind::Terminated),
            (NSWorkspaceDidActivateApplicationNotification, Kind::Activated),
            (NSWorkspaceDidHideApplicationNotification, Kind::Hidden),
            (NSWorkspaceDidUnhideApplicationNotification, Kind::Unhidden),
            (NSWorkspaceActiveSpaceDidChangeNotification, Kind::SpaceChanged),
            (NSWorkspaceDidWakeNotification, Kind::Woke),
        ]
    }
}

fn to_event(kind: Kind, notification: &NSNotification) -> Option<WorkspaceEvent> {
    match kind {
        Kind::SpaceChanged => return Some(WorkspaceEvent::SpaceChanged),
        Kind::Woke => return Some(WorkspaceEvent::Woke),
        _ => {}
    }
    let app = app_of(notification)?;
    let pid = Pid(app.processIdentifier());
    Some(match kind {
        Kind::Launched => WorkspaceEvent::Launched(describe(&app)?),
        Kind::Terminated => WorkspaceEvent::Terminated(pid),
        Kind::Activated => WorkspaceEvent::Activated(pid),
        Kind::Hidden => WorkspaceEvent::Hidden(pid),
        Kind::Unhidden => WorkspaceEvent::Unhidden(pid),
        Kind::SpaceChanged | Kind::Woke => unreachable!(),
    })
}

fn app_of(notification: &NSNotification) -> Option<Retained<NSRunningApplication>> {
    let info = notification.userInfo()?;
    // SAFETY: Notification user info key constants are immutable statics exported by AppKit.
    let value = info.objectForKey(unsafe { NSWorkspaceApplicationKey })?;
    value.downcast::<NSRunningApplication>().ok()
}

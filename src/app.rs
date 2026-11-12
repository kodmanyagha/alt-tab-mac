// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchTime};
use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

use crate::config::{self, Config};
use crate::macos::workspace::{self, WorkspaceEvent};
use crate::macos::{capture, event_tap, permissions, screens, shutdown};
use crate::tracker::messages::Msg;
use crate::tracker::store::{FocusRegistry, SnapshotStore};
use crate::tracker::{self, TrackerHandle};
use crate::ui::{app_delegate, status_item};
use crate::{activator, controller, thumbnails};

const PERMISSION_POLL: Duration = Duration::from_secs(1);

pub fn run() {
    let mtm = MainThreadMarker::new().expect("must run on the main thread");
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    shutdown::install_restore_hooks();
    let config = config::load_or_create();
    crate::i18n::apply(config.general.language);
    crate::logging::set_level(config.general.log_level);
    status_item::install(mtm);
    app_delegate::install(mtm);
    start_when_trusted(config);
    app.run();
}

/// Waits until Accessibility permission is granted, then starts the services.
fn start_when_trusted(config: Config) {
    if permissions::is_trusted() {
        start_services(config);
        return;
    }
    tracing::warn!("waiting for Accessibility permission");
    permissions::request();
    poll_permission(config);
}

fn poll_permission(config: Config) {
    let when = DispatchTime::try_from(PERMISSION_POLL).unwrap_or(DispatchTime::NOW);
    let _ = DispatchQueue::main().after(when, move || {
        if permissions::is_trusted() {
            start_services(config);
        } else {
            poll_permission(config);
        }
    });
}

fn start_services(config: Config) {
    let Some(mtm) = MainThreadMarker::new() else {
        tracing::error!("services must be started on the main thread");
        return;
    };
    let store = SnapshotStore::default();
    let registry = FocusRegistry::default();
    let tracker = tracker::spawn(store.clone(), registry.clone(), Box::new(notify_main_of_publish));
    let activator = activator::spawn(registry, store.clone());
    let refresh = Duration::from_millis(config.general.refresh_interval_ms);
    let services = controller::Services { store, tracker, activator, thumbnailer: thumbnails::spawn() };
    controller::install(mtm, config, services);
    ensure_screen_recording_permission();
    controller::start_periodic_refresh();
    forward_workspace_events(mtm, tracker);
    send_initial_state(mtm, tracker);
    tracker::start_refresh_timer(tracker, refresh);
    install_event_tap();
}

/// Asks for Screen Recording permission once if any shortcut uses previews. Without it, the app icon is shown in the
/// preview area.
fn ensure_screen_recording_permission() {
    if controller::previews_enabled() && !capture::has_permission() {
        tracing::warn!("no Screen Recording permission, requesting it for previews");
        capture::request_permission();
    }
}

fn notify_main_of_publish() {
    DispatchQueue::main().exec_async(controller::on_snapshot_published);
}

fn install_event_tap() {
    if event_tap::install(Box::new(controller::on_key)) {
        controller::take_over_system_shortcuts();
        controller::start_health_checks();
        tracing::info!("ready");
    } else {
        tracing::error!("could not create event tap, Accessibility permission missing?");
    }
}

fn send_initial_state(mtm: MainThreadMarker, tracker: TrackerHandle) {
    tracker.send(Msg::Init {
        apps: workspace::running_apps(),
        frontmost: workspace::frontmost_pid(),
        screens: screens::screen_infos(mtm),
    });
}

fn forward_workspace_events(mtm: MainThreadMarker, tracker: TrackerHandle) {
    workspace::observe(move |event| tracker.send(to_message(event, mtm)));
}

fn to_message(event: WorkspaceEvent, mtm: MainThreadMarker) -> Msg {
    match event {
        WorkspaceEvent::Launched(desc) => Msg::AppLaunched(desc),
        WorkspaceEvent::Terminated(pid) => Msg::AppTerminated(pid),
        WorkspaceEvent::Activated(pid) => Msg::AppActivated(pid),
        WorkspaceEvent::Hidden(pid) => Msg::AppHidden(pid, true),
        WorkspaceEvent::Unhidden(pid) => Msg::AppHidden(pid, false),
        WorkspaceEvent::SpaceChanged => Msg::SpaceChanged,
        WorkspaceEvent::Woke => Msg::Reconcile,
        WorkspaceEvent::ScreensChanged => Msg::ScreensChanged(screens::screen_infos(mtm)),
    }
}

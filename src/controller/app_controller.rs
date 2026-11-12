// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::sync::mpsc::Sender;

use objc2::MainThreadMarker;

use super::bindings::{self, Bindings};
use crate::activator::Command;
use crate::config::Config;
use crate::core::geometry::Rect;
use crate::core::input::KeyRouter;
use crate::core::model::{WindowId, WindowInfo};
use crate::core::style::Style;
use crate::core::switcher::Switcher;
use crate::thumbnails::ThumbnailSender;
use crate::tracker::TrackerHandle;
use crate::tracker::store::SnapshotStore;
use crate::ui::Look;
use crate::ui::icons::IconCache;
use crate::ui::panel_set::PanelSet;
use crate::ui::thumbnails::ThumbnailCache;

pub struct Services {
    pub store: SnapshotStore,
    pub tracker: TrackerHandle,
    pub activator: Sender<Command>,
    pub thumbnailer: ThumbnailSender,
}

pub struct AppController {
    pub(super) config: Config,
    pub(super) bindings: Bindings,
    pub(super) router: KeyRouter,
    pub(super) switcher: Switcher,
    pub(super) session_windows: Vec<WindowInfo>,
    pub(super) session_token: u64,
    pub(super) session_screens: Vec<Rect>,
    pub(super) session_look: Look,
    pub(super) panel: PanelSet,
    pub(super) icons: IconCache,
    pub(super) thumbnails: ThumbnailCache,
    pub(super) last_focused: Option<WindowId>,
    pub(super) store: SnapshotStore,
    pub(super) tracker: TrackerHandle,
    pub(super) activator: Sender<Command>,
    pub(super) thumbnailer: ThumbnailSender,
    pub(super) owns_system_shortcuts: bool,
    pub(super) takeover_blocked: bool,
}

thread_local! {
    static CONTROLLER: RefCell<Option<AppController>> = const { RefCell::new(None) };
}

/// Access to the controller on the main thread.
pub fn with_controller<R>(f: impl FnOnce(&mut AppController) -> R) -> Option<R> {
    CONTROLLER.with_borrow_mut(|c| c.as_mut().map(f))
}

pub fn install(mtm: MainThreadMarker, config: Config, services: Services) {
    let controller = AppController {
        bindings: bindings::build(&config.shortcuts),
        config,
        router: KeyRouter::default(),
        switcher: Switcher::default(),
        session_windows: Vec::new(),
        session_token: 0,
        session_screens: Vec::new(),
        session_look: Look::new(Style::default(), Default::default()),
        panel: PanelSet::new(mtm),
        icons: IconCache::default(),
        thumbnails: ThumbnailCache::default(),
        last_focused: None,
        store: services.store,
        tracker: services.tracker,
        activator: services.activator,
        thumbnailer: services.thumbnailer,
        owns_system_shortcuts: false,
        takeover_blocked: false,
    };
    CONTROLLER.set(Some(controller));
    with_controller(|c| c.apply_config());
}

/// Whether any shortcut uses previews (to ask for Screen Recording permission).
pub fn previews_enabled() -> bool {
    with_controller(|c| c.previews_enabled()).unwrap_or(false)
}

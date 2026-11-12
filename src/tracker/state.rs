// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::{HashMap, HashSet};

use crate::core::geometry::Rect;
use crate::core::model::{Pid, SpaceId, WindowId, WindowInfo};
use crate::core::mru::MruList;
use crate::macos::ax::{AxElement, AxObserver};
use crate::macos::window_list::CgWindow;

use super::store::{FocusRegistry, SnapshotStore};

pub struct TrackedApp {
    pub name: String,
    pub bundle_id: String,
    pub is_hidden: bool,
    pub element: AxElement,
    pub observer: Option<AxObserver>,
    pub observe_attempts: u8,
}

pub struct TrackedWindow {
    pub info: WindowInfo,
    pub element: AxElement,
    pub is_orphan: bool,
}

pub type PublishHook = Box<dyn Fn() + Send>;

pub struct Tracker {
    pub apps: HashMap<Pid, TrackedApp>,
    pub windows: HashMap<WindowId, TrackedWindow>,
    pub mru: MruList,
    pub frontmost: Option<Pid>,
    pub focused: Option<WindowId>,
    pub screens: Vec<Rect>,
    pub screen_names: Vec<String>,
    pub active_spaces: Option<Vec<SpaceId>>,
    pub self_window: Option<WindowInfo>,
    pub cg: HashMap<WindowId, CgWindow>,
    pub ignored: HashSet<WindowId>,
    pub reconcile_scheduled: bool,
    pub store: SnapshotStore,
    pub registry: FocusRegistry,
    pub on_publish: PublishHook,
}

impl Tracker {
    pub fn new(store: SnapshotStore, registry: FocusRegistry, on_publish: PublishHook) -> Self {
        Self {
            apps: HashMap::new(),
            windows: HashMap::new(),
            mru: MruList::default(),
            frontmost: None,
            focused: None,
            screens: Vec::new(),
            screen_names: Vec::new(),
            active_spaces: None,
            self_window: None,
            cg: HashMap::new(),
            ignored: HashSet::new(),
            reconcile_scheduled: false,
            store,
            registry,
            on_publish,
        }
    }

    pub fn window_by_element(&self, pid: Pid, element: &AxElement) -> Option<WindowId> {
        self.windows.iter().find(|(_, w)| w.info.pid == pid && &w.element == element).map(|(&id, _)| id)
    }
}

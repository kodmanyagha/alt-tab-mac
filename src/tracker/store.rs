// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::core::model::{Pid, Snapshot, WindowId};
use crate::macos::ax::AxElement;

#[derive(Clone, Default)]
pub struct SnapshotStore(Arc<Mutex<Arc<Snapshot>>>);

impl SnapshotStore {
    pub fn current(&self) -> Arc<Snapshot> {
        self.0.lock().clone()
    }

    pub fn publish(&self, snapshot: Snapshot) {
        *self.0.lock() = Arc::new(snapshot);
    }
}

#[derive(Clone)]
pub struct FocusHandles {
    pub pid: Pid,
    pub window: AxElement,
    pub app: AxElement,
}

#[derive(Clone, Default)]
pub struct FocusRegistry(Arc<Mutex<HashMap<WindowId, FocusHandles>>>);

impl FocusRegistry {
    pub fn get(&self, id: WindowId) -> Option<FocusHandles> {
        self.0.lock().get(&id).cloned()
    }

    pub fn replace(&self, map: HashMap<WindowId, FocusHandles>) {
        *self.0.lock() = map;
    }
}

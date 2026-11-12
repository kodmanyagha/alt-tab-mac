// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::messages::Msg;
use super::state::{PublishHook, Tracker};
use super::store::{FocusRegistry, SnapshotStore};
use crate::macos::run_loop::{RunLoopHandle, run_current};

thread_local! {
    static TRACKER: RefCell<Option<Tracker>> = const { RefCell::new(None) };
}

/// Access to the tracker state on the tracker thread.
pub fn with_tracker(f: impl FnOnce(&mut Tracker)) {
    TRACKER.with_borrow_mut(|tracker| {
        if let Some(tracker) = tracker.as_mut() {
            f(tracker)
        }
    });
}

#[derive(Clone, Copy)]
pub struct TrackerHandle(RunLoopHandle);

impl TrackerHandle {
    pub fn send(&self, msg: Msg) {
        self.0.perform(move || with_tracker(|tracker| tracker.handle(msg)));
    }
}

/// Startup only: starts the tracker thread and waits for its run loop.
pub fn spawn(store: SnapshotStore, registry: FocusRegistry, on_publish: PublishHook) -> TrackerHandle {
    let (tx, rx) = mpsc::channel();
    thread::Builder::new()
        .name("tracker".into())
        .spawn(move || {
            TRACKER.set(Some(Tracker::new(store, registry, on_publish)));
            if tx.send(RunLoopHandle::current()).is_ok() {
                run_current();
            }
        })
        .expect("failed to start tracker thread");
    TrackerHandle(rx.recv().expect("failed to get tracker run loop"))
}

/// Startup only: requests a full reconciliation at regular intervals.
pub fn start_refresh_timer(handle: TrackerHandle, interval: Duration) {
    thread::Builder::new()
        .name("refresh-timer".into())
        .spawn(move || {
            loop {
                thread::sleep(interval);
                handle.send(Msg::Reconcile);
            }
        })
        .expect("failed to start timer thread");
}

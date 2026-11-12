// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::{self, Sender};

use dispatch2::DispatchQueue;

use crate::core::actions::WindowAction;
use crate::core::model::WindowId;
use crate::macos::focus::{FocusTarget, focus};
use crate::macos::window_actions;
use crate::tracker::store::{FocusHandles, FocusRegistry, SnapshotStore};
use crate::ui::settings::own_window;
use std::thread;

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Focus(WindowId),
    Act(WindowId, WindowAction),
}

pub fn spawn(registry: FocusRegistry, store: SnapshotStore) -> Sender<Command> {
    let (tx, rx) = mpsc::channel::<Command>();
    thread::Builder::new()
        .name("activator".into())
        .spawn(move || {
            for command in rx {
                execute(command, &registry, &store);
            }
        })
        .expect("failed to start activator thread");
    tx
}

fn execute(command: Command, registry: &FocusRegistry, store: &SnapshotStore) {
    if targets_own_window(command, store) {
        DispatchQueue::main().exec_async(move || own_window::on_command(command));
        return;
    }
    match command {
        Command::Focus(id) => activate(id, registry, store),
        Command::Act(id, action) => act(id, action, registry),
    }
}

fn targets_own_window(command: Command, store: &SnapshotStore) -> bool {
    let (Command::Focus(id) | Command::Act(id, _)) = command;
    store.current().window(id).is_some_and(|w| w.pid.0 as u32 == std::process::id())
}

fn act(id: WindowId, action: WindowAction, registry: &FocusRegistry) {
    if let Some(FocusHandles { pid, window, app }) = registry.get(id) {
        window_actions::perform(action, pid, &window, &app);
    }
}

fn activate(id: WindowId, registry: &FocusRegistry, store: &SnapshotStore) {
    let Some(target) = focus_target(id, registry, store) else {
        tracing::warn!(window = ?id, "window to switch to not found");
        return;
    };
    focus(&target);
}

fn focus_target(id: WindowId, registry: &FocusRegistry, store: &SnapshotStore) -> Option<FocusTarget> {
    let handles = registry.get(id)?;
    let snapshot = store.current();
    let info = snapshot.window(id)?;
    Some(FocusTarget {
        pid: handles.pid,
        window_id: id,
        window: handles.window,
        app: handles.app,
        is_minimized: info.is_minimized,
        is_app_hidden: info.is_app_hidden,
    })
}

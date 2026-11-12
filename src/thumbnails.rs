// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::mpsc::{self, Receiver, Sender};

use dispatch2::DispatchQueue;

use crate::core::model::WindowId;
use crate::macos::capture::{self, CaptureTarget};
use crate::macos::image::CgImage;
use std::thread;

pub type ThumbnailSender = Sender<Vec<CaptureTarget>>;

pub fn spawn() -> ThumbnailSender {
    let (tx, rx) = mpsc::channel::<Vec<CaptureTarget>>();
    thread::Builder::new().name("thumbnails".into()).spawn(move || run(rx)).expect("failed to start thumbnail thread");
    tx
}

fn run(rx: Receiver<Vec<CaptureTarget>>) {
    while let Ok(first) = rx.recv() {
        let batch = merge_pending(first, &rx);
        if !capture::has_permission() {
            continue;
        }
        for (id, image) in capture::capture_all(&batch) {
            deliver(id, image);
        }
    }
}

/// Also takes the queued requests and captures each window once.
fn merge_pending(mut batch: Vec<CaptureTarget>, rx: &Receiver<Vec<CaptureTarget>>) -> Vec<CaptureTarget> {
    while let Ok(more) = rx.try_recv() {
        batch.extend(more);
    }
    let mut seen: Vec<WindowId> = Vec::new();
    batch.retain(|t| {
        let first = !seen.contains(&t.id);
        seen.push(t.id);
        first
    });
    batch
}

fn deliver(id: WindowId, image: CgImage) {
    DispatchQueue::main().exec_async(move || crate::controller::on_thumbnail(id, image));
}

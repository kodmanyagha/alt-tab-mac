// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchTime};

use super::app_controller::{AppController, with_controller};
use crate::core::model::{WindowId, WindowInfo};
use crate::macos::capture::CaptureTarget;
use crate::macos::image::CgImage;

const PERIODIC_REFRESH: Duration = Duration::from_secs(10);

impl AppController {
    pub(super) fn previews_enabled(&self) -> bool {
        self.bindings.styles.iter().any(|s| s.uses_previews())
    }

    /// Requests previews for the given windows that can be captured.
    pub(super) fn request_thumbnails<'a>(&self, windows: impl Iterator<Item = &'a WindowInfo>) {
        if !self.previews_enabled() {
            return;
        }
        let targets: Vec<CaptureTarget> = windows.filter(|w| is_capturable(w)).map(target).collect();
        if !targets.is_empty() {
            let _ = self.thumbnailer.send(targets);
        }
    }

    /// If the focus changed, requests a preview of the window that was left.
    pub(super) fn capture_previous_focus(&mut self, focused: Option<WindowId>) {
        if focused == self.last_focused {
            return;
        }
        let previous = std::mem::replace(&mut self.last_focused, focused);
        let snapshot = self.store.current();
        self.request_thumbnails(previous.and_then(|id| snapshot.window(id)).into_iter());
    }

    fn refresh_visible_thumbnails(&self) {
        if !self.panel.is_visible() {
            let snapshot = self.store.current();
            self.request_thumbnails(snapshot.windows.iter());
        }
    }
}

/// Windows on other desktops, minimized windows and windows of hidden apps cannot be captured.
fn is_capturable(w: &WindowInfo) -> bool {
    w.is_on_active_space && !w.is_minimized && !w.is_app_hidden
}

fn target(w: &WindowInfo) -> CaptureTarget {
    CaptureTarget { id: w.id, frame: w.frame }
}

/// Called from the thumbnail thread when an image is ready.
pub fn on_thumbnail(id: WindowId, image: CgImage) {
    with_controller(|c| {
        let ns_image = c.thumbnails.insert(id, &image);
        let index = c.session_windows.iter().position(|w| w.id == id);
        if let (Some(index), true) = (index, c.panel.is_visible()) {
            c.panel.update_thumbnail(index, ns_image);
        }
    });
}

/// Refreshes previews of visible windows while the panel is closed.
pub fn start_periodic_refresh() {
    let Ok(when) = DispatchTime::try_from(PERIODIC_REFRESH) else {
        return;
    };
    let _ = DispatchQueue::main().after(when, || {
        with_controller(|c| c.refresh_visible_thumbnails());
        start_periodic_refresh();
    });
}

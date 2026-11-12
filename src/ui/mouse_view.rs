// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{NSAutoresizingMaskOptions, NSEvent, NSTrackingArea, NSTrackingAreaOptions, NSView};
use objc2_foundation::NSRect;

use crate::core::geometry::Point;

define_class!(
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[name = "AltTabMacMouseView"]
    pub struct MouseView;

    impl MouseView {
        /// Handle the first click directly even though the panel is not active.
        #[unsafe(method(acceptsFirstMouse:))]
        fn accepts_first_mouse(&self, _event: Option<&NSEvent>) -> bool {
            true
        }

        #[unsafe(method(mouseMoved:))]
        fn mouse_moved(&self, event: &NSEvent) {
            crate::controller::on_mouse_move(self.local_point(event));
        }

        #[unsafe(method(mouseDown:))]
        fn mouse_down(&self, event: &NSEvent) {
            crate::controller::on_mouse_click(self.local_point(event));
        }
    }
);

impl MouseView {
    pub fn new(mtm: MainThreadMarker, frame: NSRect) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `this` is a freshly allocated `MouseView` and `initWithFrame:` is the designated NSView initializer.
        let view: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: frame] };
        view.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
        );
        view.track_mouse_moves();
        view
    }

    fn track_mouse_moves(&self) {
        use objc2::AllocAnyThread;

        let options = NSTrackingAreaOptions::MouseMoved
            | NSTrackingAreaOptions::ActiveAlways
            | NSTrackingAreaOptions::InVisibleRect;
        // SAFETY: The tracking area is freshly allocated, `self` stays alive while it is installed, and `userInfo` may be nil.
        let area = unsafe {
            NSTrackingArea::initWithRect_options_owner_userInfo(
                NSTrackingArea::alloc(),
                self.bounds(),
                options,
                Some(self as &AnyObject),
                None,
            )
        };
        self.addTrackingArea(&area);
    }

    /// Event location → point relative to the view's top-left corner.
    fn local_point(&self, event: &NSEvent) -> Point {
        let p = self.convertPoint_fromView(event.locationInWindow(), None);
        Point { x: p.x, y: self.bounds().size.height - p.y }
    }
}

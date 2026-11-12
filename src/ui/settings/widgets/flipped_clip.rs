// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::NSClipView;
use objc2_foundation::NSRect;

define_class!(
    #[unsafe(super(NSClipView))]
    #[thread_kind = MainThreadOnly]
    #[name = "AltTabMacFlippedClipView"]
    pub struct FlippedClipView;

    impl FlippedClipView {
        #[unsafe(method(isFlipped))]
        fn is_flipped(&self) -> bool {
            true
        }
    }
);

impl FlippedClipView {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `this` is a freshly allocated `FlippedClipView` and `initWithFrame:` is the designated NSView initializer.
        unsafe { msg_send![super(this), initWithFrame: NSRect::ZERO] }
    }
}

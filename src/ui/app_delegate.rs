// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{NSApplication, NSApplicationDelegate};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AltTabMacAppDelegate"]
    struct AppDelegate;

    // SAFETY: `AppDelegate` is a plain NSObject subclass with no extra ivars.
    unsafe impl NSObjectProtocol for AppDelegate {}

    // SAFETY: The implemented delegate methods match the signatures AppKit expects.
    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationShouldHandleReopen:hasVisibleWindows:))]
        fn should_handle_reopen(&self, _app: &NSApplication, _has_visible_windows: bool) -> bool {
            crate::ui::settings::open(self.mtm());
            true
        }
    }
);

/// Installs the delegate. NSApplication holds its delegate weakly, so the object is leaked on purpose for the app's
/// lifetime.
pub fn install(mtm: MainThreadMarker) {
    let this = AppDelegate::alloc(mtm).set_ivars(());
    // SAFETY: `this` is a freshly allocated `AppDelegate` and `init` is the NSObject designated initializer.
    let delegate: Retained<AppDelegate> = unsafe { msg_send![super(this), init] };
    NSApplication::sharedApplication(mtm).setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
    std::mem::forget(delegate);
}

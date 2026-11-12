// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_app_kit::NSApplicationWillTerminateNotification;
use objc2_foundation::NSNotificationCenter;

use super::notifications::add_observer;
use super::system_shortcuts;
use crate::logging;

const SIGHUP: i32 = 1;
const SIGINT: i32 = 2;
const SIGQUIT: i32 = 3;
const SIGTERM: i32 = 15;

unsafe extern "C" {
    fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    fn _exit(status: i32) -> !;
}

pub fn install_restore_hooks() {
    restore_on_terminate();
    restore_on_signals();
    restore_on_panic();
}

fn restore_on_terminate() {
    // SAFETY: Notification name constants are immutable statics exported by AppKit.
    let name = unsafe { NSApplicationWillTerminateNotification };
    add_observer(&NSNotificationCenter::defaultCenter(), name, |_| {
        system_shortcuts::restore();
        logging::flush();
    });
}

fn restore_on_signals() {
    for sig in [SIGHUP, SIGINT, SIGQUIT, SIGTERM] {
        // SAFETY: `on_signal` is an `extern "C" fn(i32)`, the handler type `signal` expects.
        unsafe { signal(sig, on_signal) };
    }
}

extern "C" fn on_signal(sig: i32) {
    system_shortcuts::restore();
    logging::flush();
    // SAFETY: `_exit` is async-signal-safe and never returns.
    unsafe { _exit(128 + sig) };
}

fn restore_on_panic() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        system_shortcuts::restore();
        previous(info);
        logging::flush();
    }));
}

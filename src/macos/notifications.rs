// SPDX-License-Identifier: GPL-3.0-or-later

use std::ptr::NonNull;

use block2::RcBlock;
use objc2_foundation::{NSNotification, NSNotificationCenter, NSNotificationName};

pub fn add_observer(center: &NSNotificationCenter, name: &NSNotificationName, f: impl Fn(&NSNotification) + 'static) {
    // SAFETY: NSNotificationCenter always passes a valid notification to the block.
    let block = RcBlock::new(move |n: NonNull<NSNotification>| f(unsafe { n.as_ref() }));
    // SAFETY: The block is `'static` and the observer token is kept for the app's lifetime.
    let token = unsafe { center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block) };
    std::mem::forget(token);
}

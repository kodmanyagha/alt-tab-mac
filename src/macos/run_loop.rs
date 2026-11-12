// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::ffi::c_void;

use block2::RcBlock;

use super::ffi::cf::{
    CFRunLoopAddTimer, CFRunLoopGetCurrent, CFRunLoopPerformBlock, CFRunLoopRef, CFRunLoopRun, CFRunLoopTimerCreate,
    CFRunLoopWakeUp, kCFRunLoopDefaultMode,
};

#[derive(Clone, Copy)]
pub struct RunLoopHandle(CFRunLoopRef);

// SAFETY: A CFRunLoopRef may be used from any thread for `CFRunLoopPerformBlock` and `CFRunLoopWakeUp`, the only operations exposed.
unsafe impl Send for RunLoopHandle {}
// SAFETY: `RunLoopHandle` exposes no method that mutates through a shared reference without CF's own locking.
unsafe impl Sync for RunLoopHandle {}

impl RunLoopHandle {
    pub fn current() -> Self {
        // SAFETY: `CFRunLoopGetCurrent` has no preconditions and returns the thread's run loop, which lives as long as the thread.
        Self(unsafe { CFRunLoopGetCurrent() })
    }

    /// Runs `work` on the target thread on the next turn of its run loop.
    pub fn perform(&self, work: impl FnOnce() + Send + 'static) {
        let slot = Cell::new(Some(work));
        let block = RcBlock::new(move || {
            if let Some(work) = slot.take() {
                work();
            }
        });
        // SAFETY: The run loop is valid for the life of its thread and CF copies the block before returning.
        unsafe {
            CFRunLoopPerformBlock(self.0, kCFRunLoopDefaultMode, &*block as *const _ as *const c_void);
            CFRunLoopWakeUp(self.0);
        }
    }
}

/// Runs the current thread's run loop forever.
pub fn run_current() {
    add_keep_alive_timer();
    // SAFETY: `CFRunLoopRun` has no preconditions; it runs the current thread's loop.
    unsafe { CFRunLoopRun() };
}

/// A run loop without sources returns immediately. A timer that never fires keeps the loop waiting for `perform` work.
fn add_keep_alive_timer() {
    extern "C" fn never_fires(_timer: *mut c_void, _info: *mut c_void) {}
    const FAR_FUTURE: f64 = 1.0e12;
    // SAFETY: The timer callback has the C signature CF expects and the timer is added to the current thread's run loop.
    unsafe {
        let timer = CFRunLoopTimerCreate(std::ptr::null(), FAR_FUTURE, 0.0, 0, 0, never_fires, std::ptr::null_mut());
        CFRunLoopAddTimer(CFRunLoopGetCurrent(), timer, kCFRunLoopDefaultMode);
    }
}

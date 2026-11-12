// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::rc::Rc;

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::NSControl;
use objc2_foundation::NSObject;

type Handler = Rc<dyn Fn(&NSControl)>;

thread_local! {
    static HANDLERS: RefCell<Vec<Handler>> = const { RefCell::new(Vec::new()) };
    static TARGET: RefCell<Option<Retained<ActionTarget>>> = const { RefCell::new(None) };
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AltTabMacSettingsTarget"]
    struct ActionTarget;

    impl ActionTarget {
        #[unsafe(method(changed:))]
        fn changed(&self, sender: &NSControl) {
            let handler = HANDLERS.with_borrow(|h| h.get(sender.tag() as usize).cloned());
            if let Some(handler) = handler {
                handler(sender);
            }
        }
    }
);

/// `handler` is called every time `control` changes.
pub fn bind(mtm: MainThreadMarker, control: &NSControl, handler: impl Fn(&NSControl) + 'static) {
    let index = HANDLERS.with_borrow_mut(|h| {
        h.push(Rc::new(handler));
        h.len() - 1
    });
    control.setTag(index as isize);
    let target = target(mtm);
    // SAFETY: Controls keep a weak target; `ActionTarget` is stored in a thread local for the app's lifetime and implements `changed:`.
    unsafe {
        control.setTarget(Some(&*target as &AnyObject));
        control.setAction(Some(sel!(changed:)));
    }
}

/// Drops the old bindings before the window content is rebuilt.
pub fn reset() {
    HANDLERS.with_borrow_mut(|h| h.clear());
}

fn target(mtm: MainThreadMarker) -> Retained<ActionTarget> {
    TARGET.with_borrow_mut(|t| {
        t.get_or_insert_with(|| {
            let this = ActionTarget::alloc(mtm).set_ivars(());
            // SAFETY: `this` is a freshly allocated `ActionTarget` and `init` is the NSObject designated initializer.
            unsafe { msg_send![super(this), init] }
        })
        .clone()
    })
}

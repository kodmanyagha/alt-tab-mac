// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{NSMenu, NSMenuItem, NSWorkspace};
use objc2_foundation::{NSObject, NSString, NSURL};

use crate::config;
use crate::i18n::{Key, t};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AltTabMacMenuTarget"]
    pub struct MenuTarget;

    impl MenuTarget {
        #[unsafe(method(openSettings:))]
        fn open_settings(&self, _sender: Option<&AnyObject>) {
            crate::ui::settings::open(self.mtm());
        }

        #[unsafe(method(openConfig:))]
        fn open_config(&self, _sender: Option<&AnyObject>) {
            let path = config::config_path();
            let url = NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()));
            NSWorkspace::sharedWorkspace().openURL(&url);
        }

        #[unsafe(method(reloadConfig:))]
        fn reload_config(&self, _sender: Option<&AnyObject>) {
            crate::controller::reload_config();
        }
    }
);

impl MenuTarget {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(());
        // SAFETY: `this` is a freshly allocated `MenuTarget` and `init` is the NSObject designated initializer.
        unsafe { msg_send![super(this), init] }
    }
}

pub fn build_menu(mtm: MainThreadMarker, target: &MenuTarget) -> Retained<NSMenu> {
    let menu = NSMenu::new(mtm);
    menu.addItem(&menu_item(mtm, t(Key::MenuSettings), ",", Some(sel!(openSettings:)), Some(target)));
    menu.addItem(&NSMenuItem::separatorItem(mtm));
    menu.addItem(&menu_item(mtm, t(Key::MenuOpenConfig), "", Some(sel!(openConfig:)), Some(target)));
    menu.addItem(&menu_item(mtm, t(Key::MenuReloadConfig), "", Some(sel!(reloadConfig:)), Some(target)));
    menu.addItem(&NSMenuItem::separatorItem(mtm));
    menu.addItem(&menu_item(mtm, t(Key::MenuQuit), "q", Some(sel!(terminate:)), None));
    menu
}

fn menu_item(
    mtm: MainThreadMarker,
    title: &str,
    key: &str,
    action: Option<Sel>,
    target: Option<&MenuTarget>,
) -> Retained<NSMenuItem> {
    // SAFETY: The item is freshly allocated on the main thread; a nil action is allowed.
    let item = unsafe {
        NSMenuItem::initWithTitle_action_keyEquivalent(
            NSMenuItem::alloc(mtm),
            &NSString::from_str(title),
            action,
            &NSString::from_str(key),
        )
    };
    if let Some(target) = target {
        // SAFETY: Menu items keep a weak target; `MenuTarget` is stored next to the status item for the app's lifetime.
        unsafe { item.setTarget(Some(target)) };
    }
    item
}

// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSButton, NSColor, NSControlStateValueOff, NSControlStateValueOn, NSFont, NSPopUpButton, NSTextField,
};
use objc2_foundation::{NSRect, NSString};

use crate::ui::settings::binding::bind;

pub fn label(mtm: MainThreadMarker, text: &str) -> Retained<NSTextField> {
    NSTextField::labelWithString(&NSString::from_str(text), mtm)
}

pub fn header(mtm: MainThreadMarker, text: &str) -> Retained<NSTextField> {
    let field = label(mtm, text);
    field.setFont(Some(&NSFont::boldSystemFontOfSize(13.0)));
    field
}

/// Small gray description text (wraps onto multiple lines).
pub fn note(mtm: MainThreadMarker, text: &str) -> Retained<NSTextField> {
    let field = NSTextField::wrappingLabelWithString(&NSString::from_str(text), mtm);
    field.setFont(Some(&NSFont::systemFontOfSize(11.0)));
    field.setTextColor(Some(&NSColor::secondaryLabelColor()));
    field
}

pub fn checkbox(
    mtm: MainThreadMarker,
    title: &str,
    checked: bool,
    on_change: impl Fn(bool) + 'static,
) -> Retained<NSButton> {
    // SAFETY: A nil target and action are allowed; the binding registry sets them afterwards.
    let button = unsafe { NSButton::checkboxWithTitle_target_action(&NSString::from_str(title), None, None, mtm) };
    button.setState(if checked { NSControlStateValueOn } else { NSControlStateValueOff });
    bind(mtm, &button, move |control| {
        // SAFETY: This handler is only bound to the checkbox created above, which is an `NSButton`.
        let button: &NSButton = unsafe { &*(control as *const _ as *const NSButton) };
        on_change(button.state() == NSControlStateValueOn);
    });
    button
}

pub fn popup(
    mtm: MainThreadMarker,
    options: &[String],
    selected: usize,
    on_change: impl Fn(usize) + 'static,
) -> Retained<NSPopUpButton> {
    use objc2::MainThreadOnly;

    let popup = NSPopUpButton::initWithFrame_pullsDown(NSPopUpButton::alloc(mtm), NSRect::ZERO, false);
    for option in options {
        popup.addItemWithTitle(&NSString::from_str(option));
    }
    popup.selectItemAtIndex(selected as isize);
    bind(mtm, &popup, move |control| {
        // SAFETY: This handler is only bound to the pop-up created above, which is an `NSPopUpButton`.
        let popup: &NSPopUpButton = unsafe { &*(control as *const _ as *const NSPopUpButton) };
        on_change(popup.indexOfSelectedItem().max(0) as usize);
    });
    popup
}

pub fn button(mtm: MainThreadMarker, title: &str, on_click: impl Fn() + 'static) -> Retained<NSButton> {
    button_with_sender(mtm, title, move |_| on_click())
}

/// For cases that need the clicked button itself (shortcut recorder).
pub fn button_with_sender(
    mtm: MainThreadMarker,
    title: &str,
    on_click: impl Fn(&NSButton) + 'static,
) -> Retained<NSButton> {
    // SAFETY: A nil target and action are allowed; the binding registry sets them afterwards.
    let button = unsafe { NSButton::buttonWithTitle_target_action(&NSString::from_str(title), None, None, mtm) };
    // SAFETY: This handler is only bound to the button created above, which is an `NSButton`.
    bind(mtm, &button, move |control| on_click(unsafe { &*(control as *const _ as *const NSButton) }));
    button
}

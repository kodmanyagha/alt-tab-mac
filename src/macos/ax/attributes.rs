// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_core_foundation::{CFArray, CFBoolean, CFType};

use super::element::AxElement;
use crate::macos::cf;
use crate::macos::ffi::ax::{_AXUIElementGetWindow, AX_SUCCESS};

const TITLE: &str = "AXTitle";
const ROLE: &str = "AXRole";
const SUBROLE: &str = "AXSubrole";
const MODAL: &str = "AXModal";
const MINIMIZED: &str = "AXMinimized";
const HIDDEN: &str = "AXHidden";
const WINDOWS: &str = "AXWindows";
const FOCUSED_WINDOW: &str = "AXFocusedWindow";
const RAISE: &str = "AXRaise";
const PRESS: &str = "AXPress";
const CLOSE_BUTTON: &str = "AXCloseButton";

impl AxElement {
    pub fn title(&self) -> Option<String> {
        self.attribute(TITLE).and_then(|v| cf::to_string(&v))
    }

    pub fn is_minimized(&self) -> bool {
        self.attribute(MINIMIZED).and_then(|v| cf::to_bool(&v)).unwrap_or(false)
    }

    pub fn set_minimized(&self, minimized: bool) -> bool {
        self.set_attribute(MINIMIZED, CFBoolean::new(minimized))
    }

    /// On an application element: hides or shows the app.
    pub fn set_hidden(&self, hidden: bool) -> bool {
        self.set_attribute(HIDDEN, CFBoolean::new(hidden))
    }

    pub fn role(&self) -> Option<String> {
        self.attribute(ROLE).and_then(|v| cf::to_string(&v))
    }

    pub fn subrole(&self) -> Option<String> {
        self.attribute(SUBROLE).and_then(|v| cf::to_string(&v))
    }

    /// Whether the window blocks interaction with its main window while open.
    pub fn is_modal(&self) -> bool {
        self.attribute(MODAL).and_then(|v| cf::to_bool(&v)).unwrap_or(false)
    }

    /// On an application element: all of its windows.
    pub fn windows(&self) -> Vec<AxElement> {
        let Some(value) = self.attribute(WINDOWS) else {
            return Vec::new();
        };
        let Some(array) = value.downcast_ref::<CFArray>() else {
            return Vec::new();
        };
        // SAFETY: The array only holds CFType values, which is the most general element type.
        let array: &CFArray<CFType> = unsafe { array.cast_unchecked() };
        array.iter().filter_map(|item| AxElement::from_borrowed(&*item as *const CFType as _)).collect()
    }

    /// On an application element: the focused window.
    pub fn focused_window(&self) -> Option<AxElement> {
        let value = self.attribute(FOCUSED_WINDOW)?;
        AxElement::from_borrowed(&*value as *const CFType as _)
    }

    pub fn window_id(&self) -> Option<u32> {
        let mut id = 0u32;
        // SAFETY: `self` is a valid window element and `id` is a live local to write into.
        let err = unsafe { _AXUIElementGetWindow(self.as_ptr(), &mut id) };
        (err == AX_SUCCESS && id != 0).then_some(id)
    }

    pub fn raise(&self) -> bool {
        self.perform(RAISE)
    }

    /// Presses the window's close button (the app may ask whether to save).
    pub fn close(&self) -> bool {
        let Some(button) = self.attribute(CLOSE_BUTTON) else {
            return false;
        };
        AxElement::from_borrowed(&*button as *const CFType as _).is_some_and(|b| b.perform(PRESS))
    }
}

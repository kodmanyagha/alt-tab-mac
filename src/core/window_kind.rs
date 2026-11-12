// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowKind {
    Standard,
    Dialog,
    ModalDialog,
    FloatingPanel,
    Other,
}

/// Window kind from the AX subrole and modal flag.
pub fn classify(subrole: Option<&str>, is_modal: bool) -> WindowKind {
    match (subrole, is_modal) {
        (Some("AXStandardWindow") | Some("AXDialog"), true) => WindowKind::ModalDialog,
        (Some("AXStandardWindow"), false) => WindowKind::Standard,
        (Some("AXDialog") | Some("AXSystemDialog"), false) => WindowKind::Dialog,
        (Some("AXFloatingWindow") | Some("AXSystemFloatingWindow"), _) => WindowKind::FloatingPanel,
        _ => WindowKind::Other,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WindowKinds {
    pub dialogs: bool,
    pub modal_dialogs: bool,
    pub floating_panels: bool,
}

impl Default for WindowKinds {
    fn default() -> Self {
        Self { dialogs: true, modal_dialogs: false, floating_panels: false }
    }
}

impl WindowKinds {
    pub fn allows(&self, kind: WindowKind) -> bool {
        match kind {
            WindowKind::Standard => true,
            WindowKind::Dialog => self.dialogs,
            WindowKind::ModalDialog => self.modal_dialogs,
            WindowKind::FloatingPanel => self.floating_panels,
            WindowKind::Other => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification() {
        assert_eq!(classify(Some("AXStandardWindow"), false), WindowKind::Standard);
        assert_eq!(classify(Some("AXDialog"), false), WindowKind::Dialog);
        assert_eq!(classify(Some("AXDialog"), true), WindowKind::ModalDialog);
        assert_eq!(classify(Some("AXFloatingWindow"), false), WindowKind::FloatingPanel);
        assert_eq!(classify(Some("AXUnknown"), false), WindowKind::Other);
        assert_eq!(classify(None, false), WindowKind::Other);
    }

    #[test]
    fn defaults_hide_panels_and_modal_dialogs() {
        let kinds = WindowKinds::default();
        assert!(kinds.allows(WindowKind::Standard), "standard windows are allowed");
        assert!(kinds.allows(WindowKind::Dialog), "dialogs are allowed");
        assert!(!kinds.allows(WindowKind::ModalDialog), "modal dialogs are filtered out");
        assert!(!kinds.allows(WindowKind::FloatingPanel), "floating panels are filtered out");
        assert!(!kinds.allows(WindowKind::Other), "other windows are filtered out");
    }
}

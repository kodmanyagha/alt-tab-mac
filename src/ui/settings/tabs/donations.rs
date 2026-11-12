// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSButton, NSFont, NSFontWeightRegular, NSLayoutConstraintOrientation, NSLineBreakMode, NSPasteboard,
    NSPasteboardTypeString, NSTextField, NSView,
};
use objc2_foundation::NSString;

use crate::i18n::{Key, t};
use crate::ui::settings::widgets::{button_with_sender, field, fieldset, hstack, note, two_columns};

enum Name {
    Raw(&'static str),
    Translated(Key),
}

const DONATIONS: [(Name, &str); 6] = [
    (Name::Raw("Bitcoin (BTC)"), "bc1qnac4z5363m5ke937gqlkn69jfhcqqz9uv85drv"),
    (Name::Raw("Ethereum (ETH, USDT...)"), "0xa3Ecccc8A958309519659f0d317A0B5eEEd43a26"),
    (Name::Raw("Solana (SOL)"), "A6Q6aYPihLpvZhBvgXbFhxeXVTVb9T5t5jnciJEJ5RXB"),
    (Name::Raw("TRON (TRX, USDT...)"), "TXt2esaLDc7rebiPCC2SUPaqrbao1NnGab"),
    (Name::Translated(Key::BinanceUserId), "22770019"),
    (Name::Translated(Key::BinanceUsername), "rustdev"),
];

const LOW_HUGGING: f32 = 1.0;

pub fn donations_fieldset(mtm: MainThreadMarker) -> Retained<NSView> {
    let intro = note(mtm, t(Key::DonateNote));
    let rows: Vec<Retained<NSView>> = DONATIONS.iter().map(|(name, value)| donation_row(mtm, name, value)).collect();
    let half = rows.len().div_ceil(2);
    let left: Vec<&NSView> = rows[..half].iter().map(|r| &**r).collect();
    let right: Vec<&NSView> = rows[half..].iter().map(|r| &**r).collect();
    let columns = two_columns(mtm, &left, &right);
    fieldset(mtm, t(Key::Donate), &[&intro, &columns])
}

/// Name on top; address and "Copy" button below.
fn donation_row(mtm: MainThreadMarker, name: &Name, value: &'static str) -> Retained<NSView> {
    let title = match name {
        Name::Raw(text) => text,
        Name::Translated(key) => t(*key),
    };
    let address = address_label(mtm, value);
    let copy = button_with_sender(mtm, t(Key::Copy), move |b| copy_to_clipboard(value, b));
    field(mtm, title, &hstack(mtm, &[&address, &copy]))
}

/// Selectable, in a monospaced font; long addresses wrap.
fn address_label(mtm: MainThreadMarker, value: &str) -> Retained<NSTextField> {
    let label = NSTextField::wrappingLabelWithString(&NSString::from_str(value), mtm);
    label.setSelectable(true);
    label.setLineBreakMode(NSLineBreakMode::ByCharWrapping);
    // SAFETY: Font weight constants are immutable statics exported by AppKit.
    label.setFont(Some(&NSFont::monospacedSystemFontOfSize_weight(11.0, unsafe { NSFontWeightRegular })));
    label.setContentHuggingPriority_forOrientation(LOW_HUGGING, NSLayoutConstraintOrientation::Horizontal);
    label
}

fn copy_to_clipboard(value: &str, button: &NSButton) {
    let pasteboard = NSPasteboard::generalPasteboard();
    pasteboard.clearContents();
    // SAFETY: Pasteboard type constants are immutable statics exported by AppKit.
    pasteboard.setString_forType(&NSString::from_str(value), unsafe { NSPasteboardTypeString });
    button.setTitle(&NSString::from_str(t(Key::Copied)));
}

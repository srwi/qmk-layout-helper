use crate::keycode_labels::advanced::get_advanced_keycode_label;
use crate::keycode_labels::basic::get_basic_keycode_label;
use crate::keycode_labels::layer::get_layer_keycode_label;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeycodeKind {
    Basic,
    Modifier,
    Special,
}

pub struct KeycodeLabel {
    pub long: Option<String>,
    pub short: Option<String>,
    pub symbol: Option<String>,
    pub kind: KeycodeKind,
    pub layer_ref: Option<u8>,
}

impl Default for KeycodeLabel {
    fn default() -> Self {
        KeycodeLabel {
            long: None,
            short: None,
            symbol: None,
            kind: KeycodeKind::Basic,
            layer_ref: None,
        }
    }
}

pub fn get_keycode_label(bytes: u16) -> KeycodeLabel {
    get_basic_keycode_label(bytes)
        .or_else(|| get_layer_keycode_label(bytes))
        .or_else(|| get_advanced_keycode_label(bytes))
        .unwrap_or_else(|| get_hex_keycode_label(bytes))
}

fn get_hex_keycode_label(keycode_bytes: u16) -> KeycodeLabel {
    KeycodeLabel {
        long: Some(format!("0x{:04X}", keycode_bytes)),
        ..Default::default()
    }
}

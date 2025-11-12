use eframe::egui::{self};
use std::ops::Range;

use crate::basic_keycode_label::get_basic_keycode_label;

// The constants may be different for protocol versions other than 12:
const QK_MODS: Range<u16> = 0x0100..0x2000;
const QK_MOD_TAP: Range<u16> = 0x2000..0x4000;
const QK_LAYER_TAP: Range<u16> = 0x4000..0x5000;
const QK_LAYER_MOD: Range<u16> = 0x5000..0x5200;
const QK_TO: Range<u16> = 0x5200..0x5220;
const QK_MOMENTARY: Range<u16> = 0x5220..0x5240;
const QK_DEF_LAYER: Range<u16> = 0x5240..0x5260;
const QK_TOGGLE_LAYER: Range<u16> = 0x5260..0x5280;
const QK_ONE_SHOT_LAYER: Range<u16> = 0x5280..0x52A0;
const QK_ONE_SHOT_MOD: Range<u16> = 0x52a0..0x52c0;
const QK_LAYER_TAP_TOGGLE: Range<u16> = 0x52C0..0x52E0;
const QK_MACRO: Range<u16> = 0x7700..0x7780;
const QK_KB: Range<u16> = 0x7E00..0x7F00;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum KeycodeKind {
    Basic,
    Modifier,
    Special,
}

pub struct KeycodeLabel {
    pub long: Option<String>,
    pub short: Option<String>,
    pub kind: KeycodeKind,
    pub layer_ref: Option<u8>,
}

impl Default for KeycodeLabel {
    fn default() -> Self {
        KeycodeLabel {
            long: Some("n/a".to_string()),
            short: None,
            kind: KeycodeKind::Basic,
            layer_ref: None,
        }
    }
}

pub fn get_keycode_color(
    layer: u8,
    kind: KeycodeKind,
    desaturate: bool,
) -> (egui::Color32, egui::Color32, egui::Color32) {
    const ALPHA: u8 = 239;
    const DESATURATE_FACTOR: f32 = 0.7;

    const BLACK: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 0, 0, ALPHA);
    const LAYER_0: egui::Color32 = egui::Color32::from_rgba_premultiplied(83, 83, 83, ALPHA);
    const LAYER_1: egui::Color32 = egui::Color32::from_rgba_premultiplied(80, 140, 115, ALPHA);
    const LAYER_2: egui::Color32 = egui::Color32::from_rgba_premultiplied(100, 115, 150, ALPHA);
    const LAYER_3: egui::Color32 = egui::Color32::from_rgba_premultiplied(140, 110, 150, ALPHA);
    const LAYER_4: egui::Color32 = egui::Color32::from_rgba_premultiplied(95, 121, 127, ALPHA);
    const LAYER_5: egui::Color32 = egui::Color32::from_rgba_premultiplied(147, 137, 110, ALPHA);
    const LAYER_N: egui::Color32 = egui::Color32::from_rgba_premultiplied(127, 127, 127, ALPHA);

    let mut background_color = match layer {
        0 => LAYER_0,
        1 => LAYER_1,
        2 => LAYER_2,
        3 => LAYER_3,
        4 => LAYER_4,
        5 => LAYER_5,
        _ => LAYER_N,
    };

    if kind == KeycodeKind::Special {
        background_color = background_color.lerp_to_gamma(BLACK, 0.6);
    } else if kind == KeycodeKind::Modifier {
        background_color = background_color.lerp_to_gamma(BLACK, 0.3);
    }

    let mut border_color = background_color.lerp_to_gamma(BLACK, 0.2);

    // Never desaturate layer 0
    if desaturate && layer != 0 {
        background_color = background_color.lerp_to_gamma(LAYER_0, DESATURATE_FACTOR);
        border_color = border_color.lerp_to_gamma(LAYER_0, DESATURATE_FACTOR);
    }

    let font_color = if desaturate {
        egui::Color32::WHITE.gamma_multiply(1.0 - DESATURATE_FACTOR)
    } else {
        egui::Color32::WHITE
    };

    (background_color, border_color, font_color)
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

fn get_advanced_keycode_label(keycode_bytes: u16) -> Option<KeycodeLabel> {
    match keycode_bytes {
        input_bytes if QK_MODS.contains(&input_bytes) => {
            let keycode = (input_bytes & 0xff) as u16;
            let keycode_str = get_basic_keycode_label(keycode)
                .and_then(|k| k.long)
                .unwrap_or_else(|| format!("0x{:02X}", keycode));

            let input_modifiers = input_bytes & 0x1f00;

            const QK_LCTL: u16 = 0x0100;
            const QK_LSFT: u16 = 0x0200;
            const QK_LALT: u16 = 0x0400;
            const QK_LGUI: u16 = 0x0800;
            const QK_RMODS_MIN: u16 = 0x1000;
            const QK_RCTL: u16 = 0x1100;
            const QK_RSFT: u16 = 0x1200;
            const QK_RALT: u16 = 0x1400;
            const QK_RGUI: u16 = 0x1800;

            let modifier_key_to_value: &[(&str, u16)] = &[
                ("LCTL", QK_LCTL),
                ("C", QK_LCTL),
                ("LSFT", QK_LSFT),
                ("S", QK_LSFT),
                ("LALT", QK_LALT),
                ("A", QK_LALT),
                ("LGUI", QK_LGUI),
                ("LCMD", QK_LGUI),
                ("LWIN", QK_LGUI),
                ("G", QK_LGUI),
                ("RCTL", QK_RCTL),
                ("RSFT", QK_RSFT),
                ("ALGR", QK_RALT),
                ("RALT", QK_RALT),
                ("RCMD", QK_RGUI),
                ("RWIN", QK_RGUI),
                ("RGUI", QK_RGUI),
                ("SCMD", QK_LSFT | QK_LGUI),
                ("SWIN", QK_LSFT | QK_LGUI),
                ("SGUI", QK_LSFT | QK_LGUI),
                ("LSG", QK_LSFT | QK_LGUI),
                ("LAG", QK_LALT | QK_LGUI),
                ("RSG", QK_RSFT | QK_RGUI),
                ("RAG", QK_RALT | QK_RGUI),
                ("LCA", QK_LCTL | QK_LALT),
                ("LSA", QK_LSFT | QK_LALT),
                ("SAGR", QK_RSFT | QK_RALT),
                ("RSA", QK_RSFT | QK_RALT),
                ("RCS", QK_RCTL | QK_RSFT),
                ("LCAG", QK_LCTL | QK_LALT | QK_LGUI),
                ("MEH", QK_LCTL | QK_LALT | QK_LSFT),
                ("HYPR", QK_LCTL | QK_LALT | QK_LSFT | QK_LGUI),
            ];

            // Try to find exact matches first
            if let Some((name, _)) = modifier_key_to_value
                .iter()
                .find(|(_, v)| *v == input_modifiers)
            {
                return Some(KeycodeLabel {
                    long: Some(format!("{}({})", name, keycode_str)),
                    kind: KeycodeKind::Modifier,
                    ..Default::default()
                });
            }

            // Left and right side modifiers are mutually exclusive. Therefore a single boolean
            // is used to indicate which side to use.
            let is_right_side_mods = (input_modifiers & QK_RMODS_MIN) != 0;
            let enabled: Vec<&str> = modifier_key_to_value
                .iter()
                .filter(|(_, modifiers)| {
                    if is_right_side_mods {
                        *modifiers >= QK_RMODS_MIN
                    } else {
                        *modifiers < QK_RMODS_MIN
                    }
                })
                .filter_map(|(modifiers_name, modifiers)| {
                    if (input_modifiers & *modifiers) == *modifiers {
                        Some(*modifiers_name)
                    } else {
                        None
                    }
                })
                .collect();

            if !enabled.is_empty() {
                // Build nested parentheses style, e.g. LCTL(LALT(A))
                let mut nested_mods = String::new();
                for (i, part) in enabled.iter().enumerate() {
                    if i > 0 {
                        nested_mods.push('(');
                    }
                    nested_mods.push_str(part);
                }
                if !nested_mods.is_empty() {
                    nested_mods.push('(');
                }
                nested_mods.push_str(&keycode_str);
                for _ in 0..enabled.len() {
                    nested_mods.push(')');
                }

                return Some(KeycodeLabel {
                    long: Some(nested_mods),
                    kind: KeycodeKind::Modifier,
                    ..Default::default()
                });
            }

            None
        }
        input_bytes if QK_MOD_TAP.contains(&input_bytes) => {
            let remainder = input_bytes & !(QK_MOD_TAP.start);

            let mod_value = (remainder >> 8) & 0x1F;
            let mod_str = mod_value_to_string(mod_value);

            let keycode = (remainder & 0xFF) as u8;
            let keycode_str = get_basic_keycode_label(keycode as u16)
                .and_then(|k| k.long)
                .unwrap_or_else(|| format!("0x{:02X}", keycode));

            Some(KeycodeLabel {
                long: Some(format!("MT({},{})", mod_str, keycode_str)),
                kind: KeycodeKind::Modifier,
                ..Default::default()
            })
        }
        input_bytes if QK_LAYER_MOD.contains(&input_bytes) => {
            let remainder = input_bytes & !(QK_LAYER_MOD.start);
            let mask = 0x1f;
            let shift = 5;

            let layer = remainder >> shift;

            let mod_value = remainder & mask;
            let mod_str = mod_value_to_string(mod_value);

            Some(KeycodeLabel {
                long: Some(format!("LM({},{})", layer, mod_str)),
                kind: KeycodeKind::Modifier,
                layer_ref: Some(layer as u8),
                ..Default::default()
            })
        }
        input_bytes if QK_ONE_SHOT_MOD.contains(&input_bytes) => {
            let remainder = input_bytes & !(QK_ONE_SHOT_MOD.start);

            let mod_str = mod_value_to_string(remainder);

            Some(KeycodeLabel {
                long: Some(format!("OSM({})", mod_str)),
                kind: KeycodeKind::Modifier,
                ..Default::default()
            })
        }
        input_bytes if QK_LAYER_TAP.contains(&input_bytes) => {
            let remainder = input_bytes & !(QK_LAYER_TAP.start);

            let layer = remainder >> 8;

            let keycode = (remainder & 0xFF) as u8;
            let keycode_str = get_basic_keycode_label(keycode as u16)
                .and_then(|k| k.long)
                .unwrap_or_else(|| format!("0x{:02X}", keycode));

            Some(KeycodeLabel {
                long: Some(format!("LT({},{})", layer, keycode_str)),
                kind: KeycodeKind::Modifier,
                layer_ref: Some(layer as u8),
                ..Default::default()
            })
        }
        _ => None,
    }
}

fn mod_value_to_string(mod_mask: u16) -> String {
    const MOD_LCTL: u16 = 0x01;
    const MOD_LSFT: u16 = 0x02;
    const MOD_LALT: u16 = 0x04;
    const MOD_LGUI: u16 = 0x08;
    const MOD_RCTL: u16 = 0x10;
    const MOD_RSFT: u16 = 0x20;
    const MOD_RALT: u16 = 0x40;
    const MOD_RGUI: u16 = 0x80;

    let mut mods = Vec::new();
    if mod_mask & MOD_LCTL != 0 {
        mods.push("MOD_LCTL");
    }
    if mod_mask & MOD_LSFT != 0 {
        mods.push("MOD_LSFT");
    }
    if mod_mask & MOD_LALT != 0 {
        mods.push("MOD_LALT");
    }
    if mod_mask & MOD_LGUI != 0 {
        mods.push("MOD_LGUI");
    }
    if mod_mask & MOD_RCTL != 0 {
        mods.push("MOD_RCTL");
    }
    if mod_mask & MOD_RSFT != 0 {
        mods.push("MOD_RSFT");
    }
    if mod_mask & MOD_RALT != 0 {
        mods.push("MOD_RALT");
    }
    if mod_mask & MOD_RGUI != 0 {
        mods.push("MOD_RGUI");
    }

    if mods.is_empty() {
        "None".to_string()
    } else {
        mods.join(" | ")
    }
}

fn get_layer_keycode_label(keycode_bytes: u16) -> Option<KeycodeLabel> {
    let (long_name, layer_ref) = match keycode_bytes {
        b if QK_TO.contains(&b) => {
            let l = (b - QK_TO.start) as u8;
            (Some(format!("TO({})", l)), Some(l))
        }
        b if QK_MOMENTARY.contains(&b) => {
            let l = (b - QK_MOMENTARY.start) as u8;
            (Some(format!("MO({})", l)), Some(l))
        }
        b if QK_TOGGLE_LAYER.contains(&b) => {
            let l = (b - QK_TOGGLE_LAYER.start) as u8;
            (Some(format!("TG({})", l)), Some(l))
        }
        b if QK_ONE_SHOT_LAYER.contains(&b) => {
            let l = (b - QK_ONE_SHOT_LAYER.start) as u8;
            (Some(format!("OSL({})", l)), Some(l))
        }
        b if QK_LAYER_TAP_TOGGLE.contains(&b) => {
            let l = (b - QK_LAYER_TAP_TOGGLE.start) as u8;
            (Some(format!("TT({})", l)), Some(l))
        }
        b if QK_DEF_LAYER.contains(&b) => {
            let l = (b - QK_DEF_LAYER.start) as u8;
            (Some(format!("DF({})", l)), None)
        }
        b if QK_KB.contains(&b) => {
            let n = b - QK_KB.start;
            (Some(format!("CUSTOM({})", n)), None)
        }
        b if QK_MACRO.contains(&b) => {
            let n = b - QK_MACRO.start;
            (Some(format!("MACRO({})", n)), None)
        }
        _ => return None,
    };

    Some(KeycodeLabel {
        long: long_name,
        layer_ref,
        ..Default::default()
    })
}

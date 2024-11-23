use eframe::egui;
use qmk_via_api::keycodes::Keycode;
use std::ops::Range;

const QK_TO: Range<u16> = 0x5200..0x5220;
const QK_MOMENTARY: Range<u16> = 0x5220..0x5240;
const QK_DEF_LAYER: Range<u16> = 0x5240..0x5260;
const QK_TOGGLE_LAYER: Range<u16> = 0x5260..0x5280;
const QK_ONE_SHOT_LAYER: Range<u16> = 0x5280..0x52A0;
const QK_LAYER_TAP_TOGGLE: Range<u16> = 0x52C0..0x52E0;
const QK_KB: Range<u16> = 0x7E00..0x7E40;
const QK_MACRO: Range<u16> = 0x7700..0x7780;

pub struct KeycodeLabel {
    pub long: Option<String>,
    pub short: Option<String>,
    pub color: egui::Color32,
}

impl Default for KeycodeLabel {
    fn default() -> Self {
        KeycodeLabel {
            long: Some("n/a".to_string()),
            short: None,
            color: egui::Color32::from_rgba_unmultiplied(150, 150, 150, 225),
        }
    }
}

pub fn get_keycode_label(bytes: u16) -> KeycodeLabel {
    get_basic_keycode_label(bytes)
        .or_else(|| get_layer_keycode_label(bytes))
        .unwrap_or_else(KeycodeLabel::default)
}

fn get_layer_keycode_label(keycode_bytes: u16) -> Option<KeycodeLabel> {
    let long_name = match keycode_bytes {
        b if QK_TO.contains(&b) => {
            let layer = b - QK_TO.start;
            format!("TO({})", layer)
        }
        b if QK_MOMENTARY.contains(&b) => {
            let layer = b - QK_MOMENTARY.start;
            format!("MO({})", layer)
        }
        b if QK_DEF_LAYER.contains(&b) => {
            let layer = b - QK_DEF_LAYER.start;
            format!("DF({})", layer)
        }
        b if QK_TOGGLE_LAYER.contains(&b) => {
            let layer = b - QK_TOGGLE_LAYER.start;
            format!("TG({})", layer)
        }
        b if QK_ONE_SHOT_LAYER.contains(&b) => {
            let layer = b - QK_ONE_SHOT_LAYER.start;
            format!("OSL({})", layer)
        }
        b if QK_LAYER_TAP_TOGGLE.contains(&b) => {
            let layer = b - QK_LAYER_TAP_TOGGLE.start;
            format!("TT({})", layer)
        }
        b if QK_KB.contains(&b) => {
            let n = b - QK_KB.start;
            format!("CUSTOM({})", n)
        }
        b if QK_MACRO.contains(&b) => {
            let n = b - QK_MACRO.start;
            format!("MACRO({})", n)
        }
        _ => return None,
    };

    Some(KeycodeLabel {
        long: Some(long_name),
        short: None,
        ..Default::default()
    })
}

fn get_basic_keycode_label(keycode_bytes: u16) -> Option<KeycodeLabel> {
    let keycode = Keycode::try_from(keycode_bytes).ok()?;

    let color_modifier = egui::Color32::from_rgba_premultiplied(105, 105, 105, 225);
    let color_special = egui::Color32::from_rgba_premultiplied(65, 65, 65, 225);

    match keycode {
        Keycode::KC_NO => Some(KeycodeLabel {
            long: None,
            short: None,
            ..Default::default()
        }),
        Keycode::KC_TRANSPARENT => Some(KeycodeLabel {
            long: Some("🔽".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_A => Some(KeycodeLabel {
            long: Some("A".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_B => Some(KeycodeLabel {
            long: Some("B".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_C => Some(KeycodeLabel {
            long: Some("C".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_D => Some(KeycodeLabel {
            long: Some("D".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_E => Some(KeycodeLabel {
            long: Some("E".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F => Some(KeycodeLabel {
            long: Some("F".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_G => Some(KeycodeLabel {
            long: Some("G".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_H => Some(KeycodeLabel {
            long: Some("H".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_I => Some(KeycodeLabel {
            long: Some("I".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_J => Some(KeycodeLabel {
            long: Some("J".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_K => Some(KeycodeLabel {
            long: Some("K".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_L => Some(KeycodeLabel {
            long: Some("L".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_M => Some(KeycodeLabel {
            long: Some("M".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_N => Some(KeycodeLabel {
            long: Some("N".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_O => Some(KeycodeLabel {
            long: Some("O".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_P => Some(KeycodeLabel {
            long: Some("P".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_Q => Some(KeycodeLabel {
            long: Some("Q".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_R => Some(KeycodeLabel {
            long: Some("R".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_S => Some(KeycodeLabel {
            long: Some("S".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_T => Some(KeycodeLabel {
            long: Some("T".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_U => Some(KeycodeLabel {
            long: Some("U".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_V => Some(KeycodeLabel {
            long: Some("V".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_W => Some(KeycodeLabel {
            long: Some("W".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_X => Some(KeycodeLabel {
            long: Some("X".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_Y => Some(KeycodeLabel {
            long: Some("Y".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_Z => Some(KeycodeLabel {
            long: Some("Z".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_1 => Some(KeycodeLabel {
            long: Some("!\n1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_2 => Some(KeycodeLabel {
            long: Some("@\n2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_3 => Some(KeycodeLabel {
            long: Some("#\n3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_4 => Some(KeycodeLabel {
            long: Some("$\n4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_5 => Some(KeycodeLabel {
            long: Some("%\n5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_6 => Some(KeycodeLabel {
            long: Some("^\n6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_7 => Some(KeycodeLabel {
            long: Some("&\n7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_8 => Some(KeycodeLabel {
            long: Some("*\n8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_9 => Some(KeycodeLabel {
            long: Some("(\n9".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_0 => Some(KeycodeLabel {
            long: Some(")\n0".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_ENTER => Some(KeycodeLabel {
            long: Some("Enter".to_string()),
            short: Some("Ent".to_string()),
            color: color_special,
        }),
        Keycode::KC_ESCAPE => Some(KeycodeLabel {
            long: Some("Esc".to_string()),
            short: None,
            color: color_special,
        }),
        Keycode::KC_BACKSPACE => Some(KeycodeLabel {
            long: Some("Backspace".to_string()),
            short: Some("Bksp".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_TAB => Some(KeycodeLabel {
            long: Some("Tab".to_string()),
            short: None,
            color: color_modifier,
        }),
        Keycode::KC_SPACE => Some(KeycodeLabel {
            long: Some("Space".to_string()),
            short: Some("Spc".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MINUS => Some(KeycodeLabel {
            long: Some("_\n-".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_EQUAL => Some(KeycodeLabel {
            long: Some("+\n=".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LEFT_BRACKET => Some(KeycodeLabel {
            long: Some("{\n[".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_RIGHT_BRACKET => Some(KeycodeLabel {
            long: Some("}\n]".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_BACKSLASH => Some(KeycodeLabel {
            long: Some("|\n\\".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_NONUS_HASH => Some(KeycodeLabel {
            long: Some("NUHS".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SEMICOLON => Some(KeycodeLabel {
            long: Some(":\n;".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_QUOTE => Some(KeycodeLabel {
            long: Some("\"\n\'".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_GRAVE => Some(KeycodeLabel {
            long: Some("~\n`".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_COMMA => Some(KeycodeLabel {
            long: Some("<\n,".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_DOT => Some(KeycodeLabel {
            long: Some(">\n.".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SLASH => Some(KeycodeLabel {
            long: Some("?\n/".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Caps Lock".to_string()),
            short: Some("Caps".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_F1 => Some(KeycodeLabel {
            long: Some("F1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F2 => Some(KeycodeLabel {
            long: Some("F2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F3 => Some(KeycodeLabel {
            long: Some("F3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F4 => Some(KeycodeLabel {
            long: Some("F4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F5 => Some(KeycodeLabel {
            long: Some("F5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F6 => Some(KeycodeLabel {
            long: Some("F6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F7 => Some(KeycodeLabel {
            long: Some("F7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F8 => Some(KeycodeLabel {
            long: Some("F8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F9 => Some(KeycodeLabel {
            long: Some("F9".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F10 => Some(KeycodeLabel {
            long: Some("F10".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F11 => Some(KeycodeLabel {
            long: Some("F11".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F12 => Some(KeycodeLabel {
            long: Some("F12".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_PRINT_SCREEN => Some(KeycodeLabel {
            long: Some("Print Screen".to_string()),
            short: Some("PrtSc".to_string()),
            ..Default::default()
        }),
        Keycode::KC_SCROLL_LOCK => Some(KeycodeLabel {
            long: Some("Scroll Lock".to_string()),
            short: Some("ScrLk".to_string()),
            ..Default::default()
        }),
        Keycode::KC_PAUSE => Some(KeycodeLabel {
            long: Some("Pause".to_string()),
            short: Some("Paus".to_string()),
            ..Default::default()
        }),
        Keycode::KC_INSERT => Some(KeycodeLabel {
            long: Some("Insert".to_string()),
            short: Some("Ins".to_string()),
            ..Default::default()
        }),
        Keycode::KC_HOME => Some(KeycodeLabel {
            long: Some("Home".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_PAGE_UP => Some(KeycodeLabel {
            long: Some("Page Up".to_string()),
            short: Some("PgUp".to_string()),
            ..Default::default()
        }),
        Keycode::KC_DELETE => Some(KeycodeLabel {
            long: Some("Del".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_END => Some(KeycodeLabel {
            long: Some("End".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_PAGE_DOWN => Some(KeycodeLabel {
            long: Some("Page Down".to_string()),
            short: Some("PgDn".to_string()),
            ..Default::default()
        }),
        Keycode::KC_RIGHT => Some(KeycodeLabel {
            long: Some("➡".to_string()),
            short: None,
            color: color_modifier,
        }),
        Keycode::KC_LEFT => Some(KeycodeLabel {
            long: Some("⬅".to_string()),
            short: None,
            color: color_modifier,
        }),
        Keycode::KC_DOWN => Some(KeycodeLabel {
            long: Some("⬇".to_string()),
            short: None,
            color: color_modifier,
        }),
        Keycode::KC_UP => Some(KeycodeLabel {
            long: Some("⬆".to_string()),
            short: None,
            color: color_modifier,
        }),
        Keycode::KC_NUM_LOCK => Some(KeycodeLabel {
            long: Some("Num\nLock".to_string()),
            short: Some("NumLk".to_string()),
            ..Default::default()
        }),
        Keycode::KC_KP_SLASH => Some(KeycodeLabel {
            long: Some("÷".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_ASTERISK => Some(KeycodeLabel {
            long: Some("×".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_MINUS => Some(KeycodeLabel {
            long: Some("-".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_PLUS => Some(KeycodeLabel {
            long: Some("+".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_ENTER => Some(KeycodeLabel {
            long: Some("Num\nEnter".to_string()),
            short: Some("N.Ent".to_string()),
            ..Default::default()
        }),
        Keycode::KC_KP_1 => Some(KeycodeLabel {
            long: Some("1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_2 => Some(KeycodeLabel {
            long: Some("2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_3 => Some(KeycodeLabel {
            long: Some("3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_4 => Some(KeycodeLabel {
            long: Some("4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_5 => Some(KeycodeLabel {
            long: Some("5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_6 => Some(KeycodeLabel {
            long: Some("6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_7 => Some(KeycodeLabel {
            long: Some("7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_8 => Some(KeycodeLabel {
            long: Some("8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_9 => Some(KeycodeLabel {
            long: Some("9".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_0 => Some(KeycodeLabel {
            long: Some("0".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_DOT => Some(KeycodeLabel {
            long: Some(".".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_NONUS_BACKSLASH => Some(KeycodeLabel {
            long: Some("NUBS".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_APPLICATION => Some(KeycodeLabel {
            long: Some("Menu".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KB_POWER => Some(KeycodeLabel {
            long: Some("Power".to_string()),
            short: Some("Pwr".to_string()),
            ..Default::default()
        }),
        Keycode::KC_KP_EQUAL => Some(KeycodeLabel {
            long: Some("=".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F13 => Some(KeycodeLabel {
            long: Some("F13".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F14 => Some(KeycodeLabel {
            long: Some("F14".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F15 => Some(KeycodeLabel {
            long: Some("F15".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F16 => Some(KeycodeLabel {
            long: Some("F16".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F17 => Some(KeycodeLabel {
            long: Some("F17".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F18 => Some(KeycodeLabel {
            long: Some("F18".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F19 => Some(KeycodeLabel {
            long: Some("F19".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F20 => Some(KeycodeLabel {
            long: Some("F20".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F21 => Some(KeycodeLabel {
            long: Some("F21".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F22 => Some(KeycodeLabel {
            long: Some("F22".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F23 => Some(KeycodeLabel {
            long: Some("F23".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_F24 => Some(KeycodeLabel {
            long: Some("F24".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_EXECUTE => Some(KeycodeLabel {
            long: Some("Exec".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_HELP => Some(KeycodeLabel {
            long: Some("Help".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MENU => Some(KeycodeLabel {
            long: Some("Menu".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SELECT => Some(KeycodeLabel {
            long: Some("Select".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_STOP => Some(KeycodeLabel {
            long: Some("Stop".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_AGAIN => Some(KeycodeLabel {
            long: Some("Again".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_UNDO => Some(KeycodeLabel {
            long: Some("Undo".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CUT => Some(KeycodeLabel {
            long: Some("Cut".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_COPY => Some(KeycodeLabel {
            long: Some("Copy".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_PASTE => Some(KeycodeLabel {
            long: Some("Paste".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_FIND => Some(KeycodeLabel {
            long: Some("Find".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KB_MUTE => Some(KeycodeLabel {
            long: Some("Mute".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KB_VOLUME_UP => Some(KeycodeLabel {
            long: Some("Vol +".to_string()),
            short: Some("Vol+".to_string()),
            ..Default::default()
        }),
        Keycode::KC_KB_VOLUME_DOWN => Some(KeycodeLabel {
            long: Some("Vol -".to_string()),
            short: Some("Vol-".to_string()),
            ..Default::default()
        }),
        Keycode::KC_LOCKING_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Locking Caps Lock".to_string()),
            short: Some("LCaps".to_string()),
            ..Default::default()
        }),
        Keycode::KC_LOCKING_NUM_LOCK => Some(KeycodeLabel {
            long: Some("Locking Num Lock".to_string()),
            short: Some("LNum".to_string()),
            ..Default::default()
        }),
        Keycode::KC_LOCKING_SCROLL_LOCK => Some(KeycodeLabel {
            long: Some("Locking Scroll Lock".to_string()),
            short: Some("LScrl".to_string()),
            ..Default::default()
        }),
        Keycode::KC_KP_COMMA => Some(KeycodeLabel {
            long: Some(",".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_KP_EQUAL_AS400 => Some(KeycodeLabel {
            long: Some("=".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_1 => Some(KeycodeLabel {
            long: Some("Int1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_2 => Some(KeycodeLabel {
            long: Some("Int2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_3 => Some(KeycodeLabel {
            long: Some("Int3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_4 => Some(KeycodeLabel {
            long: Some("Int4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_5 => Some(KeycodeLabel {
            long: Some("Int5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_6 => Some(KeycodeLabel {
            long: Some("Int6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_7 => Some(KeycodeLabel {
            long: Some("Int7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_8 => Some(KeycodeLabel {
            long: Some("Int8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_INTERNATIONAL_9 => Some(KeycodeLabel {
            long: Some("Int9".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_1 => Some(KeycodeLabel {
            long: Some("Lang1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_2 => Some(KeycodeLabel {
            long: Some("Lang2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_3 => Some(KeycodeLabel {
            long: Some("Lang3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_4 => Some(KeycodeLabel {
            long: Some("Lang4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_5 => Some(KeycodeLabel {
            long: Some("Lang5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_6 => Some(KeycodeLabel {
            long: Some("Lang6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_7 => Some(KeycodeLabel {
            long: Some("Lang7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_8 => Some(KeycodeLabel {
            long: Some("Lang8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LANGUAGE_9 => Some(KeycodeLabel {
            long: Some("Lang9".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_ALTERNATE_ERASE => Some(KeycodeLabel {
            long: Some("Alt Erase".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SYSTEM_REQUEST => Some(KeycodeLabel {
            long: Some("SysReq".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CANCEL => Some(KeycodeLabel {
            long: Some("Cancel".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CLEAR => Some(KeycodeLabel {
            long: Some("Clear".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_PRIOR => Some(KeycodeLabel {
            long: Some("Prior".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_RETURN => Some(KeycodeLabel {
            long: Some("Return".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SEPARATOR => Some(KeycodeLabel {
            long: Some("Separator".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_OUT => Some(KeycodeLabel {
            long: Some("Out".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_OPER => Some(KeycodeLabel {
            long: Some("Oper".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CLEAR_AGAIN => Some(KeycodeLabel {
            long: Some("Clear Again".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CRSEL => Some(KeycodeLabel {
            long: Some("CrSel".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_EXSEL => Some(KeycodeLabel {
            long: Some("ExSel".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SYSTEM_POWER => Some(KeycodeLabel {
            long: Some("Power".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SYSTEM_SLEEP => Some(KeycodeLabel {
            long: Some("Sleep".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_SYSTEM_WAKE => Some(KeycodeLabel {
            long: Some("Wake".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_AUDIO_MUTE => Some(KeycodeLabel {
            long: Some("Mute".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_AUDIO_VOL_UP => Some(KeycodeLabel {
            long: Some("Vol +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_AUDIO_VOL_DOWN => Some(KeycodeLabel {
            long: Some("Vol -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MEDIA_NEXT_TRACK => Some(KeycodeLabel {
            long: Some("Next".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MEDIA_PREV_TRACK => Some(KeycodeLabel {
            long: Some("Previous".to_string()),
            short: Some("Prev".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MEDIA_STOP => Some(KeycodeLabel {
            long: Some("Media Stop".to_string()),
            short: Some("Stop".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MEDIA_PLAY_PAUSE => Some(KeycodeLabel {
            long: Some("Play".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MEDIA_SELECT => Some(KeycodeLabel {
            long: Some("Select".to_string()),
            short: Some("Sel".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MEDIA_EJECT => Some(KeycodeLabel {
            long: Some("Eject".to_string()),
            short: Some("Ejct".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MAIL => Some(KeycodeLabel {
            long: Some("Mail".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_CALCULATOR => Some(KeycodeLabel {
            long: Some("Calc".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MY_COMPUTER => Some(KeycodeLabel {
            long: Some("My Comp".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_SEARCH => Some(KeycodeLabel {
            long: Some("Search".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_HOME => Some(KeycodeLabel {
            long: Some("Home".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_BACK => Some(KeycodeLabel {
            long: Some("Back".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_FORWARD => Some(KeycodeLabel {
            long: Some("Forward".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_STOP => Some(KeycodeLabel {
            long: Some("Stop".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_REFRESH => Some(KeycodeLabel {
            long: Some("Refresh".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_WWW_FAVORITES => Some(KeycodeLabel {
            long: Some("Favorites".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_MEDIA_FAST_FORWARD => Some(KeycodeLabel {
            long: Some("Fast Forward".to_string()),
            short: Some("FF".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MEDIA_REWIND => Some(KeycodeLabel {
            long: Some("Rewind".to_string()),
            short: Some("Rwd".to_string()),
            ..Default::default()
        }),
        Keycode::KC_BRIGHTNESS_UP => Some(KeycodeLabel {
            long: Some("Screen +".to_string()),
            short: Some("Scr +".to_string()),
            ..Default::default()
        }),
        Keycode::KC_BRIGHTNESS_DOWN => Some(KeycodeLabel {
            long: Some("Screen -".to_string()),
            short: Some("Scr -".to_string()),
            ..Default::default()
        }),
        Keycode::KC_CONTROL_PANEL => Some(KeycodeLabel {
            long: Some("Control Panel".to_string()),
            short: Some("Ctrl P".to_string()),
            ..Default::default()
        }),
        Keycode::KC_ASSISTANT => Some(KeycodeLabel {
            long: Some("Assistant".to_string()),
            short: Some("Asst".to_string()),
            ..Default::default()
        }),
        Keycode::KC_MISSION_CONTROL => Some(KeycodeLabel {
            long: Some("Mission Control".to_string()),
            short: Some("MC".to_string()),
            ..Default::default()
        }),
        Keycode::KC_LAUNCHPAD => Some(KeycodeLabel {
            long: Some("Launchpad".to_string()),
            short: Some("LP".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MOUSE_CURSOR_UP => Some(KeycodeLabel {
            long: Some("Mouse ↑".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_CURSOR_DOWN => Some(KeycodeLabel {
            long: Some("Mouse ↓".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_CURSOR_LEFT => Some(KeycodeLabel {
            long: Some("Mouse ←".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_CURSOR_RIGHT => Some(KeycodeLabel {
            long: Some("Mouse →".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_1 => Some(KeycodeLabel {
            long: Some("Mouse Btn1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_2 => Some(KeycodeLabel {
            long: Some("Mouse Btn2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_3 => Some(KeycodeLabel {
            long: Some("Mouse Btn3".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_4 => Some(KeycodeLabel {
            long: Some("Mouse Btn4".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_5 => Some(KeycodeLabel {
            long: Some("Mouse Btn5".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_6 => Some(KeycodeLabel {
            long: Some("Mouse Btn6".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_7 => Some(KeycodeLabel {
            long: Some("Mouse Btn7".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_BUTTON_8 => Some(KeycodeLabel {
            long: Some("Mouse Btn8".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_WHEEL_UP => Some(KeycodeLabel {
            long: Some("Mouse Wh ↑".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_WHEEL_DOWN => Some(KeycodeLabel {
            long: Some("Mouse Wh ↓".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_WHEEL_LEFT => Some(KeycodeLabel {
            long: Some("Mouse Wh ←".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_WHEEL_RIGHT => Some(KeycodeLabel {
            long: Some("Mouse Wh →".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_ACCELERATION_0 => Some(KeycodeLabel {
            long: Some("Mouse Acc0".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_ACCELERATION_1 => Some(KeycodeLabel {
            long: Some("Mouse Acc1".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_MOUSE_ACCELERATION_2 => Some(KeycodeLabel {
            long: Some("Mouse Acc2".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::KC_LEFT_CTRL => Some(KeycodeLabel {
            long: Some("Left Ctrl".to_string()),
            short: Some("LCtrl".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_LEFT_SHIFT => Some(KeycodeLabel {
            long: Some("Left Shift".to_string()),
            short: Some("LShft".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_LEFT_ALT => Some(KeycodeLabel {
            long: Some("Left Alt".to_string()),
            short: Some("LAlt".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_LEFT_GUI => Some(KeycodeLabel {
            long: Some("Left Win".to_string()),
            short: Some("LWin".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_RIGHT_CTRL => Some(KeycodeLabel {
            long: Some("Right Ctrl".to_string()),
            short: Some("RCtrl".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_RIGHT_SHIFT => Some(KeycodeLabel {
            long: Some("Right Shift".to_string()),
            short: Some("RShft".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_RIGHT_ALT => Some(KeycodeLabel {
            long: Some("Right Alt".to_string()),
            short: Some("RAlt".to_string()),
            color: color_modifier,
        }),
        Keycode::KC_RIGHT_GUI => Some(KeycodeLabel {
            long: Some("Right Win".to_string()),
            short: Some("RWin".to_string()),
            color: color_modifier,
        }),
        Keycode::QK_SWAP_HANDS_TOGGLE => Some(KeycodeLabel {
            long: Some("Swap Hands Toggle".to_string()),
            short: Some("SwpHT".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_TAP_TOGGLE => Some(KeycodeLabel {
            long: Some("Swap Hands Tap Toggle".to_string()),
            short: Some("SwpTT".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_MOMENTARY_ON => Some(KeycodeLabel {
            long: Some("Swap Hands On".to_string()),
            short: Some("SwpOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_MOMENTARY_OFF => Some(KeycodeLabel {
            long: Some("Swap Hands Off".to_string()),
            short: Some("SwpOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_OFF => Some(KeycodeLabel {
            long: Some("Swap Hands Off".to_string()),
            short: Some("SwpOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_ON => Some(KeycodeLabel {
            long: Some("Swap Hands On".to_string()),
            short: Some("SwpOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SWAP_HANDS_ONE_SHOT => Some(KeycodeLabel {
            long: Some("Swap Hands One Shot".to_string()),
            short: Some("SwpOS".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_CONTROL_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Swap Ctrl Caps".to_string()),
            short: Some("SwCtCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_CONTROL_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Unswap Ctrl Caps".to_string()),
            short: Some("UnCtCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_CONTROL_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Toggle Ctrl Caps".to_string()),
            short: Some("TgCtCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_CAPS_LOCK_AS_CONTROL_OFF => Some(KeycodeLabel {
            long: Some("Caps as Ctrl Off".to_string()),
            short: Some("CpCtOf".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_CAPS_LOCK_AS_CONTROL_ON => Some(KeycodeLabel {
            long: Some("Caps as Ctrl On".to_string()),
            short: Some("CpCtOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_LALT_LGUI => Some(KeycodeLabel {
            long: Some("Swap LAlt LGui".to_string()),
            short: Some("SwAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_LALT_LGUI => Some(KeycodeLabel {
            long: Some("Unswap LAlt LGui".to_string()),
            short: Some("UnAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_RALT_RGUI => Some(KeycodeLabel {
            long: Some("Swap RAlt RGui".to_string()),
            short: Some("SwAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_RALT_RGUI => Some(KeycodeLabel {
            long: Some("Unswap RAlt RGui".to_string()),
            short: Some("UnAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_GUI_ON => Some(KeycodeLabel {
            long: Some("GUI On".to_string()),
            short: Some("GuiOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_GUI_OFF => Some(KeycodeLabel {
            long: Some("GUI Off".to_string()),
            short: Some("GuiOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_GUI => Some(KeycodeLabel {
            long: Some("Toggle GUI".to_string()),
            short: Some("TgGui".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_GRAVE_ESC => Some(KeycodeLabel {
            long: Some("Swap ` Esc".to_string()),
            short: Some("Sw`Esc".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_GRAVE_ESC => Some(KeycodeLabel {
            long: Some("Unswap ` Esc".to_string()),
            short: Some("Un`Esc".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_BACKSLASH_BACKSPACE => Some(KeycodeLabel {
            long: Some("Swap \\ Bksp".to_string()),
            short: Some("Sw\\Bk".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_BACKSLASH_BACKSPACE => Some(KeycodeLabel {
            long: Some("Unswap \\ Bksp".to_string()),
            short: Some("Un\\Bk".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_BACKSLASH_BACKSPACE => Some(KeycodeLabel {
            long: Some("Toggle \\ Bksp".to_string()),
            short: Some("Tg\\Bk".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_NKRO_ON => Some(KeycodeLabel {
            long: Some("NKRO On".to_string()),
            short: Some("NKROOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_NKRO_OFF => Some(KeycodeLabel {
            long: Some("NKRO Off".to_string()),
            short: Some("NKROOf".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_NKRO => Some(KeycodeLabel {
            long: Some("Toggle NKRO".to_string()),
            short: Some("NKRO".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_ALT_GUI => Some(KeycodeLabel {
            long: Some("Swap Alt GUI".to_string()),
            short: Some("SwAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_ALT_GUI => Some(KeycodeLabel {
            long: Some("Unswap Alt GUI".to_string()),
            short: Some("UnAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_ALT_GUI => Some(KeycodeLabel {
            long: Some("Toggle Alt GUI".to_string()),
            short: Some("TgAltG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_LCTL_LGUI => Some(KeycodeLabel {
            long: Some("Swap LCtl LGui".to_string()),
            short: Some("SwCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_LCTL_LGUI => Some(KeycodeLabel {
            long: Some("Unswap LCtl LGui".to_string()),
            short: Some("UnCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_RCTL_RGUI => Some(KeycodeLabel {
            long: Some("Swap RCtl RGui".to_string()),
            short: Some("SwCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_RCTL_RGUI => Some(KeycodeLabel {
            long: Some("Unswap RCtl RGui".to_string()),
            short: Some("UnCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_CTL_GUI => Some(KeycodeLabel {
            long: Some("Swap Ctl GUI".to_string()),
            short: Some("SwCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_CTL_GUI => Some(KeycodeLabel {
            long: Some("Unswap Ctl GUI".to_string()),
            short: Some("UnCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_CTL_GUI => Some(KeycodeLabel {
            long: Some("Toggle Ctl GUI".to_string()),
            short: Some("TgCtlG".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_EE_HANDS_LEFT => Some(KeycodeLabel {
            long: Some("EE Hands Left".to_string()),
            short: Some("EEHndL".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_EE_HANDS_RIGHT => Some(KeycodeLabel {
            long: Some("EE Hands Right".to_string()),
            short: Some("EEHndR".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_SWAP_ESCAPE_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Swap Esc Caps".to_string()),
            short: Some("SwEsCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_UNSWAP_ESCAPE_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Unswap Esc Caps".to_string()),
            short: Some("UnEsCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAGIC_TOGGLE_ESCAPE_CAPS_LOCK => Some(KeycodeLabel {
            long: Some("Toggle Esc Caps".to_string()),
            short: Some("TgEsCp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_ON => Some(KeycodeLabel {
            long: Some("MIDI On".to_string()),
            short: Some("MDOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OFF => Some(KeycodeLabel {
            long: Some("MIDI Off".to_string()),
            short: Some("MDOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TOGGLE => Some(KeycodeLabel {
            long: Some("MIDI Toggle".to_string()),
            short: Some("MDTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_0 => Some(KeycodeLabel {
            long: Some("MIDI C0".to_string()),
            short: Some("MDC0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_0 => Some(KeycodeLabel {
            long: Some("MIDI C#0".to_string()),
            short: Some("MDC#0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_0 => Some(KeycodeLabel {
            long: Some("MIDI D0".to_string()),
            short: Some("MDD0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_0 => Some(KeycodeLabel {
            long: Some("MIDI D#0".to_string()),
            short: Some("MDD#0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_0 => Some(KeycodeLabel {
            long: Some("MIDI E0".to_string()),
            short: Some("MDE0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_0 => Some(KeycodeLabel {
            long: Some("MIDI F0".to_string()),
            short: Some("MDF0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_0 => Some(KeycodeLabel {
            long: Some("MIDI F#0".to_string()),
            short: Some("MDF#0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_0 => Some(KeycodeLabel {
            long: Some("MIDI G0".to_string()),
            short: Some("MDG0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_0 => Some(KeycodeLabel {
            long: Some("MIDI G#0".to_string()),
            short: Some("MDG#0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_0 => Some(KeycodeLabel {
            long: Some("MIDI A0".to_string()),
            short: Some("MDA0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_0 => Some(KeycodeLabel {
            long: Some("MIDI A#0".to_string()),
            short: Some("MDA#0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_0 => Some(KeycodeLabel {
            long: Some("MIDI B0".to_string()),
            short: Some("MDB0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_1 => Some(KeycodeLabel {
            long: Some("MIDI C1".to_string()),
            short: Some("MDC1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_1 => Some(KeycodeLabel {
            long: Some("MIDI C#1".to_string()),
            short: Some("MDC#1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_1 => Some(KeycodeLabel {
            long: Some("MIDI D1".to_string()),
            short: Some("MDD1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_1 => Some(KeycodeLabel {
            long: Some("MIDI D#1".to_string()),
            short: Some("MDD#1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_1 => Some(KeycodeLabel {
            long: Some("MIDI E1".to_string()),
            short: Some("MDE1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_1 => Some(KeycodeLabel {
            long: Some("MIDI F1".to_string()),
            short: Some("MDF1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_1 => Some(KeycodeLabel {
            long: Some("MIDI F#1".to_string()),
            short: Some("MDF#1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_1 => Some(KeycodeLabel {
            long: Some("MIDI G1".to_string()),
            short: Some("MDG1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_1 => Some(KeycodeLabel {
            long: Some("MIDI G#1".to_string()),
            short: Some("MDG#1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_1 => Some(KeycodeLabel {
            long: Some("MIDI A1".to_string()),
            short: Some("MDA1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_1 => Some(KeycodeLabel {
            long: Some("MIDI A#1".to_string()),
            short: Some("MDA#1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_1 => Some(KeycodeLabel {
            long: Some("MIDI B1".to_string()),
            short: Some("MDB1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_2 => Some(KeycodeLabel {
            long: Some("MIDI C2".to_string()),
            short: Some("MDC2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_2 => Some(KeycodeLabel {
            long: Some("MIDI C#2".to_string()),
            short: Some("MDC#2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_2 => Some(KeycodeLabel {
            long: Some("MIDI D2".to_string()),
            short: Some("MDD2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_2 => Some(KeycodeLabel {
            long: Some("MIDI D#2".to_string()),
            short: Some("MDD#2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_2 => Some(KeycodeLabel {
            long: Some("MIDI E2".to_string()),
            short: Some("MDE2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_2 => Some(KeycodeLabel {
            long: Some("MIDI F2".to_string()),
            short: Some("MDF2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_2 => Some(KeycodeLabel {
            long: Some("MIDI F#2".to_string()),
            short: Some("MDF#2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_2 => Some(KeycodeLabel {
            long: Some("MIDI G2".to_string()),
            short: Some("MDG2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_2 => Some(KeycodeLabel {
            long: Some("MIDI G#2".to_string()),
            short: Some("MDG#2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_2 => Some(KeycodeLabel {
            long: Some("MIDI A2".to_string()),
            short: Some("MDA2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_2 => Some(KeycodeLabel {
            long: Some("MIDI A#2".to_string()),
            short: Some("MDA#2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_2 => Some(KeycodeLabel {
            long: Some("MIDI B2".to_string()),
            short: Some("MDB2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_3 => Some(KeycodeLabel {
            long: Some("MIDI C3".to_string()),
            short: Some("MDC3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_3 => Some(KeycodeLabel {
            long: Some("MIDI C#3".to_string()),
            short: Some("MDC#3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_3 => Some(KeycodeLabel {
            long: Some("MIDI D3".to_string()),
            short: Some("MDD3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_3 => Some(KeycodeLabel {
            long: Some("MIDI D#3".to_string()),
            short: Some("MDD#3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_3 => Some(KeycodeLabel {
            long: Some("MIDI E3".to_string()),
            short: Some("MDE3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_3 => Some(KeycodeLabel {
            long: Some("MIDI F3".to_string()),
            short: Some("MDF3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_3 => Some(KeycodeLabel {
            long: Some("MIDI F#3".to_string()),
            short: Some("MDF#3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_3 => Some(KeycodeLabel {
            long: Some("MIDI G3".to_string()),
            short: Some("MDG3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_3 => Some(KeycodeLabel {
            long: Some("MIDI G#3".to_string()),
            short: Some("MDG#3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_3 => Some(KeycodeLabel {
            long: Some("MIDI A3".to_string()),
            short: Some("MDA3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_3 => Some(KeycodeLabel {
            long: Some("MIDI A#3".to_string()),
            short: Some("MDA#3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_3 => Some(KeycodeLabel {
            long: Some("MIDI B3".to_string()),
            short: Some("MDB3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_4 => Some(KeycodeLabel {
            long: Some("MIDI C4".to_string()),
            short: Some("MDC4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_4 => Some(KeycodeLabel {
            long: Some("MIDI C#4".to_string()),
            short: Some("MDC#4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_4 => Some(KeycodeLabel {
            long: Some("MIDI D4".to_string()),
            short: Some("MDD4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_4 => Some(KeycodeLabel {
            long: Some("MIDI D#4".to_string()),
            short: Some("MDD#4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_4 => Some(KeycodeLabel {
            long: Some("MIDI E4".to_string()),
            short: Some("MDE4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_4 => Some(KeycodeLabel {
            long: Some("MIDI F4".to_string()),
            short: Some("MDF4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_4 => Some(KeycodeLabel {
            long: Some("MIDI F#4".to_string()),
            short: Some("MDF#4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_4 => Some(KeycodeLabel {
            long: Some("MIDI G4".to_string()),
            short: Some("MDG4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_4 => Some(KeycodeLabel {
            long: Some("MIDI G#4".to_string()),
            short: Some("MDG#4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_4 => Some(KeycodeLabel {
            long: Some("MIDI A4".to_string()),
            short: Some("MDA4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_4 => Some(KeycodeLabel {
            long: Some("MIDI A#4".to_string()),
            short: Some("MDA#4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_4 => Some(KeycodeLabel {
            long: Some("MIDI B4".to_string()),
            short: Some("MDB4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_5 => Some(KeycodeLabel {
            long: Some("MIDI C5".to_string()),
            short: Some("MDC5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_C_SHARP_5 => Some(KeycodeLabel {
            long: Some("MIDI C#5".to_string()),
            short: Some("MDC#5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_5 => Some(KeycodeLabel {
            long: Some("MIDI D5".to_string()),
            short: Some("MDD5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_D_SHARP_5 => Some(KeycodeLabel {
            long: Some("MIDI D#5".to_string()),
            short: Some("MDD#5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_E_5 => Some(KeycodeLabel {
            long: Some("MIDI E5".to_string()),
            short: Some("MDE5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_5 => Some(KeycodeLabel {
            long: Some("MIDI F5".to_string()),
            short: Some("MDF5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_F_SHARP_5 => Some(KeycodeLabel {
            long: Some("MIDI F#5".to_string()),
            short: Some("MDF#5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_5 => Some(KeycodeLabel {
            long: Some("MIDI G5".to_string()),
            short: Some("MDG5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_G_SHARP_5 => Some(KeycodeLabel {
            long: Some("MIDI G#5".to_string()),
            short: Some("MDG#5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_5 => Some(KeycodeLabel {
            long: Some("MIDI A5".to_string()),
            short: Some("MDA5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_A_SHARP_5 => Some(KeycodeLabel {
            long: Some("MIDI A#5".to_string()),
            short: Some("MDA#5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_NOTE_B_5 => Some(KeycodeLabel {
            long: Some("MIDI B5".to_string()),
            short: Some("MDB5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_N2 => Some(KeycodeLabel {
            long: Some("MIDI Oct -2".to_string()),
            short: Some("MDO-2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_N1 => Some(KeycodeLabel {
            long: Some("MIDI Oct -1".to_string()),
            short: Some("MDO-1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_0 => Some(KeycodeLabel {
            long: Some("MIDI Oct 0".to_string()),
            short: Some("MDO0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_1 => Some(KeycodeLabel {
            long: Some("MIDI Oct 1".to_string()),
            short: Some("MDO1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_2 => Some(KeycodeLabel {
            long: Some("MIDI Oct 2".to_string()),
            short: Some("MDO2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_3 => Some(KeycodeLabel {
            long: Some("MIDI Oct 3".to_string()),
            short: Some("MDO3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_4 => Some(KeycodeLabel {
            long: Some("MIDI Oct 4".to_string()),
            short: Some("MDO4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_5 => Some(KeycodeLabel {
            long: Some("MIDI Oct 5".to_string()),
            short: Some("MDO5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_6 => Some(KeycodeLabel {
            long: Some("MIDI Oct 6".to_string()),
            short: Some("MDO6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_7 => Some(KeycodeLabel {
            long: Some("MIDI Oct 7".to_string()),
            short: Some("MDO7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Oct Down".to_string()),
            short: Some("MDO-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_OCTAVE_UP => Some(KeycodeLabel {
            long: Some("MIDI Oct Up".to_string()),
            short: Some("MDO+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N6 => Some(KeycodeLabel {
            long: Some("MIDI Trans -6".to_string()),
            short: Some("MDT-6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N5 => Some(KeycodeLabel {
            long: Some("MIDI Trans -5".to_string()),
            short: Some("MDT-5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N4 => Some(KeycodeLabel {
            long: Some("MIDI Trans -4".to_string()),
            short: Some("MDT-4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N3 => Some(KeycodeLabel {
            long: Some("MIDI Trans -3".to_string()),
            short: Some("MDT-3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N2 => Some(KeycodeLabel {
            long: Some("MIDI Trans -2".to_string()),
            short: Some("MDT-2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_N1 => Some(KeycodeLabel {
            long: Some("MIDI Trans -1".to_string()),
            short: Some("MDT-1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_0 => Some(KeycodeLabel {
            long: Some("MIDI Trans 0".to_string()),
            short: Some("MDT0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_1 => Some(KeycodeLabel {
            long: Some("MIDI Trans 1".to_string()),
            short: Some("MDT1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_2 => Some(KeycodeLabel {
            long: Some("MIDI Trans 2".to_string()),
            short: Some("MDT2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_3 => Some(KeycodeLabel {
            long: Some("MIDI Trans 3".to_string()),
            short: Some("MDT3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_4 => Some(KeycodeLabel {
            long: Some("MIDI Trans 4".to_string()),
            short: Some("MDT4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_5 => Some(KeycodeLabel {
            long: Some("MIDI Trans 5".to_string()),
            short: Some("MDT5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_6 => Some(KeycodeLabel {
            long: Some("MIDI Trans 6".to_string()),
            short: Some("MDT6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Trans Down".to_string()),
            short: Some("MDT-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_TRANSPOSE_UP => Some(KeycodeLabel {
            long: Some("MIDI Trans Up".to_string()),
            short: Some("MDT+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_0 => Some(KeycodeLabel {
            long: Some("MIDI Vel 0".to_string()),
            short: Some("MDV0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_1 => Some(KeycodeLabel {
            long: Some("MIDI Vel 1".to_string()),
            short: Some("MDV1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_2 => Some(KeycodeLabel {
            long: Some("MIDI Vel 2".to_string()),
            short: Some("MDV2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_3 => Some(KeycodeLabel {
            long: Some("MIDI Vel 3".to_string()),
            short: Some("MDV3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_4 => Some(KeycodeLabel {
            long: Some("MIDI Vel 4".to_string()),
            short: Some("MDV4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_5 => Some(KeycodeLabel {
            long: Some("MIDI Vel 5".to_string()),
            short: Some("MDV5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_6 => Some(KeycodeLabel {
            long: Some("MIDI Vel 6".to_string()),
            short: Some("MDV6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_7 => Some(KeycodeLabel {
            long: Some("MIDI Vel 7".to_string()),
            short: Some("MDV7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_8 => Some(KeycodeLabel {
            long: Some("MIDI Vel 8".to_string()),
            short: Some("MDV8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_9 => Some(KeycodeLabel {
            long: Some("MIDI Vel 9".to_string()),
            short: Some("MDV9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_10 => Some(KeycodeLabel {
            long: Some("MIDI Vel 10".to_string()),
            short: Some("MDV10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Vel Down".to_string()),
            short: Some("MDV-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_VELOCITY_UP => Some(KeycodeLabel {
            long: Some("MIDI Vel Up".to_string()),
            short: Some("MDV+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_1 => Some(KeycodeLabel {
            long: Some("MIDI Ch 1".to_string()),
            short: Some("MDC1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_2 => Some(KeycodeLabel {
            long: Some("MIDI Ch 2".to_string()),
            short: Some("MDC2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_3 => Some(KeycodeLabel {
            long: Some("MIDI Ch 3".to_string()),
            short: Some("MDC3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_4 => Some(KeycodeLabel {
            long: Some("MIDI Ch 4".to_string()),
            short: Some("MDC4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_5 => Some(KeycodeLabel {
            long: Some("MIDI Ch 5".to_string()),
            short: Some("MDC5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_6 => Some(KeycodeLabel {
            long: Some("MIDI Ch 6".to_string()),
            short: Some("MDC6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_7 => Some(KeycodeLabel {
            long: Some("MIDI Ch 7".to_string()),
            short: Some("MDC7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_8 => Some(KeycodeLabel {
            long: Some("MIDI Ch 8".to_string()),
            short: Some("MDC8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_9 => Some(KeycodeLabel {
            long: Some("MIDI Ch 9".to_string()),
            short: Some("MDC9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_10 => Some(KeycodeLabel {
            long: Some("MIDI Ch 10".to_string()),
            short: Some("MDC10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_11 => Some(KeycodeLabel {
            long: Some("MIDI Ch 11".to_string()),
            short: Some("MDC11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_12 => Some(KeycodeLabel {
            long: Some("MIDI Ch 12".to_string()),
            short: Some("MDC12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_13 => Some(KeycodeLabel {
            long: Some("MIDI Ch 13".to_string()),
            short: Some("MDC13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_14 => Some(KeycodeLabel {
            long: Some("MIDI Ch 14".to_string()),
            short: Some("MDC14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_15 => Some(KeycodeLabel {
            long: Some("MIDI Ch 15".to_string()),
            short: Some("MDC15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_16 => Some(KeycodeLabel {
            long: Some("MIDI Ch 16".to_string()),
            short: Some("MDC16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Ch Down".to_string()),
            short: Some("MDC-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_CHANNEL_UP => Some(KeycodeLabel {
            long: Some("MIDI Ch Up".to_string()),
            short: Some("MDC+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_ALL_NOTES_OFF => Some(KeycodeLabel {
            long: Some("MIDI All Off".to_string()),
            short: Some("MDAOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_SUSTAIN => Some(KeycodeLabel {
            long: Some("MIDI Sustain".to_string()),
            short: Some("MDSus".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_PORTAMENTO => Some(KeycodeLabel {
            long: Some("MIDI Portamento".to_string()),
            short: Some("MDPort".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_SOSTENUTO => Some(KeycodeLabel {
            long: Some("MIDI Sostenuto".to_string()),
            short: Some("MDSost".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_SOFT => Some(KeycodeLabel {
            long: Some("MIDI Soft".to_string()),
            short: Some("MDSoft".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_LEGATO => Some(KeycodeLabel {
            long: Some("MIDI Legato".to_string()),
            short: Some("MDLeg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_MODULATION => Some(KeycodeLabel {
            long: Some("MIDI Modulation".to_string()),
            short: Some("MDMod".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_MODULATION_SPEED_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Mod Speed -".to_string()),
            short: Some("MDM-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_MODULATION_SPEED_UP => Some(KeycodeLabel {
            long: Some("MIDI Mod Speed +".to_string()),
            short: Some("MDM+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_PITCH_BEND_DOWN => Some(KeycodeLabel {
            long: Some("MIDI Pitch -".to_string()),
            short: Some("MDP-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MIDI_PITCH_BEND_UP => Some(KeycodeLabel {
            long: Some("MIDI Pitch +".to_string()),
            short: Some("MDP+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_ON => Some(KeycodeLabel {
            long: Some("Sequencer On".to_string()),
            short: Some("SeqOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_OFF => Some(KeycodeLabel {
            long: Some("Sequencer Off".to_string()),
            short: Some("SeqOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_TOGGLE => Some(KeycodeLabel {
            long: Some("Sequencer Toggle".to_string()),
            short: Some("SeqTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_TEMPO_DOWN => Some(KeycodeLabel {
            long: Some("Seq Tempo -".to_string()),
            short: Some("SeqT-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_TEMPO_UP => Some(KeycodeLabel {
            long: Some("Seq Tempo +".to_string()),
            short: Some("SeqT+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_RESOLUTION_DOWN => Some(KeycodeLabel {
            long: Some("Seq Res -".to_string()),
            short: Some("SeqR-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_RESOLUTION_UP => Some(KeycodeLabel {
            long: Some("Seq Res +".to_string()),
            short: Some("SeqR+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_STEPS_ALL => Some(KeycodeLabel {
            long: Some("Seq All Steps".to_string()),
            short: Some("SeqAll".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SEQUENCER_STEPS_CLEAR => Some(KeycodeLabel {
            long: Some("Seq Clear Steps".to_string()),
            short: Some("SeqClr".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_0 => Some(KeycodeLabel {
            long: Some("Joy Btn 0".to_string()),
            short: Some("JoyB0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_1 => Some(KeycodeLabel {
            long: Some("Joy Btn 1".to_string()),
            short: Some("JoyB1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_2 => Some(KeycodeLabel {
            long: Some("Joy Btn 2".to_string()),
            short: Some("JoyB2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_3 => Some(KeycodeLabel {
            long: Some("Joy Btn 3".to_string()),
            short: Some("JoyB3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_4 => Some(KeycodeLabel {
            long: Some("Joy Btn 4".to_string()),
            short: Some("JoyB4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_5 => Some(KeycodeLabel {
            long: Some("Joy Btn 5".to_string()),
            short: Some("JoyB5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_6 => Some(KeycodeLabel {
            long: Some("Joy Btn 6".to_string()),
            short: Some("JoyB6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_7 => Some(KeycodeLabel {
            long: Some("Joy Btn 7".to_string()),
            short: Some("JoyB7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_8 => Some(KeycodeLabel {
            long: Some("Joy Btn 8".to_string()),
            short: Some("JoyB8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_9 => Some(KeycodeLabel {
            long: Some("Joy Btn 9".to_string()),
            short: Some("JoyB9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_10 => Some(KeycodeLabel {
            long: Some("Joy Btn 10".to_string()),
            short: Some("JoyB10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_11 => Some(KeycodeLabel {
            long: Some("Joy Btn 11".to_string()),
            short: Some("JoyB11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_12 => Some(KeycodeLabel {
            long: Some("Joy Btn 12".to_string()),
            short: Some("JoyB12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_13 => Some(KeycodeLabel {
            long: Some("Joy Btn 13".to_string()),
            short: Some("JoyB13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_14 => Some(KeycodeLabel {
            long: Some("Joy Btn 14".to_string()),
            short: Some("JoyB14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_15 => Some(KeycodeLabel {
            long: Some("Joy Btn 15".to_string()),
            short: Some("JoyB15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_16 => Some(KeycodeLabel {
            long: Some("Joy Btn 16".to_string()),
            short: Some("JoyB16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_17 => Some(KeycodeLabel {
            long: Some("Joy Btn 17".to_string()),
            short: Some("JoyB17".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_18 => Some(KeycodeLabel {
            long: Some("Joy Btn 18".to_string()),
            short: Some("JoyB18".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_19 => Some(KeycodeLabel {
            long: Some("Joy Btn 19".to_string()),
            short: Some("JoyB19".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_20 => Some(KeycodeLabel {
            long: Some("Joy Btn 20".to_string()),
            short: Some("JoyB20".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_21 => Some(KeycodeLabel {
            long: Some("Joy Btn 21".to_string()),
            short: Some("JoyB21".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_22 => Some(KeycodeLabel {
            long: Some("Joy Btn 22".to_string()),
            short: Some("JoyB22".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_23 => Some(KeycodeLabel {
            long: Some("Joy Btn 23".to_string()),
            short: Some("JoyB23".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_24 => Some(KeycodeLabel {
            long: Some("Joy Btn 24".to_string()),
            short: Some("JoyB24".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_25 => Some(KeycodeLabel {
            long: Some("Joy Btn 25".to_string()),
            short: Some("JoyB25".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_26 => Some(KeycodeLabel {
            long: Some("Joy Btn 26".to_string()),
            short: Some("JoyB26".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_27 => Some(KeycodeLabel {
            long: Some("Joy Btn 27".to_string()),
            short: Some("JoyB27".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_28 => Some(KeycodeLabel {
            long: Some("Joy Btn 28".to_string()),
            short: Some("JoyB28".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_29 => Some(KeycodeLabel {
            long: Some("Joy Btn 29".to_string()),
            short: Some("JoyB29".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_30 => Some(KeycodeLabel {
            long: Some("Joy Btn 30".to_string()),
            short: Some("JoyB30".to_string()),
            ..Default::default()
        }),
        Keycode::QK_JOYSTICK_BUTTON_31 => Some(KeycodeLabel {
            long: Some("Joy Btn 31".to_string()),
            short: Some("JoyB31".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_1 => Some(KeycodeLabel {
            long: Some("Prog Btn 1".to_string()),
            short: Some("PB1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_2 => Some(KeycodeLabel {
            long: Some("Prog Btn 2".to_string()),
            short: Some("PB2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_3 => Some(KeycodeLabel {
            long: Some("Prog Btn 3".to_string()),
            short: Some("PB3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_4 => Some(KeycodeLabel {
            long: Some("Prog Btn 4".to_string()),
            short: Some("PB4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_5 => Some(KeycodeLabel {
            long: Some("Prog Btn 5".to_string()),
            short: Some("PB5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_6 => Some(KeycodeLabel {
            long: Some("Prog Btn 6".to_string()),
            short: Some("PB6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_7 => Some(KeycodeLabel {
            long: Some("Prog Btn 7".to_string()),
            short: Some("PB7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_8 => Some(KeycodeLabel {
            long: Some("Prog Btn 8".to_string()),
            short: Some("PB8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_9 => Some(KeycodeLabel {
            long: Some("Prog Btn 9".to_string()),
            short: Some("PB9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_10 => Some(KeycodeLabel {
            long: Some("Prog Btn 10".to_string()),
            short: Some("PB10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_11 => Some(KeycodeLabel {
            long: Some("Prog Btn 11".to_string()),
            short: Some("PB11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_12 => Some(KeycodeLabel {
            long: Some("Prog Btn 12".to_string()),
            short: Some("PB12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_13 => Some(KeycodeLabel {
            long: Some("Prog Btn 13".to_string()),
            short: Some("PB13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_14 => Some(KeycodeLabel {
            long: Some("Prog Btn 14".to_string()),
            short: Some("PB14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_15 => Some(KeycodeLabel {
            long: Some("Prog Btn 15".to_string()),
            short: Some("PB15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_16 => Some(KeycodeLabel {
            long: Some("Prog Btn 16".to_string()),
            short: Some("PB16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_17 => Some(KeycodeLabel {
            long: Some("Prog Btn 17".to_string()),
            short: Some("PB17".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_18 => Some(KeycodeLabel {
            long: Some("Prog Btn 18".to_string()),
            short: Some("PB18".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_19 => Some(KeycodeLabel {
            long: Some("Prog Btn 19".to_string()),
            short: Some("PB19".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_20 => Some(KeycodeLabel {
            long: Some("Prog Btn 20".to_string()),
            short: Some("PB20".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_21 => Some(KeycodeLabel {
            long: Some("Prog Btn 21".to_string()),
            short: Some("PB21".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_22 => Some(KeycodeLabel {
            long: Some("Prog Btn 22".to_string()),
            short: Some("PB22".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_23 => Some(KeycodeLabel {
            long: Some("Prog Btn 23".to_string()),
            short: Some("PB23".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_24 => Some(KeycodeLabel {
            long: Some("Prog Btn 24".to_string()),
            short: Some("PB24".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_25 => Some(KeycodeLabel {
            long: Some("Prog Btn 25".to_string()),
            short: Some("PB25".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_26 => Some(KeycodeLabel {
            long: Some("Prog Btn 26".to_string()),
            short: Some("PB26".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_27 => Some(KeycodeLabel {
            long: Some("Prog Btn 27".to_string()),
            short: Some("PB27".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_28 => Some(KeycodeLabel {
            long: Some("Prog Btn 28".to_string()),
            short: Some("PB28".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_29 => Some(KeycodeLabel {
            long: Some("Prog Btn 29".to_string()),
            short: Some("PB29".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_30 => Some(KeycodeLabel {
            long: Some("Prog Btn 30".to_string()),
            short: Some("PB30".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_31 => Some(KeycodeLabel {
            long: Some("Prog Btn 31".to_string()),
            short: Some("PB31".to_string()),
            ..Default::default()
        }),
        Keycode::QK_PROGRAMMABLE_BUTTON_32 => Some(KeycodeLabel {
            long: Some("Prog Btn 32".to_string()),
            short: Some("PB32".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_ON => Some(KeycodeLabel {
            long: Some("Audio On".to_string()),
            short: Some("AudOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_OFF => Some(KeycodeLabel {
            long: Some("Audio Off".to_string()),
            short: Some("AudOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_TOGGLE => Some(KeycodeLabel {
            long: Some("Audio Toggle".to_string()),
            short: Some("AudTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_TOGGLE => Some(KeycodeLabel {
            long: Some("Clicky Toggle".to_string()),
            short: Some("ClkTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_ON => Some(KeycodeLabel {
            long: Some("Clicky Enable".to_string()),
            short: Some("ClkOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_OFF => Some(KeycodeLabel {
            long: Some("Clicky Disable".to_string()),
            short: Some("ClkOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_UP => Some(KeycodeLabel {
            long: Some("Clicky Up".to_string()),
            short: Some("Clk+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_DOWN => Some(KeycodeLabel {
            long: Some("Clicky Down".to_string()),
            short: Some("Clk-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_CLICKY_RESET => Some(KeycodeLabel {
            long: Some("Clicky Reset".to_string()),
            short: Some("ClkRst".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MUSIC_ON => Some(KeycodeLabel {
            long: Some("Music On".to_string()),
            short: Some("MusicOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MUSIC_OFF => Some(KeycodeLabel {
            long: Some("Music Off".to_string()),
            short: Some("MusicOf".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MUSIC_TOGGLE => Some(KeycodeLabel {
            long: Some("Music Toggle".to_string()),
            short: Some("MusicTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MUSIC_MODE_NEXT => Some(KeycodeLabel {
            long: Some("Music Mode".to_string()),
            short: Some("MusicMd".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_VOICE_NEXT => Some(KeycodeLabel {
            long: Some("Voice Next".to_string()),
            short: Some("Voice+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUDIO_VOICE_PREVIOUS => Some(KeycodeLabel {
            long: Some("Voice Prev".to_string()),
            short: Some("Voice-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_STENO_BOLT => Some(KeycodeLabel {
            long: Some("Steno Bolt".to_string()),
            short: Some("StBolt".to_string()),
            ..Default::default()
        }),
        Keycode::QK_STENO_GEMINI => Some(KeycodeLabel {
            long: Some("Steno Gemini".to_string()),
            short: Some("StGem".to_string()),
            ..Default::default()
        }),
        Keycode::QK_STENO_COMB => Some(KeycodeLabel {
            long: Some("Steno Comb".to_string()),
            short: Some("StComb".to_string()),
            ..Default::default()
        }),
        Keycode::QK_STENO_COMB_MAX => Some(KeycodeLabel {
            long: Some("Steno Comb Max".to_string()),
            short: Some("StCMax".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_0 => Some(KeycodeLabel {
            long: Some("Macro 0".to_string()),
            short: Some("M0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_1 => Some(KeycodeLabel {
            long: Some("Macro 1".to_string()),
            short: Some("M1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_2 => Some(KeycodeLabel {
            long: Some("Macro 2".to_string()),
            short: Some("M2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_3 => Some(KeycodeLabel {
            long: Some("Macro 3".to_string()),
            short: Some("M3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_4 => Some(KeycodeLabel {
            long: Some("Macro 4".to_string()),
            short: Some("M4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_5 => Some(KeycodeLabel {
            long: Some("Macro 5".to_string()),
            short: Some("M5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_6 => Some(KeycodeLabel {
            long: Some("Macro 6".to_string()),
            short: Some("M6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_7 => Some(KeycodeLabel {
            long: Some("Macro 7".to_string()),
            short: Some("M7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_8 => Some(KeycodeLabel {
            long: Some("Macro 8".to_string()),
            short: Some("M8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_9 => Some(KeycodeLabel {
            long: Some("Macro 9".to_string()),
            short: Some("M9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_10 => Some(KeycodeLabel {
            long: Some("Macro 10".to_string()),
            short: Some("M10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_11 => Some(KeycodeLabel {
            long: Some("Macro 11".to_string()),
            short: Some("M11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_12 => Some(KeycodeLabel {
            long: Some("Macro 12".to_string()),
            short: Some("M12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_13 => Some(KeycodeLabel {
            long: Some("Macro 13".to_string()),
            short: Some("M13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_14 => Some(KeycodeLabel {
            long: Some("Macro 14".to_string()),
            short: Some("M14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_15 => Some(KeycodeLabel {
            long: Some("Macro 15".to_string()),
            short: Some("M15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_16 => Some(KeycodeLabel {
            long: Some("Macro 16".to_string()),
            short: Some("M16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_17 => Some(KeycodeLabel {
            long: Some("Macro 17".to_string()),
            short: Some("M17".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_18 => Some(KeycodeLabel {
            long: Some("Macro 18".to_string()),
            short: Some("M18".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_19 => Some(KeycodeLabel {
            long: Some("Macro 19".to_string()),
            short: Some("M19".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_20 => Some(KeycodeLabel {
            long: Some("Macro 20".to_string()),
            short: Some("M20".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_21 => Some(KeycodeLabel {
            long: Some("Macro 21".to_string()),
            short: Some("M21".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_22 => Some(KeycodeLabel {
            long: Some("Macro 22".to_string()),
            short: Some("M22".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_23 => Some(KeycodeLabel {
            long: Some("Macro 23".to_string()),
            short: Some("M23".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_24 => Some(KeycodeLabel {
            long: Some("Macro 24".to_string()),
            short: Some("M24".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_25 => Some(KeycodeLabel {
            long: Some("Macro 25".to_string()),
            short: Some("M25".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_26 => Some(KeycodeLabel {
            long: Some("Macro 26".to_string()),
            short: Some("M26".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_27 => Some(KeycodeLabel {
            long: Some("Macro 27".to_string()),
            short: Some("M27".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_28 => Some(KeycodeLabel {
            long: Some("Macro 28".to_string()),
            short: Some("M28".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_29 => Some(KeycodeLabel {
            long: Some("Macro 29".to_string()),
            short: Some("M29".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_30 => Some(KeycodeLabel {
            long: Some("Macro 30".to_string()),
            short: Some("M30".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MACRO_31 => Some(KeycodeLabel {
            long: Some("Macro 31".to_string()),
            short: Some("M31".to_string()),
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_ON => Some(KeycodeLabel {
            long: Some("BL On".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_OFF => Some(KeycodeLabel {
            long: Some("BL Off".to_string()),
            short: Some("BL Off".to_string()),
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_TOGGLE => Some(KeycodeLabel {
            long: Some("BL Toggle".to_string()),
            short: Some("BLTog".to_string()),
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_DOWN => Some(KeycodeLabel {
            long: Some("BL -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_UP => Some(KeycodeLabel {
            long: Some("BL +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_STEP => Some(KeycodeLabel {
            long: Some("BL Cycle".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_BACKLIGHT_TOGGLE_BREATHING => Some(KeycodeLabel {
            long: Some("BR Toggle".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_ON => Some(KeycodeLabel {
            long: Some("LED On".to_string()),
            short: Some("LEDOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_OFF => Some(KeycodeLabel {
            long: Some("LED Off".to_string()),
            short: Some("LEDOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_TOGGLE => Some(KeycodeLabel {
            long: Some("RGB Toggle".to_string()),
            short: Some("RGBTog".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_MODE_NEXT => Some(KeycodeLabel {
            long: Some("RGB Mode +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_MODE_PREVIOUS => Some(KeycodeLabel {
            long: Some("RGB Mode -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_BRIGHTNESS_UP => Some(KeycodeLabel {
            long: Some("LED Bri +".to_string()),
            short: Some("LED+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_BRIGHTNESS_DOWN => Some(KeycodeLabel {
            long: Some("LED Bri -".to_string()),
            short: Some("LED-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_SPEED_UP => Some(KeycodeLabel {
            long: Some("LED Spd +".to_string()),
            short: Some("LEDSp+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LED_MATRIX_SPEED_DOWN => Some(KeycodeLabel {
            long: Some("LED Spd -".to_string()),
            short: Some("LEDSp-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_TOGGLE => Some(KeycodeLabel {
            long: Some("UG Toggle".to_string()),
            short: Some("UGTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_MODE_NEXT => Some(KeycodeLabel {
            long: Some("UG Mode +".to_string()),
            short: Some("UGM+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_MODE_PREVIOUS => Some(KeycodeLabel {
            long: Some("UG Mode -".to_string()),
            short: Some("UGM-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_HUE_UP => Some(KeycodeLabel {
            long: Some("Hue +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_HUE_DOWN => Some(KeycodeLabel {
            long: Some("Hue -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_SATURATION_UP => Some(KeycodeLabel {
            long: Some("Sat +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_SATURATION_DOWN => Some(KeycodeLabel {
            long: Some("Sat -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_VALUE_UP => Some(KeycodeLabel {
            long: Some("Bright +".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_VALUE_DOWN => Some(KeycodeLabel {
            long: Some("Bright -".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_SPEED_UP => Some(KeycodeLabel {
            long: Some("Effect Speed+".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_UNDERGLOW_SPEED_DOWN => Some(KeycodeLabel {
            long: Some("Effect Speed-".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_PLAIN => Some(KeycodeLabel {
            long: Some("RGB Mode P".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_BREATHE => Some(KeycodeLabel {
            long: Some("RGB Mode B".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_RAINBOW => Some(KeycodeLabel {
            long: Some("RGB Mode R".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_SWIRL => Some(KeycodeLabel {
            long: Some("RGB Mode SW".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_SNAKE => Some(KeycodeLabel {
            long: Some("RGB Mode SN".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_KNIGHT => Some(KeycodeLabel {
            long: Some("RGB Mode K".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_XMAS => Some(KeycodeLabel {
            long: Some("RGB Mode X".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_GRADIENT => Some(KeycodeLabel {
            long: Some("RGB Mode G".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_RGBTEST => Some(KeycodeLabel {
            long: Some("RGB Mode Test".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::RGB_MODE_TWINKLE => Some(KeycodeLabel {
            long: Some("RGB Mode T".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_ON => Some(KeycodeLabel {
            long: Some("RGB On".to_string()),
            short: Some("RGBOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_OFF => Some(KeycodeLabel {
            long: Some("RGB Off".to_string()),
            short: Some("RGBOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_TOGGLE => Some(KeycodeLabel {
            long: Some("RGB Toggle".to_string()),
            short: Some("RGBTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_MODE_NEXT => Some(KeycodeLabel {
            long: Some("RGB Mode +".to_string()),
            short: Some("RGBM+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_MODE_PREVIOUS => Some(KeycodeLabel {
            long: Some("RGB Mode -".to_string()),
            short: Some("RGBM-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_HUE_UP => Some(KeycodeLabel {
            long: Some("RGB Hue +".to_string()),
            short: Some("RGBH+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_HUE_DOWN => Some(KeycodeLabel {
            long: Some("RGB Hue -".to_string()),
            short: Some("RGBH-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_SATURATION_UP => Some(KeycodeLabel {
            long: Some("RGB Sat +".to_string()),
            short: Some("RGBS+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_SATURATION_DOWN => Some(KeycodeLabel {
            long: Some("RGB Sat -".to_string()),
            short: Some("RGBS-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_VALUE_UP => Some(KeycodeLabel {
            long: Some("RGB Val +".to_string()),
            short: Some("RGBV+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_VALUE_DOWN => Some(KeycodeLabel {
            long: Some("RGB Val -".to_string()),
            short: Some("RGBV-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_SPEED_UP => Some(KeycodeLabel {
            long: Some("RGB Spd +".to_string()),
            short: Some("RGBSp+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_RGB_MATRIX_SPEED_DOWN => Some(KeycodeLabel {
            long: Some("RGB Spd -".to_string()),
            short: Some("RGBSp-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_BOOTLOADER => Some(KeycodeLabel {
            long: Some("Bootloader".to_string()),
            short: Some("Boot".to_string()),
            ..Default::default()
        }),
        Keycode::QK_REBOOT => Some(KeycodeLabel {
            long: Some("Reboot".to_string()),
            short: Some("Reboot".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DEBUG_TOGGLE => Some(KeycodeLabel {
            long: Some("Debug Toggle".to_string()),
            short: Some("DbgTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_CLEAR_EEPROM => Some(KeycodeLabel {
            long: Some("Clear EEPROM".to_string()),
            short: Some("ClrEE".to_string()),
            ..Default::default()
        }),
        Keycode::QK_MAKE => Some(KeycodeLabel {
            long: Some("Make".to_string()),
            short: Some("Make".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_DOWN => Some(KeycodeLabel {
            long: Some("AutoShift -".to_string()),
            short: Some("AS -".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_UP => Some(KeycodeLabel {
            long: Some("AutoShift +".to_string()),
            short: Some("AS +".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_REPORT => Some(KeycodeLabel {
            long: Some("AutoShift Rep".to_string()),
            short: Some("AS R".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_ON => Some(KeycodeLabel {
            long: Some("AutoShift On".to_string()),
            short: Some("AS On".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_OFF => Some(KeycodeLabel {
            long: Some("AutoShift Off".to_string()),
            short: Some("ASOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTO_SHIFT_TOGGLE => Some(KeycodeLabel {
            long: Some("AutoShift Tog".to_string()),
            short: Some("AS Tg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_GRAVE_ESCAPE => Some(KeycodeLabel {
            long: Some("Esc `".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_VELOCIKEY_TOGGLE => Some(KeycodeLabel {
            long: Some("Velocikey".to_string()),
            short: Some("VelKey".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_LEFT_CTRL_PARENTHESIS_OPEN => Some(KeycodeLabel {
            long: Some("LC (".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_RIGHT_CTRL_PARENTHESIS_CLOSE => Some(KeycodeLabel {
            long: Some("RC )".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_LEFT_SHIFT_PARENTHESIS_OPEN => Some(KeycodeLabel {
            long: Some("LS (".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_RIGHT_SHIFT_PARENTHESIS_CLOSE => Some(KeycodeLabel {
            long: Some("RS )".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_LEFT_ALT_PARENTHESIS_OPEN => Some(KeycodeLabel {
            long: Some("LA (".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_RIGHT_ALT_PARENTHESIS_CLOSE => Some(KeycodeLabel {
            long: Some("RA )".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_SPACE_CADET_RIGHT_SHIFT_ENTER => Some(KeycodeLabel {
            long: Some("SftEnt".to_string()),
            short: None,
            ..Default::default()
        }),
        Keycode::QK_OUTPUT_AUTO => Some(KeycodeLabel {
            long: Some("Out Auto".to_string()),
            short: Some("OutAuto".to_string()),
            ..Default::default()
        }),
        Keycode::QK_OUTPUT_USB => Some(KeycodeLabel {
            long: Some("Out USB".to_string()),
            short: Some("OutUSB".to_string()),
            ..Default::default()
        }),
        Keycode::QK_OUTPUT_BLUETOOTH => Some(KeycodeLabel {
            long: Some("Out BT".to_string()),
            short: Some("OutBT".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_NEXT => Some(KeycodeLabel {
            long: Some("Unicode +".to_string()),
            short: Some("Uni +".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_PREVIOUS => Some(KeycodeLabel {
            long: Some("Unicode -".to_string()),
            short: Some("Uni -".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_MACOS => Some(KeycodeLabel {
            long: Some("Unicode macOS".to_string()),
            short: Some("UniMac".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_LINUX => Some(KeycodeLabel {
            long: Some("Unicode Linux".to_string()),
            short: Some("UniLnx".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_WINDOWS => Some(KeycodeLabel {
            long: Some("Unicode Win".to_string()),
            short: Some("UniWin".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_BSD => Some(KeycodeLabel {
            long: Some("Unicode BSD".to_string()),
            short: Some("UniBSD".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_WINCOMPOSE => Some(KeycodeLabel {
            long: Some("Unicode WinC".to_string()),
            short: Some("UniWinC".to_string()),
            ..Default::default()
        }),
        Keycode::QK_UNICODE_MODE_EMACS => Some(KeycodeLabel {
            long: Some("Unicode Emacs".to_string()),
            short: Some("UniEm".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_ON => Some(KeycodeLabel {
            long: Some("Haptic On".to_string()),
            short: Some("HapOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_OFF => Some(KeycodeLabel {
            long: Some("Haptic Off".to_string()),
            short: Some("HapOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_TOGGLE => Some(KeycodeLabel {
            long: Some("Haptic Toggle".to_string()),
            short: Some("HapTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_RESET => Some(KeycodeLabel {
            long: Some("Haptic Reset".to_string()),
            short: Some("HapRst".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_FEEDBACK_TOGGLE => Some(KeycodeLabel {
            long: Some("Haptic FB Tog".to_string()),
            short: Some("HapFBTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_BUZZ_TOGGLE => Some(KeycodeLabel {
            long: Some("Haptic Buzz".to_string()),
            short: Some("HapBuzz".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_MODE_NEXT => Some(KeycodeLabel {
            long: Some("Haptic +".to_string()),
            short: Some("Hap +".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_MODE_PREVIOUS => Some(KeycodeLabel {
            long: Some("Haptic -".to_string()),
            short: Some("Hap -".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_CONTINUOUS_TOGGLE => Some(KeycodeLabel {
            long: Some("Haptic Cont".to_string()),
            short: Some("HapCont".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_CONTINUOUS_UP => Some(KeycodeLabel {
            long: Some("Haptic + ".to_string()),
            short: Some("HapC+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_CONTINUOUS_DOWN => Some(KeycodeLabel {
            long: Some("Haptic -".to_string()),
            short: Some("HapC-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_DWELL_UP => Some(KeycodeLabel {
            long: Some("Haptic Dwell +".to_string()),
            short: Some("HapDw+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_HAPTIC_DWELL_DOWN => Some(KeycodeLabel {
            long: Some("Haptic Dwell -".to_string()),
            short: Some("HapDw-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_COMBO_ON => Some(KeycodeLabel {
            long: Some("Combo On".to_string()),
            short: Some("CombOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_COMBO_OFF => Some(KeycodeLabel {
            long: Some("Combo Off".to_string()),
            short: Some("CombOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_COMBO_TOGGLE => Some(KeycodeLabel {
            long: Some("Combo Toggle".to_string()),
            short: Some("CombTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_MACRO_RECORD_START_1 => Some(KeycodeLabel {
            long: Some("DM Rec 1".to_string()),
            short: Some("DMRec1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_MACRO_RECORD_START_2 => Some(KeycodeLabel {
            long: Some("DM Rec 2".to_string()),
            short: Some("DMRec2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_MACRO_RECORD_STOP => Some(KeycodeLabel {
            long: Some("DM Stop".to_string()),
            short: Some("DMStop".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_MACRO_PLAY_1 => Some(KeycodeLabel {
            long: Some("DM Play 1".to_string()),
            short: Some("DMPlay1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_MACRO_PLAY_2 => Some(KeycodeLabel {
            long: Some("DM Play 2".to_string()),
            short: Some("DMPlay2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LEADER => Some(KeycodeLabel {
            long: Some("Leader".to_string()),
            short: Some("Lead".to_string()),
            ..Default::default()
        }),
        Keycode::QK_LOCK => Some(KeycodeLabel {
            long: Some("Lock".to_string()),
            short: Some("Lock".to_string()),
            ..Default::default()
        }),
        Keycode::QK_ONE_SHOT_ON => Some(KeycodeLabel {
            long: Some("OneShot On".to_string()),
            short: Some("1ShotOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_ONE_SHOT_OFF => Some(KeycodeLabel {
            long: Some("OneShot Off".to_string()),
            short: Some("1ShotOf".to_string()),
            ..Default::default()
        }),
        Keycode::QK_ONE_SHOT_TOGGLE => Some(KeycodeLabel {
            long: Some("OneShot Toggle".to_string()),
            short: Some("1ShotTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KEY_OVERRIDE_TOGGLE => Some(KeycodeLabel {
            long: Some("KO Toggle".to_string()),
            short: Some("KOTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KEY_OVERRIDE_ON => Some(KeycodeLabel {
            long: Some("KO On".to_string()),
            short: Some("KOOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KEY_OVERRIDE_OFF => Some(KeycodeLabel {
            long: Some("KO Off".to_string()),
            short: Some("KOOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SECURE_LOCK => Some(KeycodeLabel {
            long: Some("Secure Lock".to_string()),
            short: Some("SecLock".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SECURE_UNLOCK => Some(KeycodeLabel {
            long: Some("Secure Unlock".to_string()),
            short: Some("SecUnlk".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SECURE_TOGGLE => Some(KeycodeLabel {
            long: Some("Secure Toggle".to_string()),
            short: Some("SecTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_SECURE_REQUEST => Some(KeycodeLabel {
            long: Some("Secure Request".to_string()),
            short: Some("SecReq".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_TAPPING_TERM_PRINT => Some(KeycodeLabel {
            long: Some("DT Term".to_string()),
            short: Some("DTTerm".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_TAPPING_TERM_UP => Some(KeycodeLabel {
            long: Some("DT Term +".to_string()),
            short: Some("DTTerm+".to_string()),
            ..Default::default()
        }),
        Keycode::QK_DYNAMIC_TAPPING_TERM_DOWN => Some(KeycodeLabel {
            long: Some("DT Term -".to_string()),
            short: Some("DTTerm-".to_string()),
            ..Default::default()
        }),
        Keycode::QK_CAPS_WORD_TOGGLE => Some(KeycodeLabel {
            long: Some("Caps Word".to_string()),
            short: Some("CapWord".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTOCORRECT_ON => Some(KeycodeLabel {
            long: Some("Autocorrect On".to_string()),
            short: Some("ACOn".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTOCORRECT_OFF => Some(KeycodeLabel {
            long: Some("Autocorrect Off".to_string()),
            short: Some("ACOff".to_string()),
            ..Default::default()
        }),
        Keycode::QK_AUTOCORRECT_TOGGLE => Some(KeycodeLabel {
            long: Some("Autocorrect Tog".to_string()),
            short: Some("ACTg".to_string()),
            ..Default::default()
        }),
        Keycode::QK_TRI_LAYER_LOWER => Some(KeycodeLabel {
            long: Some("Tri Lower".to_string()),
            short: Some("TriLow".to_string()),
            ..Default::default()
        }),
        Keycode::QK_TRI_LAYER_UPPER => Some(KeycodeLabel {
            long: Some("Tri Upper".to_string()),
            short: Some("TriUp".to_string()),
            ..Default::default()
        }),
        Keycode::QK_REPEAT_KEY => Some(KeycodeLabel {
            long: Some("Repeat Key".to_string()),
            short: Some("RepKey".to_string()),
            ..Default::default()
        }),
        Keycode::QK_ALT_REPEAT_KEY => Some(KeycodeLabel {
            long: Some("Alt Repeat".to_string()),
            short: Some("ARepKey".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_0 => Some(KeycodeLabel {
            long: Some("KB 0".to_string()),
            short: Some("KB0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_1 => Some(KeycodeLabel {
            long: Some("KB 1".to_string()),
            short: Some("KB1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_2 => Some(KeycodeLabel {
            long: Some("KB 2".to_string()),
            short: Some("KB2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_3 => Some(KeycodeLabel {
            long: Some("KB 3".to_string()),
            short: Some("KB3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_4 => Some(KeycodeLabel {
            long: Some("KB 4".to_string()),
            short: Some("KB4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_5 => Some(KeycodeLabel {
            long: Some("KB 5".to_string()),
            short: Some("KB5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_6 => Some(KeycodeLabel {
            long: Some("KB 6".to_string()),
            short: Some("KB6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_7 => Some(KeycodeLabel {
            long: Some("KB 7".to_string()),
            short: Some("KB7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_8 => Some(KeycodeLabel {
            long: Some("KB 8".to_string()),
            short: Some("KB8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_9 => Some(KeycodeLabel {
            long: Some("KB 9".to_string()),
            short: Some("KB9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_10 => Some(KeycodeLabel {
            long: Some("KB 10".to_string()),
            short: Some("KB10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_11 => Some(KeycodeLabel {
            long: Some("KB 11".to_string()),
            short: Some("KB11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_12 => Some(KeycodeLabel {
            long: Some("KB 12".to_string()),
            short: Some("KB12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_13 => Some(KeycodeLabel {
            long: Some("KB 13".to_string()),
            short: Some("KB13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_14 => Some(KeycodeLabel {
            long: Some("KB 14".to_string()),
            short: Some("KB14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_15 => Some(KeycodeLabel {
            long: Some("KB 15".to_string()),
            short: Some("KB15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_16 => Some(KeycodeLabel {
            long: Some("KB 16".to_string()),
            short: Some("KB16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_17 => Some(KeycodeLabel {
            long: Some("KB 17".to_string()),
            short: Some("KB17".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_18 => Some(KeycodeLabel {
            long: Some("KB 18".to_string()),
            short: Some("KB18".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_19 => Some(KeycodeLabel {
            long: Some("KB 19".to_string()),
            short: Some("KB19".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_20 => Some(KeycodeLabel {
            long: Some("KB 20".to_string()),
            short: Some("KB20".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_21 => Some(KeycodeLabel {
            long: Some("KB 21".to_string()),
            short: Some("KB21".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_22 => Some(KeycodeLabel {
            long: Some("KB 22".to_string()),
            short: Some("KB22".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_23 => Some(KeycodeLabel {
            long: Some("KB 23".to_string()),
            short: Some("KB23".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_24 => Some(KeycodeLabel {
            long: Some("KB 24".to_string()),
            short: Some("KB24".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_25 => Some(KeycodeLabel {
            long: Some("KB 25".to_string()),
            short: Some("KB25".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_26 => Some(KeycodeLabel {
            long: Some("KB 26".to_string()),
            short: Some("KB26".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_27 => Some(KeycodeLabel {
            long: Some("KB 27".to_string()),
            short: Some("KB27".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_28 => Some(KeycodeLabel {
            long: Some("KB 28".to_string()),
            short: Some("KB28".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_29 => Some(KeycodeLabel {
            long: Some("KB 29".to_string()),
            short: Some("KB29".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_30 => Some(KeycodeLabel {
            long: Some("KB 30".to_string()),
            short: Some("KB30".to_string()),
            ..Default::default()
        }),
        Keycode::QK_KB_31 => Some(KeycodeLabel {
            long: Some("KB 31".to_string()),
            short: Some("KB31".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_0 => Some(KeycodeLabel {
            long: Some("User 0".to_string()),
            short: Some("Usr0".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_1 => Some(KeycodeLabel {
            long: Some("User 1".to_string()),
            short: Some("Usr1".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_2 => Some(KeycodeLabel {
            long: Some("User 2".to_string()),
            short: Some("Usr2".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_3 => Some(KeycodeLabel {
            long: Some("User 3".to_string()),
            short: Some("Usr3".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_4 => Some(KeycodeLabel {
            long: Some("User 4".to_string()),
            short: Some("Usr4".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_5 => Some(KeycodeLabel {
            long: Some("User 5".to_string()),
            short: Some("Usr5".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_6 => Some(KeycodeLabel {
            long: Some("User 6".to_string()),
            short: Some("Usr6".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_7 => Some(KeycodeLabel {
            long: Some("User 7".to_string()),
            short: Some("Usr7".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_8 => Some(KeycodeLabel {
            long: Some("User 8".to_string()),
            short: Some("Usr8".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_9 => Some(KeycodeLabel {
            long: Some("User 9".to_string()),
            short: Some("Usr9".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_10 => Some(KeycodeLabel {
            long: Some("User 10".to_string()),
            short: Some("Usr10".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_11 => Some(KeycodeLabel {
            long: Some("User 11".to_string()),
            short: Some("Usr11".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_12 => Some(KeycodeLabel {
            long: Some("User 12".to_string()),
            short: Some("Usr12".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_13 => Some(KeycodeLabel {
            long: Some("User 13".to_string()),
            short: Some("Usr13".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_14 => Some(KeycodeLabel {
            long: Some("User 14".to_string()),
            short: Some("Usr14".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_15 => Some(KeycodeLabel {
            long: Some("User 15".to_string()),
            short: Some("Usr15".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_16 => Some(KeycodeLabel {
            long: Some("User 16".to_string()),
            short: Some("Usr16".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_17 => Some(KeycodeLabel {
            long: Some("User 17".to_string()),
            short: Some("Usr17".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_18 => Some(KeycodeLabel {
            long: Some("User 18".to_string()),
            short: Some("Usr18".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_19 => Some(KeycodeLabel {
            long: Some("User 19".to_string()),
            short: Some("Usr19".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_20 => Some(KeycodeLabel {
            long: Some("User 20".to_string()),
            short: Some("Usr20".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_21 => Some(KeycodeLabel {
            long: Some("User 21".to_string()),
            short: Some("Usr21".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_22 => Some(KeycodeLabel {
            long: Some("User 22".to_string()),
            short: Some("Usr22".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_23 => Some(KeycodeLabel {
            long: Some("User 23".to_string()),
            short: Some("Usr23".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_24 => Some(KeycodeLabel {
            long: Some("User 24".to_string()),
            short: Some("Usr24".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_25 => Some(KeycodeLabel {
            long: Some("User 25".to_string()),
            short: Some("Usr25".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_26 => Some(KeycodeLabel {
            long: Some("User 26".to_string()),
            short: Some("Usr26".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_27 => Some(KeycodeLabel {
            long: Some("User 27".to_string()),
            short: Some("Usr27".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_28 => Some(KeycodeLabel {
            long: Some("User 28".to_string()),
            short: Some("Usr28".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_29 => Some(KeycodeLabel {
            long: Some("User 29".to_string()),
            short: Some("Usr29".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_30 => Some(KeycodeLabel {
            long: Some("User 30".to_string()),
            short: Some("Usr30".to_string()),
            ..Default::default()
        }),
        Keycode::QK_USER_31 => Some(KeycodeLabel {
            long: Some("User 31".to_string()),
            short: Some("Usr31".to_string()),
            ..Default::default()
        }),
    }
}

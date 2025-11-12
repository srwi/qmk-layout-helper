use crate::keycode_labels::constants::*;
use crate::keycode_labels::keycode_label::KeycodeLabel;

pub fn get_layer_keycode_label(keycode_bytes: u16) -> Option<KeycodeLabel> {
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

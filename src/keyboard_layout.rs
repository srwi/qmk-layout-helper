use qmk_via_api::api;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct Key {
    pub row: api::Row,
    pub col: api::Column,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone)]
pub struct KeyboardInfo {
    pub vid: u16,
    pub pid: u16,
    pub rows: usize,
    pub cols: usize,
    pub keys: Vec<Key>,
}

impl KeyboardInfo {
    pub fn new(json_path: &str, layout_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_path).expect("Failed to open keyboard info JSON file.");
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        let layout = if json["layouts"].get(layout_name).is_some() {
            &json["layouts"][layout_name]["layout"]
        } else if let Some(alias) = json["layout_aliases"].get(layout_name) {
            let alias_name = alias.as_str().ok_or("Invalid alias format")?;
            &json["layouts"][alias_name]["layout"]
        } else {
            return Err(format!("Layout '{}' not found and no matching alias", layout_name).into());
        };

        let mut keys = Vec::new();
        for key in layout.as_array().unwrap() {
            let matrix: Vec<usize> = key["matrix"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as usize)
                .collect();

            let x = key["x"].as_f64().unwrap_or(0.0) as f32;
            let y = key["y"].as_f64().unwrap_or(0.0) as f32;
            let w = key["w"].as_f64().unwrap_or(1.0) as f32;
            let h = key["h"].as_f64().unwrap_or(1.0) as f32;

            keys.push(Key {
                row: matrix[0] as api::Row,
                col: matrix[1] as api::Layer,
                x,
                y,
                w,
                h,
            });
        }

        let is_split_keyboard = json
            .get("split")
            .unwrap_or_default()
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let row_multiplier = if is_split_keyboard { 2 } else { 1 };
        let rows = json["matrix_pins"]
            .get("rows")
            .unwrap()
            .as_array()
            .unwrap()
            .len()
            * row_multiplier;
        let cols = json["matrix_pins"]
            .get("cols")
            .unwrap()
            .as_array()
            .unwrap()
            .len();

        let vid = Self::hex_to_u16(json["usb"].get("vid").unwrap().as_str().unwrap()).unwrap();
        let pid = Self::hex_to_u16(json["usb"].get("pid").unwrap().as_str().unwrap()).unwrap();

        Ok(KeyboardInfo {
            vid,
            pid,
            rows,
            cols,
            keys,
        })
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        let max_x = self.keys.iter().map(|k| k.x + k.w).fold(0.0, f32::max);
        let max_y = self.keys.iter().map(|k| k.y + k.h).fold(0.0, f32::max);
        (max_x, max_y)
    }

    fn hex_to_u16(hex_string: &str) -> Result<u16, ParseIntError> {
        let cleaned_hex = hex_string.trim_start_matches("0x");
        u16::from_str_radix(cleaned_hex, 16)
    }
}

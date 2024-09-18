use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Key {
    pub label: String,
    pub matrix: (usize, usize),
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub struct KeyboardLayout {
    pub keys: Vec<Key>,
}

impl KeyboardLayout {
    pub fn new(json_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(json_path)?;
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        let mut keys = Vec::new();
        let layout = &json["layouts"]["LAYOUT"]["layout"];

        for key in layout.as_array().unwrap() {
            let matrix: Vec<usize> = key["matrix"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as usize)
                .collect();

            let label = key["label"].as_str().unwrap_or("").to_string();
            let x = key["x"].as_f64().unwrap_or(0.0) as f32;
            let y = key["y"].as_f64().unwrap_or(0.0) as f32;
            let w = key["w"].as_f64().unwrap_or(1.0) as f32;
            let h = key["h"].as_f64().unwrap_or(1.0) as f32;

            keys.push(Key {
                label,
                matrix: (matrix[0], matrix[1]),
                x,
                y,
                w,
                h,
            });
        }

        Ok(KeyboardLayout { keys })
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        let max_x = self.keys.iter().map(|k| k.x + k.w).fold(0.0, f32::max);
        let max_y = self.keys.iter().map(|k| k.y + k.h).fold(0.0, f32::max);
        (max_x, max_y)
    }
}
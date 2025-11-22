pub struct KeyMatrix {
    pub keycodes: Vec<Vec<Vec<u16>>>,
    pub pressed: Vec<Vec<bool>>,
}

impl KeyMatrix {
    pub fn new(keycodes: Vec<Vec<Vec<u16>>>, rows: usize, cols: usize) -> Self {
        KeyMatrix {
            keycodes,
            pressed: vec![vec![false; cols]; rows],
        }
    }

    pub fn get_num_layers(&self) -> usize {
        self.keycodes.len()
    }

    pub fn get_keycode(&self, layer: usize, row: usize, col: usize) -> u16 {
        self.keycodes
            .get(layer)
            .and_then(|r| r.get(row))
            .and_then(|c| c.get(col))
            .copied()
            .unwrap_or(0)
    }

    pub fn is_pressed(&self, row: usize, col: usize) -> bool {
        self.pressed
            .get(row)
            .and_then(|r| r.get(col))
            .copied()
            .unwrap_or(false)
    }

    pub fn set_pressed(&mut self, row: usize, col: usize, value: bool) {
        if let Some(r) = self.pressed.get_mut(row) {
            if col < r.len() {
                r[col] = value;
            }
        }
    }
}

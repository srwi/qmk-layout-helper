use crate::keyboard_layout::KeyboardInfo;
use qmk_via_api::api::{self};
use qmk_via_api::keycodes::Keycode;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Keyboard {
    pub keyboard_info: KeyboardInfo,
    pub matrix: Vec<Vec<Vec<u16>>>,
    pub time_to_hide_overlay: Arc<Mutex<Option<Instant>>>,
    layer_state: Arc<Mutex<u32>>,
    default_layer_state: Arc<Mutex<u32>>,
}

impl Keyboard {
    pub fn new(keyboard_info: KeyboardInfo, timeout: u64) -> Self {
        let api = api::KeyboardApi::new(keyboard_info.vid, keyboard_info.pid, 0xff60)
            .expect("Failed to connect to device.");

        let layers = api.get_layer_count().expect("Failed to get layer count") as usize;

        let matrix =
            Self::get_keycodes_from_device(&api, layers, keyboard_info.rows, keyboard_info.cols);

        let layer_state = Arc::new(Mutex::new(0));
        let default_layer_state = Arc::new(Mutex::new(0));
        let time_to_hide_overlay = Arc::new(Mutex::new(Some(Instant::now())));

        let keyboard = Keyboard {
            keyboard_info,
            matrix,
            time_to_hide_overlay: Arc::clone(&time_to_hide_overlay),
            layer_state: Arc::clone(&layer_state),
            default_layer_state: Arc::clone(&default_layer_state),
        };

        let layer_state_clone = Arc::clone(&keyboard.layer_state);
        let default_layer_state_clone = Arc::clone(&keyboard.default_layer_state);
        let time_to_hide_clone = Arc::clone(&keyboard.time_to_hide_overlay);

        thread::spawn(move || loop {
            if let Ok(response) = api.hid_read() {
                if response[0] == 0xff {
                    let size = response[1] as usize;

                    let mut default_bytes = [0u8; 4];
                    default_bytes[..size].copy_from_slice(&response[2..2 + size]);
                    let default_layer_state = u32::from_le_bytes(default_bytes);

                    let mut layer_bytes = [0u8; 4];
                    layer_bytes[..size].copy_from_slice(&response[2 + size..2 + 2 * size]);
                    let layer_state = u32::from_le_bytes(layer_bytes);

                    if layer_state > 1 {
                        *time_to_hide_clone.lock().unwrap() = None;
                    } else {
                        let time_to_hide = Instant::now() + Duration::from_millis(timeout);
                        *time_to_hide_clone.lock().unwrap() = Some(time_to_hide);
                    }

                    *layer_state_clone.lock().unwrap() = layer_state;
                    *default_layer_state_clone.lock().unwrap() = default_layer_state;
                }
            }
        });

        keyboard
    }

    fn get_keycodes_from_device(
        api: &api::KeyboardApi,
        layers: usize,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<Vec<u16>>> {
        let mut keycodes = vec![vec![vec![0; cols]; rows]; layers];
        let matrix_info = api::MatrixInfo {
            rows: rows as u8,
            cols: cols as u8,
        };

        for layer in 0..layers {
            if let Ok(raw_matrix) = api.read_raw_matrix(matrix_info, layer as u8) {
                for (i, keycode) in raw_matrix.iter().enumerate() {
                    let row = i / cols;
                    let col = i % cols;
                    keycodes[layer][row][col] = *keycode;
                }
            }
        }

        keycodes
    }

    pub fn get_effective_key_layer(&self, row: usize, col: usize) -> (u8, bool) {
        let layer_state = *self.layer_state.lock().unwrap();
        let default_layer_state = *self.default_layer_state.lock().unwrap();
        let num_layers = self.matrix.len().min(32);

        for i in (1..num_layers).rev() {
            let layer_mask = 1u32 << (i as u32);
            let is_default_layer = (default_layer_state & layer_mask) != 0;
            let is_active_layer = (layer_state & layer_mask) != 0;
            if is_active_layer || is_default_layer {
                if self.matrix[i][row][col] != Keycode::KC_TRANSPARENT as u16 {
                    return (i as u8, is_default_layer);
                }
            }
        }

        // The first layer is always the default fallback
        (0, true)
    }
}

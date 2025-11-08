use crate::keyboard_layout::KeyboardInfo;
use qmk_via_api::api::{self};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Keyboard {
    pub keyboard_info: KeyboardInfo,
    pub matrix: Vec<Vec<Vec<u16>>>,
    pub time_to_hide_overlay: Arc<Mutex<Option<Instant>>>,
    layer_state: Arc<Mutex<u32>>,
}

impl Keyboard {
    pub fn new(keyboard_info: KeyboardInfo, timeout: u64) -> Self {
        let api = api::KeyboardApi::new(keyboard_info.vid, keyboard_info.pid, 0xff60)
            .expect("Failed to connect to device.");

        let layers = api.get_layer_count().expect("Failed to get layer count") as usize;

        let matrix =
            Self::get_keycodes_from_device(&api, layers, keyboard_info.rows, keyboard_info.cols);

        let layer_state = Arc::new(Mutex::new(0));
        let time_to_hide_overlay = Arc::new(Mutex::new(Some(Instant::now())));

        let keyboard = Keyboard {
            keyboard_info,
            matrix,
            time_to_hide_overlay: Arc::clone(&time_to_hide_overlay),
            layer_state: Arc::clone(&layer_state),
        };

        let layer_state_clone = Arc::clone(&keyboard.layer_state);
        let time_to_hide_clone = Arc::clone(&keyboard.time_to_hide_overlay);

        thread::spawn(move || loop {
            if let Ok(response) = api.hid_read() {
                if response[0] == 0x01 {
                    let new_layer_state = u32::from_le_bytes(response[1..5].try_into().unwrap());
                    *layer_state_clone.lock().unwrap() = new_layer_state;

                    if new_layer_state <= 1 {
                        *time_to_hide_clone.lock().unwrap() =
                            Some(Instant::now() + Duration::from_millis(timeout));
                    } else {
                        *time_to_hide_clone.lock().unwrap() = None;
                    }
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

    pub fn get_active_layers(&self) -> Vec<u8> {
        let state = *self.layer_state.lock().unwrap();
        let mut active = Vec::new();
        let max_layers = self.matrix.len().min(32);
        for i in (0..max_layers).rev() {
            if (state & (1u32 << i)) != 0 {
                active.push(i as u8);
            }
        }
        active
    }
}

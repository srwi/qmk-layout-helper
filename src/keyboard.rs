use crate::keyboard_layout::KeyboardInfo;
use qmk_via_api::api::{self};

pub struct Keyboard {
    pub keyboard_info: KeyboardInfo,
    pub matrix: Vec<Vec<Vec<u16>>>,
}

impl Keyboard {
    pub fn new(keyboard_info: KeyboardInfo) -> Self {
        let api = api::KeyboardApi::new(keyboard_info.vid, keyboard_info.pid, 0xff60)
            .expect("Failed to connect to device.");

        let layers = api.get_layer_count().unwrap() as usize;

        let matrix =
            Self::get_keycodes_from_device(&api, layers, keyboard_info.rows, keyboard_info.cols);

        Keyboard {
            keyboard_info,
            matrix,
        }
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
            if let Some(raw_matrix) = api.read_raw_matrix(matrix_info, layer as u8) {
                for (i, keycode) in raw_matrix.iter().enumerate() {
                    let row = i / cols;
                    let col = i % cols;
                    keycodes[layer][row][col] = *keycode;
                }
            }
        }

        keycodes
    }
}

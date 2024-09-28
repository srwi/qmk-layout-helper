use crate::keyboard_layout::KeyboardInfo;
use qmk_via_api::api;

pub struct Keyboard {
    pub keyboard_info: KeyboardInfo,
    pub matrix: Vec<Vec<Vec<u16>>>,
}

impl Keyboard {
    pub fn new(keyboard_info: KeyboardInfo) -> Self {
        let api = api::KeyboardApi::new(keyboard_info.vid, keyboard_info.pid, 0xff60);

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

        // TODO: Replace with api::read_raw_matrix
        for layer in 0..layers {
            for row in 0..rows {
                for col in 0..cols {
                    keycodes[layer][row][col] = api
                        .get_key(layer as u8, row as u8, col as u8)
                        .unwrap_or_default()
                }
            }
        }

        keycodes
    }
}

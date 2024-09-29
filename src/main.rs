// use crate::egui::ViewportCommand;
use eframe::egui::Galley;
use eframe::egui::{self};
mod keyboard;
mod keyboard_layout;
mod keycode_label;
use clap::Parser;
use keyboard::Keyboard;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

struct Overlay {
    keyboard: Keyboard,
    current_layer: Arc<Mutex<u8>>,
}

impl Overlay {
    fn new(keyboard: Keyboard) -> Self {
        let current_layer = Arc::new(Mutex::new(0));
        let api = qmk_via_api::api::KeyboardApi::new(
            keyboard.keyboard_info.vid,
            keyboard.keyboard_info.pid,
            0xff60,
        );
        let layer_clone = Arc::clone(&current_layer);

        thread::spawn(move || loop {
            if let Some(response) = api.hid_read() {
                if response[0] == 0x01 {
                    let new_layer = response[1];
                    let mut layer = layer_clone.lock().unwrap();
                    *layer = new_layer;
                }
            }
        });

        Self {
            keyboard,
            current_layer,
        }
    }

    fn calculate_unit_size(&self, available_width: f32, available_height: f32) -> f32 {
        let (layout_width, layout_height) = self.keyboard.keyboard_info.get_dimensions();
        let width_ratio = available_width / layout_width;
        let height_ratio = available_height / layout_height;
        width_ratio.min(height_ratio)
    }

    fn generate_key_label_galley(
        &self,
        ui: &egui::Ui,
        keycode_label: keycode_label::KeycodeLabel,
        rect: egui::Rect,
        font: egui::FontId,
    ) -> Option<std::sync::Arc<Galley>> {
        let create_galley = |text: String| {
            ui.painter()
                .layout_no_wrap(text, font.clone(), egui::Color32::WHITE)
        };
        let galley_fits =
            |galley: &std::sync::Arc<Galley>| galley.rect.width() <= rect.width() * 0.85;

        let long_label = keycode_label.long.unwrap_or_default();

        let full_galley = create_galley(long_label.clone());
        if galley_fits(&full_galley) {
            return Some(full_galley);
        }

        let mut truncated = if keycode_label.short.is_some() {
            let short_label = keycode_label.short.unwrap_or_default();
            let short_galley = create_galley(short_label.clone());
            if galley_fits(&short_galley) {
                return Some(short_galley);
            }
            short_label
        } else {
            long_label
        };

        while truncated.len() > 1 {
            truncated.pop();
            let truncated_with_ellipsis = format!("{}...", truncated);
            let truncated_galley = create_galley(truncated_with_ellipsis);
            if galley_fits(&truncated_galley) {
                return Some(truncated_galley);
            }
        }

        None
    }
}

impl eframe::App for Overlay {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));

        let frame = egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let layer = *self.current_layer.lock().unwrap();

            let available_rect = ui.available_size();
            let unit_size = self.calculate_unit_size(available_rect.x, available_rect.y);

            for key in &self.keyboard.keyboard_info.keys {
                let rect = egui::Rect::from_min_size(
                    egui::pos2(key.x * unit_size, key.y * unit_size),
                    egui::vec2(key.w * unit_size, key.h * unit_size),
                )
                .shrink(0.05 * unit_size);

                ui.painter().rect(
                    rect,
                    0.12 * unit_size,
                    egui::Color32::from_rgba_unmultiplied(150, 150, 150, 225),
                    egui::Stroke::NONE,
                );

                let font = egui::FontId::proportional(0.3 * unit_size);

                let keycode =
                    self.keyboard.matrix[layer as usize][key.row as usize][key.col as usize];
                let keycode_label = keycode_label::get_keycode_label(keycode);

                if let Some(label_galley) =
                    self.generate_key_label_galley(ui, keycode_label, rect, font)
                {
                    let label_pos = rect.center() - label_galley.rect.center().to_vec2();
                    ui.painter()
                        .galley(label_pos, label_galley, egui::Color32::WHITE);
                }
            }
        });

        // TODO: make this on demand
        ctx.request_repaint();
    }
}

#[derive(Parser)]
struct Cli {
    keyboard_config: PathBuf,

    #[arg(short = 'l', long = "layout", default_value = "LAYOUT")]
    layout_name: String,
}

fn main() -> Result<(), eframe::Error> {
    let cli = Cli::parse();

    let keyboard_config = cli.keyboard_config.to_str().expect("Invalid path");
    let layout_name = &cli.layout_name;

    let keyboard_info = keyboard_layout::KeyboardInfo::new(&keyboard_config, &layout_name)
        .expect("Failed to read keyboard layout.");
    let keyboard = Keyboard::new(keyboard_info);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 240.0])
            // .with_decorations(false)
            // .with_taskbar(false)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "QMK Layout Helper",
        options,
        Box::new(|_cc| Ok(Box::<Overlay>::new(Overlay::new(keyboard)))),
    )
}

use std::{cmp::min, ffi::c_double};

use crate::egui::ViewportCommand;
use eframe::egui::{self, debug_text::print, TextBuffer};
use keyboard_layout::KeyboardInfo;
mod keyboard;
mod keyboard_layout;
use clap::Parser;
use keyboard::Keyboard;
use std::path::PathBuf;

struct Overlay {
    keyboard: Keyboard,
}

impl Overlay {
    fn new(keyboard: Keyboard) -> Self {
        Self { keyboard }
    }

    fn calculate_unit_size(&self, available_width: f32, available_height: f32) -> f32 {
        let (layout_width, layout_height) = self.keyboard.keyboard_info.get_dimensions();
        let width_ratio = available_width / layout_width;
        let height_ratio = available_height / layout_height;
        width_ratio.min(height_ratio)
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
                    egui::Color32::from_rgba_unmultiplied(150, 150, 150, 170),
                    egui::Stroke::NONE,
                );

                let font = egui::FontId::proportional(0.25 * unit_size);
                let text = self.keyboard.matrix[0][key.row as usize][key.col as usize].as_ref();

                let galley = ui.painter().layout_no_wrap(
                    text.to_string(),
                    font.clone(),
                    egui::Color32::WHITE,
                );

                let truncated_text = if galley.rect.width() > rect.width() {
                    // TODO: Later this should be replaced with the short key code display name
                    let mut truncated = text.to_string();
                    while truncated.len() > 1 {
                        truncated.pop();
                        let temp_galley = ui.painter().layout_no_wrap(
                            format!("{}...", truncated),
                            font.clone(),
                            egui::Color32::WHITE,
                        );
                        if temp_galley.rect.width() <= rect.width() {
                            break;
                        }
                    }
                    format!("{}...", truncated)
                } else {
                    text.to_string()
                };

                let final_galley =
                    ui.painter()
                        .layout_no_wrap(truncated_text, font, egui::Color32::WHITE);

                let text_pos = rect.center() - final_galley.rect.center().to_vec2();

                ui.painter()
                    .galley(text_pos, final_galley, egui::Color32::WHITE);
            }
        });
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

    let keyboard_info =
        KeyboardInfo::new(&keyboard_config, &layout_name).expect("Failed to read keyboard layout.");
    let keyboard = Keyboard::new(keyboard_info.clone());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 240.0])
            // .with_decorations(false)
            // .with_taskbar(false)
            // .with_mouse_passthrough(true)
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

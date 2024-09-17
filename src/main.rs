use std::{cmp::min, ffi::c_double};

use eframe::egui;
use keyboard_layout::KeyboardLayout;
use crate::egui::ViewportCommand;
mod keyboard_layout;

struct Overlay {
    keyboard_layout: KeyboardLayout,
}

impl Overlay {
    fn new(keyboard_layout: KeyboardLayout) -> Self {
        Self { keyboard_layout }
    }

    fn calculate_unit_size(&self, available_width: f32, available_height: f32) -> f32 {
        let (layout_width, layout_height) = self.keyboard_layout.get_dimensions();
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

            for key in &self.keyboard_layout.keys {
                let rect = egui::Rect::from_min_size(
                    egui::pos2(key.x * unit_size, key.y * unit_size),
                    egui::vec2(key.w * unit_size, key.h * unit_size),
                );
                ui.painter().rect(
                    rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(200, 200, 200, 100),
                    egui::Stroke::NONE,
                );
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "x",
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let keyboard_layout = KeyboardLayout::new("C:/Users/Stephan/Downloads/keyboard.json")
        .expect("Failed to read keyboard layout.");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            // .with_decorations(false)
            // .with_taskbar(false)
            // .with_mouse_passthrough(true)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };

    eframe::run_native(
        "Transparent Window",
        options,
        Box::new(|_cc| {
            Ok(Box::<Overlay>::new(Overlay::new(keyboard_layout)))
        }),
    )
}
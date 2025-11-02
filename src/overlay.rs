use crate::cli::WindowPosition;
use crate::keyboard::Keyboard;
use crate::keycode_label;
use crate::keycode_label::KeycodeLabel;

use eframe::egui::{self, Align2, Window};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Overlay {
    keyboard: Keyboard,
    current_layer: Arc<Mutex<u8>>,
    time_to_hide_overlay: Arc<Mutex<Option<Instant>>>,
    size: (u32, u32),
    margin: u32,
    position: WindowPosition,
}

impl Overlay {
    pub fn new(
        keyboard: Keyboard,
        ctx: egui::Context,
        size: (u32, u32),
        margin: u32,
        position: WindowPosition,
    ) -> Self {
        let current_layer = Arc::new(Mutex::new(0));
        let time_to_hide_overlay = Arc::new(Mutex::new(Some(Instant::now())));
        let api = qmk_via_api::api::KeyboardApi::new(
            keyboard.keyboard_info.vid,
            keyboard.keyboard_info.pid,
            0xff60,
        )
        .expect("Failed to connect to device.");

        let current_layer_clone = Arc::clone(&current_layer);
        let time_to_hide_overlay_clone = Arc::clone(&time_to_hide_overlay);

        thread::spawn(move || loop {
            if let Ok(response) = api.hid_read() {
                if response[0] == 0x01 {
                    let new_layer = response[1];
                    *current_layer_clone.lock().unwrap() = new_layer;

                    if new_layer == 0 {
                        *time_to_hide_overlay_clone.lock().unwrap() =
                            Some(Instant::now() + Duration::from_secs(1));
                    } else {
                        *time_to_hide_overlay_clone.lock().unwrap() = None;
                    }

                    ctx.request_repaint();
                }
            }
        });

        Self {
            keyboard,
            current_layer,
            time_to_hide_overlay,
            size,
            margin,
            position,
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
        keycode_label: KeycodeLabel,
        rect: egui::Rect,
        font: egui::FontId,
    ) -> Option<std::sync::Arc<egui::Galley>> {
        let create_galley = |text: String| {
            ui.painter()
                .layout_no_wrap(text, font.clone(), egui::Color32::WHITE)
        };
        let galley_fits =
            |galley: &std::sync::Arc<egui::Galley>| galley.rect.width() <= rect.width() * 0.85;

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

    fn get_anchor_params(&self) -> (Align2, egui::Vec2) {
        match self.position {
            WindowPosition::TopLeft => (
                Align2::LEFT_TOP,
                egui::vec2(self.margin as f32, self.margin as f32),
            ),
            WindowPosition::TopRight => (
                Align2::RIGHT_TOP,
                egui::vec2(-(self.margin as f32), self.margin as f32),
            ),
            WindowPosition::BottomLeft => (
                Align2::LEFT_BOTTOM,
                egui::vec2(self.margin as f32, -(self.margin as f32)),
            ),
            WindowPosition::BottomRight => (
                Align2::RIGHT_BOTTOM,
                egui::vec2(-(self.margin as f32), -(self.margin as f32)),
            ),
            WindowPosition::Bottom => (
                Align2::CENTER_BOTTOM,
                egui::vec2(0.0, -(self.margin as f32)),
            ),
            WindowPosition::Top => (Align2::CENTER_TOP, egui::vec2(0.0, self.margin as f32)),
        }
    }
}

impl eframe::App for Overlay {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));

        let mut window_open = match self.time_to_hide_overlay.lock().unwrap().as_ref() {
            Some(time_to_hide) => Instant::now() < *time_to_hide,
            None => true,
        };

        let anchor_params = self.get_anchor_params();

        Window::new("QMK Layout Helper")
            .open(&mut window_open)
            .fixed_size(egui::vec2(self.size.0 as f32, self.size.1 as f32))
            .anchor(anchor_params.0, anchor_params.1)
            .frame(egui::Frame::NONE.fill(egui::Color32::TRANSPARENT))
            .fade_out(true)
            .title_bar(false)
            .show(ctx, |ui| {
                // Required as workaround for https://github.com/emilk/egui/issues/498
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());

                let layer = *self.current_layer.lock().unwrap();

                let available_rect = ui.available_size();
                let unit_size = self.calculate_unit_size(available_rect.x, available_rect.y);
                let window_pos = ui.min_rect().min;
                for key in &self.keyboard.keyboard_info.keys {
                    let keycode =
                        self.keyboard.matrix[layer as usize][key.row as usize][key.col as usize];
                    let keycode_label = keycode_label::get_keycode_label(keycode);

                    let rect = egui::Rect::from_min_size(
                        egui::pos2(key.x * unit_size, key.y * unit_size) + window_pos.to_vec2(),
                        egui::vec2(key.w * unit_size, key.h * unit_size),
                    )
                    .shrink(0.06 * unit_size);

                    let base_keycode = self.keyboard.matrix[0][key.row as usize][key.col as usize];
                    let base_color = keycode_label::get_keycode_label(base_keycode).color;
                    let stroke = egui::Stroke::new(
                        1.0,
                        base_color.lerp_to_gamma(
                            egui::Color32::from_rgba_premultiplied(255, 255, 255, 90),
                            0.05,
                        ),
                    );
                    ui.painter().rect(
                        rect,
                        0.1 * unit_size,
                        base_color,
                        stroke,
                        egui::StrokeKind::Outside,
                    );

                    let font = egui::FontId::proportional(0.3 * unit_size);

                    if let Some(label_galley) =
                        self.generate_key_label_galley(ui, keycode_label, rect, font)
                    {
                        let label_pos = rect.center() - label_galley.rect.center().to_vec2();
                        ui.painter()
                            .galley(label_pos, label_galley, egui::Color32::WHITE);
                    }
                }
            });

        ctx.request_repaint();
    }
}

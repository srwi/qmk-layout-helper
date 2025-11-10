use crate::cli::WindowPosition;
use crate::keyboard::Keyboard;
use crate::keycode_label;
use crate::keycode_label::KeycodeLabel;

use eframe::egui::{self, Align2, Window};
use std::time::Instant;

pub struct Overlay {
    keyboard: Keyboard,
    size: f32,
    margin: u32,
    position: WindowPosition,
}

impl Overlay {
    pub fn new(
        keyboard: Keyboard,
        _ctx: egui::Context,
        size: i32,
        margin: u32,
        position: WindowPosition,
    ) -> Self {
        Self {
            keyboard,
            margin,
            position,
            size: size as f32,
        }
    }

    fn generate_key_label_galley(
        &self,
        ui: &egui::Ui,
        keycode_label: KeycodeLabel,
        rect: egui::Rect,
        font: egui::FontId,
        color: egui::Color32,
    ) -> Option<std::sync::Arc<egui::Galley>> {
        let create_galley = |text: String| ui.painter().layout_no_wrap(text, font.clone(), color);
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

        let mut window_open = match self.keyboard.time_to_hide_overlay.lock().unwrap().as_ref() {
            Some(time_to_hide) => Instant::now() < *time_to_hide,
            None => true,
        };

        let anchor_params = self.get_anchor_params();

        Window::new("QMK Layout Helper")
            .open(&mut window_open)
            .auto_sized()
            .anchor(anchor_params.0, anchor_params.1)
            .frame(egui::Frame::NONE.fill(egui::Color32::TRANSPARENT))
            .fade_out(true)
            .title_bar(false)
            .show(ctx, |ui| {
                // Allow auto_sized window to shrink to fit content
                let layout_size = self.keyboard.keyboard_info.get_dimensions();
                ui.allocate_space(egui::vec2(
                    layout_size.0 * self.size,
                    layout_size.1 * self.size,
                ));

                let window_pos = ui.min_rect().min;

                for key in &self.keyboard.keyboard_info.keys {
                    let (effective_layer, is_background_key) = self
                        .keyboard
                        .get_effective_key_layer(key.row as usize, key.col as usize);

                    let bytes = self.keyboard.matrix[effective_layer as usize][key.row as usize]
                        [key.col as usize];
                    let keycode_label = keycode_label::get_keycode_label(bytes);

                    let first_layer_bytes =
                        self.keyboard.matrix[0][key.row as usize][key.col as usize];
                    let first_layer_keycode_kind =
                        keycode_label::get_keycode_label(first_layer_bytes).kind;

                    let (fill_color, stroke_color, font_color) = keycode_label::get_keycode_color(
                        keycode_label.layer_ref.unwrap_or(effective_layer),
                        first_layer_keycode_kind,
                        is_background_key,
                    );

                    // Draw key background
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(key.x * self.size, key.y * self.size) + window_pos.to_vec2(),
                        egui::vec2(key.w * self.size, key.h * self.size),
                    )
                    .shrink(0.06 * self.size);
                    ui.painter().rect(
                        rect,
                        0.1 * self.size,
                        fill_color,
                        egui::Stroke::new(1.0, stroke_color),
                        egui::StrokeKind::Outside,
                    );

                    // Draw key label
                    let font = egui::FontId::proportional(0.28 * self.size);
                    if let Some(label_galley) =
                        self.generate_key_label_galley(ui, keycode_label, rect, font, font_color)
                    {
                        let label_pos = rect.center() - label_galley.rect.center().to_vec2();
                        ui.painter().galley(label_pos, label_galley, font_color);
                    }
                }
            });

        ctx.request_repaint();
    }
}

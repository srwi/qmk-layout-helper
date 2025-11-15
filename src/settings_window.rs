use eframe::egui::{self};
use std::sync::{Arc, Mutex};

use crate::settings::Settings;
use crate::settings::WindowPosition;

pub struct SettingsApp {
    current: Settings,
    shared: Arc<Mutex<Settings>>,
    error: Option<String>,
}

impl SettingsApp {
    pub fn new(shared: Arc<Mutex<Settings>>) -> Self {
        let current = shared.lock().map(|s| s.clone()).unwrap_or_default();
        Self {
            current,
            shared,
            error: None,
        }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Keyboard info JSON path");
                ui.add(egui::TextEdit::singleline(
                    &mut self.current.keyboard_config_path,
                ));
            });

            ui.horizontal(|ui| {
                ui.label("Layout name");
                ui.add(egui::TextEdit::singleline(&mut self.current.layout_name));
            });

            ui.horizontal(|ui| {
                ui.label("Key size (px)");
                ui.add(egui::Slider::new(&mut self.current.size, 5..=200));
            });

            ui.horizontal(|ui| {
                ui.label("Margin (px)");
                ui.add(egui::Slider::new(&mut self.current.margin, 0..=100));
            });

            ui.horizontal(|ui| {
                ui.label("Timeout (ms)");
                ui.add(
                    egui::DragValue::new(&mut self.current.timeout)
                        .range(0..=60_000)
                        .speed(50),
                );
            });

            let position_label = self.current.position.to_string();
            ui.horizontal(|ui| {
                ui.label("Position");
                egui::ComboBox::from_label("")
                    .selected_text(position_label)
                    .show_ui(ui, |ui| {
                        for pos in [
                            WindowPosition::TopLeft,
                            WindowPosition::TopRight,
                            WindowPosition::BottomLeft,
                            WindowPosition::BottomRight,
                            WindowPosition::Top,
                            WindowPosition::Bottom,
                        ] {
                            ui.selectable_value(&mut self.current.position, pos, pos.to_string());
                        }
                    });
            });

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::from_rgb(220, 80, 90), err);
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Start").clicked() {
                    let path = self.current.keyboard_config_path.trim().to_string();
                    if path.is_empty() {
                        self.error =
                            Some("Please provide the path to the keyboard info JSON.".into());
                    } else if let Ok(mut settings) = self.shared.lock() {
                        settings.keyboard_config_path = path;
                        settings.layout_name = self.current.layout_name.clone();
                        settings.size = self.current.size;
                        settings.position = self.current.position;
                        settings.timeout = self.current.timeout;
                        settings.margin = self.current.margin;
                        settings.confirmed = true;
                        settings.save_settings = self.current.save_settings;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                ui.checkbox(&mut self.current.save_settings, "Remember settings");
            });
        });
    }
}

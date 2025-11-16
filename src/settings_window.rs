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
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(30, 25),
                fill: ctx.style().visuals.window_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../resources/icon.png"))
                            .max_width(120.0),
                    );
                    ui.add_space(10.0);
                    ui.heading("QMK Layout Helper");
                    ui.hyperlink_to(
                        format!("Version {}", env!("CARGO_PKG_VERSION")),
                        "https://github.com/srwi/qmk-layout-helper",
                    );
                    ui.add_space(25.0);

                    egui::Grid::new("settings_grid")
                        .num_columns(2)
                        .striped(true)
                        .spacing([25.0, 14.0])
                        .show(ui, |ui| {
                            ui.label("Keyboard info JSON");
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.current.keyboard_config_path),
                            );
                            ui.end_row();

                            ui.label("Layout");
                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.current.layout_name),
                            );
                            ui.end_row();

                            let position_label = self.current.position.to_string();
                            ui.label("Alignment");
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_salt("position_combo")
                                    .width(ui.available_width())
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
                                            ui.selectable_value(
                                                &mut self.current.position,
                                                pos,
                                                pos.to_string(),
                                            );
                                        }
                                    });
                            });
                            ui.end_row();

                            ui.label("Distance from screen edge");
                            ui.add_sized(
                                ui.available_size(),
                                egui::DragValue::new(&mut self.current.margin)
                                    .speed(1)
                                    .suffix(" px"),
                            );
                            ui.end_row();

                            ui.label("Key unit size");
                            ui.add_sized(
                                ui.available_size(),
                                egui::DragValue::new(&mut self.current.size)
                                    .speed(1)
                                    .range(20..=1000)
                                    .suffix(" px"),
                            );
                            ui.end_row();

                            ui.label("Display duration");
                            ui.add_sized(
                                ui.available_size(),
                                egui::DragValue::new(&mut self.current.timeout)
                                    .speed(50)
                                    .range(0..=60_000)
                                    .suffix(" ms"),
                            );
                            ui.end_row();
                        });

                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(25.0);
                        ui.checkbox(&mut self.current.save_settings, "Remember settings");
                        ui.add_space(5.0);
                        if ui
                            .add_sized([90.0, 28.0], egui::Button::new("Start"))
                            .clicked()
                        {
                            let path = self.current.keyboard_config_path.trim().to_string();
                            if path.is_empty() {
                                self.error = Some(
                                    "Please provide the path to the keyboard info JSON.".into(),
                                );
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
                    });
                });
            });
    }
}

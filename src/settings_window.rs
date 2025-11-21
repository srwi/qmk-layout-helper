use crate::keyboard::Keyboard;
use crate::keyboard_info::KeyboardInfo;
use crate::settings::Settings;
use crate::settings::WindowPosition;

use eframe::egui::{self};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct SettingsApp {
    current: Settings,
    shared: Arc<Mutex<Settings>>,
    error: Option<String>,
    layout_names: Vec<String>,
}

impl SettingsApp {
    pub fn new(shared: Arc<Mutex<Settings>>) -> Self {
        let current = shared.lock().map(|s| s.clone()).unwrap_or_default();
        Self {
            current,
            shared,
            error: None,
            layout_names: Vec::new(),
        }
    }

    fn file_button_label(&self) -> String {
        let path_str = self.current.keyboard_config_path.trim();
        if path_str.is_empty() {
            "Open file…".to_string()
        } else {
            Path::new(path_str)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(path_str)
                .to_string()
        }
    }

    fn handle_picked_file(&mut self, picked: String) {
        self.current.keyboard_config_path = picked;

        let keyboard_info = match KeyboardInfo::new(&self.current.keyboard_config_path) {
            Ok(info) => info,
            Err(err) => {
                self.error = Some(format!(
                    "Failed to parse keyboard info from the selected JSON: {err}"
                ));
                return;
            }
        };

        if let Err(err) = Keyboard::try_get_api(keyboard_info.vid, keyboard_info.pid) {
            self.error = Some(format!(
                "Failed to initialize keyboard from the selected JSON: {err}"
            ));
            return;
        }

        match keyboard_info.get_layout_names() {
            Ok(names) => {
                self.layout_names = names;
                if let Some(first) = self.layout_names.first() {
                    if !self
                        .layout_names
                        .iter()
                        .any(|n| n == &self.current.layout_name)
                    {
                        self.current.layout_name = first.clone();
                    }
                }
                self.error = None;
            }
            Err(err) => {
                self.layout_names.clear();
                self.error = Some(err.to_string());
            }
        }
    }
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(30, 20),
                fill: ctx.style().visuals.window_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("QMK Layout Helper");
                    ui.hyperlink_to(
                        format!("Version {}", env!("CARGO_PKG_VERSION")),
                        "https://github.com/srwi/qmk-layout-helper",
                    );

                    ui.add_space(20.0);

                    egui::Grid::new("settings_grid")
                        .num_columns(2)
                        .striped(true)
                        .spacing([25.0, 14.0])
                        .show(ui, |ui| {
                            ui.label("Keyboard info JSON");
                            ui.add_enabled_ui(
                                self.current.keyboard_config_path.trim().is_empty(),
                                |ui| {
                                    if ui
                                        .add_sized(
                                            ui.available_size(),
                                            egui::Button::new(self.file_button_label()),
                                        )
                                        .clicked()
                                    {
                                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                                            self.handle_picked_file(path.display().to_string());
                                        }
                                    };
                                },
                            );
                            ui.end_row();

                            ui.label("Layout");
                            ui.add_enabled_ui(!self.layout_names.is_empty(), |ui| {
                                egui::ComboBox::from_id_salt("layout_combo")
                                    .width(ui.available_width())
                                    .selected_text(self.current.layout_name.as_str())
                                    .show_ui(ui, |ui| {
                                        for name in &self.layout_names {
                                            ui.selectable_value(
                                                &mut self.current.layout_name,
                                                name.clone(),
                                                name,
                                            );
                                        }
                                    });
                            });
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
                    ui.add_space(20.0);
                    ui.checkbox(&mut self.current.save_settings, "Remember settings");
                    ui.add_space(5.0);
                    ui.add_enabled_ui(!self.current.keyboard_config_path.is_empty(), |ui| {
                        if ui
                            .add_sized([90.0, 28.0], egui::Button::new("Start"))
                            .clicked()
                        {
                            if let Ok(mut settings) = self.shared.lock() {
                                settings.keyboard_config_path =
                                    self.current.keyboard_config_path.trim().to_string();
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

        if let Some(error_message) = self.error.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(error_message);
                    ui.add_space(10.0);
                    if ui.button("OK").clicked() {
                        self.error = None;
                    }
                });
        }
    }
}

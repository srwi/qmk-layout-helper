use crate::cli::{Cli, WindowPosition};
use eframe::egui::{self};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Settings {
    pub keyboard_config_path: String,
    pub layout_name: String,
    pub size: i32,
    pub position: WindowPosition,
    pub timeout: u64,
    pub margin: u32,
    pub confirmed: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            keyboard_config_path: String::new(),
            layout_name: "LAYOUT".to_string(),
            size: 60,
            position: WindowPosition::BottomRight,
            timeout: 2000,
            margin: 10,
            confirmed: false,
        }
    }
}

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
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

            ui.heading("QMK Layout Helper – Settings");
            ui.add_space(6.0);

            ui.separator();

            ui.label("Keyboard info JSON path");
            ui.add(egui::TextEdit::singleline(
                &mut self.current.keyboard_config_path,
            ));

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

            let pos_label = match self.current.position {
                WindowPosition::TopLeft => "Top Left",
                WindowPosition::TopRight => "Top Right",
                WindowPosition::BottomLeft => "Bottom Left",
                WindowPosition::BottomRight => "Bottom Right",
                WindowPosition::Top => "Top",
                WindowPosition::Bottom => "Bottom",
            };

            egui::ComboBox::from_label("Position")
                .selected_text(pos_label)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.current.position,
                        WindowPosition::TopLeft,
                        "Top Left",
                    );
                    ui.selectable_value(
                        &mut self.current.position,
                        WindowPosition::TopRight,
                        "Top Right",
                    );
                    ui.selectable_value(
                        &mut self.current.position,
                        WindowPosition::BottomLeft,
                        "Bottom Left",
                    );
                    ui.selectable_value(
                        &mut self.current.position,
                        WindowPosition::BottomRight,
                        "Bottom Right",
                    );
                    ui.selectable_value(&mut self.current.position, WindowPosition::Top, "Top");
                    ui.selectable_value(
                        &mut self.current.position,
                        WindowPosition::Bottom,
                        "Bottom",
                    );
                });

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::from_rgb(220, 80, 90), err);
            }

            ui.separator();
            if ui.button("Start overlay").clicked() {
                let path = self.current.keyboard_config_path.trim().to_string();
                if path.is_empty() {
                    self.error = Some("Please provide the path to the keyboard info JSON.".into());
                } else {
                    if let Ok(mut s) = self.shared.lock() {
                        s.keyboard_config_path = path;
                        s.layout_name = self.current.layout_name.clone();
                        s.size = self.current.size;
                        s.position = self.current.position.clone();
                        s.timeout = self.current.timeout;
                        s.margin = self.current.margin;
                        s.confirmed = true;
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });
    }
}

impl From<Cli> for Settings {
    fn from(cli: Cli) -> Self {
        Self {
            keyboard_config_path: cli.keyboard_config.to_string_lossy().to_string(),
            layout_name: cli.layout_name,
            size: cli.size,
            position: cli.position,
            timeout: cli.timeout,
            margin: cli.margin,
            confirmed: true,
        }
    }
}

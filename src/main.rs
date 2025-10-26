use clap::Parser;
use eframe::egui::{self, Align2, Window};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod keyboard;
mod keyboard_layout;
mod keycode_label;
use keyboard::Keyboard;

struct Overlay {
    keyboard: Keyboard,
    current_layer: Arc<Mutex<u8>>,
    time_to_hide_overlay: Arc<Mutex<Option<Instant>>>,
    size: (u32, u32),
    position: WindowPosition,
}

impl Overlay {
    fn new(
        keyboard: Keyboard,
        ctx: egui::Context,
        size: (u32, u32),
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
        keycode_label: keycode_label::KeycodeLabel,
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

        let anchor = match self.position {
            WindowPosition::TopLeft => Align2::LEFT_TOP,
            WindowPosition::TopRight => Align2::RIGHT_TOP,
            WindowPosition::BottomLeft => Align2::LEFT_BOTTOM,
            WindowPosition::BottomRight => Align2::RIGHT_BOTTOM,
            WindowPosition::Bottom => Align2::CENTER_BOTTOM,
            WindowPosition::Top => Align2::CENTER_TOP,
        };

        Window::new("QMK Layout Helper")
            .open(&mut window_open)
            .fixed_size(egui::vec2(self.size.0 as f32, self.size.1 as f32))
            .anchor(anchor, [0.0, 0.0])
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
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
                    .shrink(0.05 * unit_size);

                    // TODO: The color should be based on the keycodes of the first layer
                    ui.painter().rect(
                        rect,
                        0.12 * unit_size,
                        keycode_label.color,
                        egui::Stroke::NONE,
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

fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("Size must be in the format 'width,height'".to_string());
    }
    let width: u32 = parts[0].parse().map_err(|_| "Invalid width")?;
    let height: u32 = parts[1].parse().map_err(|_| "Invalid height")?;
    Ok((width, height))
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    keyboard_config: PathBuf,

    #[arg(short = 'l', long = "layout", default_value = "LAYOUT")]
    layout_name: String,

    #[arg(long, value_parser = parse_size, default_value = "700,240")]
    size: (u32, u32),

    #[arg(long, value_enum, default_value = "bottom-right")]
    position: WindowPosition,

    #[arg(long, value_parser = clap::value_parser!(u64).range(100..), default_value = "5000")]
    timeout: u64,

    #[arg(long, default_value = "10")]
    margin: u32,
}

#[derive(clap::ValueEnum, Clone)]
enum WindowPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Bottom,
    Top,
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
            .with_decorations(false)
            .with_taskbar(false)
            .with_maximized(true)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "QMK Layout Helper",
        options,
        Box::new(move |cc| {
            Ok(Box::new(Overlay::new(
                keyboard,
                cc.egui_ctx.clone(),
                cli.size,
                cli.position,
            )))
        }),
    )
}

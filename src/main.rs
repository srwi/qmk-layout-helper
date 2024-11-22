use crate::egui::ViewportCommand;
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
use std::time::{Duration, Instant};

struct Overlay {
    keyboard: Keyboard,
    current_layer: Arc<Mutex<u8>>,
    visible: Arc<Mutex<bool>>,
    last_layer_change: Arc<Mutex<Instant>>,
}

impl Overlay {
    fn new(keyboard: Keyboard, ctx: egui::Context) -> Self {
        let current_layer = Arc::new(Mutex::new(0));
        let visible = Arc::new(Mutex::new(false));
        let last_layer_change = Arc::new(Mutex::new(Instant::now()));
        let api = qmk_via_api::api::KeyboardApi::new(
            keyboard.keyboard_info.vid,
            keyboard.keyboard_info.pid,
            0xff60,
        )
        .expect("Failed to connect to device.");

        let layer_clone = Arc::clone(&current_layer);
        let visible_clone = Arc::clone(&visible);
        let last_layer_change_clone = Arc::clone(&last_layer_change);

        thread::spawn(move || loop {
            if let Some(response) = api.hid_read() {
                if response[0] == 0x01 {
                    let new_layer = response[1];
                    {
                        let mut layer = layer_clone.lock().unwrap();
                        *layer = new_layer;
                    }
                    {
                        let mut visible = visible_clone.lock().unwrap();
                        *visible = true;
                    }
                    {
                        let mut last_layer_change = last_layer_change_clone.lock().unwrap();
                        *last_layer_change = Instant::now();
                    }
                    ctx.request_repaint();

                    if new_layer == 0 {
                        thread::spawn({
                            let visible_clone = Arc::clone(&visible_clone);
                            let layer_clone = Arc::clone(&layer_clone);
                            let ctx = ctx.clone();
                            move || {
                                thread::sleep(Duration::from_secs(3));
                                let layer = *layer_clone.lock().unwrap();
                                if layer == 0 {
                                    let mut visible = visible_clone.lock().unwrap();
                                    *visible = false;
                                    ctx.request_repaint();
                                }
                            }
                        });
                    }
                }
            }
        });

        Self {
            keyboard,
            current_layer,
            visible,
            last_layer_change,
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
        let visible = *self.visible.lock().unwrap();
        if !visible {
            return;
        }

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
            .with_inner_size([cli.size.0 as f32, cli.size.1 as f32])
            // .with_position(calculate_window_pos(&cli.position, &cli.size, cli.margin))
            .with_position(egui::Pos2::new(0.0, 0.0))
            .with_decorations(false)
            .with_taskbar(false)
            //.with_window_type(egui::X11WindowType::Notification)  // TODO: possible fix for X11 always on top
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "QMK Layout Helper",
        options,
        Box::new(|cc| Ok(Box::new(Overlay::new(keyboard, cc.egui_ctx.clone())))),
    )
}

// fn get_primary_monitor_info() -> (winit::dpi::LogicalSize<f64>, f64) {
//     // TODO: window position should only be set when showing the overlay. Maybe it can be done inside the update function and therfore dont require the event loop here
//     let event_loop = winit::event_loop::EventLoop::new(); // TODO: creating an event loop here causes egui to not update anymore
//     let primary_monitor = event_loop
//         .primary_monitor()
//         .expect("No primary monitor found");
//     let physical_size = primary_monitor.size();
//     let scale_factor = primary_monitor.scale_factor();
//     let logical_size = physical_size.to_logical(scale_factor);
//     (logical_size, scale_factor)
// }

// fn calculate_window_pos(position: &WindowPosition, size: &(u32, u32), margin: u32) -> egui::Pos2 {
//     let (screen_size, scale_factor) = get_primary_monitor_info();

//     let screen_width = screen_size.width as f32;
//     let screen_height = screen_size.height as f32;
//     let (width, height) = (size.0 as f32, size.1 as f32);
//     let margin = margin as f32 / scale_factor as f32;

//     match position {
//         WindowPosition::TopLeft => egui::Pos2::new(margin, margin),
//         WindowPosition::TopRight => egui::Pos2::new(screen_width - width - margin, margin),
//         WindowPosition::BottomLeft => egui::Pos2::new(margin, screen_height - height - margin),
//         WindowPosition::BottomRight => egui::Pos2::new(
//             screen_width - width - margin,
//             screen_height - height - margin,
//         ),
//         WindowPosition::Bottom => egui::Pos2::new(
//             (screen_width - width) / 2.0,
//             screen_height - height - margin,
//         ),
//         WindowPosition::Top => egui::Pos2::new((screen_width - width) / 2.0, margin),
//     }
// }

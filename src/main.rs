#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod keyboard;
mod keyboard_info;
mod keycode_labels;
mod overlay_window;
mod settings;
mod settings_window;
mod tray;

use eframe::egui;
use keyboard::Keyboard;
use keyboard_info::KeyboardInfo;
use overlay_window::Overlay;
use settings::Settings;
use settings_window::SettingsApp;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    const SETTINGS_FILE: &str = "settings.ini";

    let settings: Settings = if let Some(loaded) = Settings::load_from_file(SETTINGS_FILE) {
        loaded
    } else {
        let shared = Arc::new(Mutex::new(Settings::default()));
        let options = eframe::NativeOptions {
            run_and_return: true,
            viewport: egui::ViewportBuilder::default()
                .with_decorations(true)
                .with_inner_size([480.0, 510.0])
                .with_resizable(false)
                .with_maximize_button(false),
            ..Default::default()
        };
        eframe::run_native("QMK Layout Helper", options, {
            let shared_settings = shared.clone();
            Box::new(move |cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);

                let mut fonts = egui::FontDefinitions::default();
                egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
                cc.egui_ctx.set_fonts(fonts);

                Ok(Box::new(SettingsApp::new(shared_settings)))
            })
        })?;
        let settings = shared.lock().unwrap().clone();
        if !settings.confirmed {
            return Ok(());
        }
        if settings.save_settings {
            if let Err(e) = settings.save_to_file(SETTINGS_FILE) {
                eprintln!("Failed to save settings: {e}");
            }
        }
        settings
    };

    let keyboard_info =
        KeyboardInfo::new(&settings.keyboard_config_path).expect("Failed to read keyboard layout.");
    let keyboard = Keyboard::new(keyboard_info, settings.layout_name, settings.timeout);

    let _tray_icon = tray::create_tray_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_taskbar(false)
            .with_maximized(true)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };

    return eframe::run_native(
        "QMK Layout Helper",
        options,
        Box::new(move |cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(Overlay::new(
                keyboard,
                settings.size,
                settings.margin,
                settings.position,
            )))
        }),
    );
}

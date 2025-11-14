#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod cli;
mod keyboard;
mod keyboard_layout;
mod keycode_labels;
mod overlay;
mod settings;
mod tray;

use clap::Parser;
use eframe::egui;

use cli::Cli;
use keyboard::Keyboard;
use keyboard_layout::KeyboardInfo;
use overlay::Overlay;
use settings::{Settings, SettingsApp};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let _tray_icon = tray::create_tray_icon();

    let settings: Settings = if std::env::args_os().len() <= 1 {
        let settings = Arc::new(Mutex::new(Settings::default()));
        let options = eframe::NativeOptions {
            run_and_return: true,
            viewport: egui::ViewportBuilder::default()
                .with_decorations(true)
                .with_inner_size([520.0, 260.0]),
            ..Default::default()
        };

        eframe::run_native("QMK Layout Helper – Settings", options, {
            let shared_settings = settings.clone();
            Box::new(move |cc| {
                let mut fonts = egui::FontDefinitions::default();
                egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
                cc.egui_ctx.set_fonts(fonts);
                Ok(Box::new(SettingsApp::new(shared_settings)))
            })
        })?;

        let s = settings.lock().unwrap().clone();
        if s.confirmed {
            s
        } else {
            return Ok(());
        }
    } else {
        Settings::from(Cli::parse())
    };

    let keyboard_info = KeyboardInfo::new(&settings.keyboard_config_path, &settings.layout_name)
        .expect("Failed to read keyboard layout.");
    let keyboard = Keyboard::new(keyboard_info, settings.timeout);

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

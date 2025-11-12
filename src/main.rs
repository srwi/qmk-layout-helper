#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod cli;
mod keyboard;
mod keyboard_layout;
mod keycode_labels;
mod overlay;
mod tray;

use clap::Parser;
use eframe::egui;

use cli::Cli;
use keyboard::Keyboard;
use keyboard_layout::KeyboardInfo;
use overlay::Overlay;

fn main() -> Result<(), eframe::Error> {
    let _tray_icon = tray::create_tray_icon();

    let cli = Cli::parse();

    let keyboard_config = cli.keyboard_config.to_str().expect("Invalid path");
    let layout_name = &cli.layout_name;

    let keyboard_info =
        KeyboardInfo::new(keyboard_config, layout_name).expect("Failed to read keyboard layout.");
    let keyboard = Keyboard::new(keyboard_info, cli.timeout);

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
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(Overlay::new(
                keyboard,
                cli.size,
                cli.margin,
                cli.position,
            )))
        }),
    )
}

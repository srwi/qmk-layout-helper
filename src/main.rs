use eframe::egui;
use crate::egui::ViewportCommand;

struct Overlay;

impl eframe::App for Overlay {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));

        let frame = egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        };

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.painter().circle(
                egui::pos2(100.0, 100.0),
                50.0,
                egui::Rgba::from_rgba_premultiplied(1.0, 0.0, 0.0, 0.2),
                egui::Stroke::default(),
            );
            ui.painter().rect(
                egui::Rect::from_min_size(egui::pos2(200.0, 50.0), egui::vec2(100.0, 100.0)),
                0.0,
                egui::Color32::GREEN,
                egui::Stroke::default(),
            );
            ui.painter().line_segment(
                [egui::pos2(50.0, 200.0), egui::pos2(250.0, 200.0)],
                egui::Stroke::new(5.0, egui::Color32::BLUE),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(),
            // .with_mouse_passthrough(true),
        ..Default::default()
    };

    eframe::run_native(
        "Transparent Window",
        options,
        Box::new(|_cc| {
            Ok(Box::<Overlay>::new(Overlay))
        }),
    )
}
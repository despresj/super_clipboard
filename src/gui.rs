use eframe::egui;
use egui::Vec2;
use std::fs::read_to_string;
use std::path::Path;

use crate::clipboard_logger::LOGFILE;

pub fn launch_gui() {
    eframe::run_native(
        "Super Clipboard",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(FileViewer::new(cc))),
    )
    .unwrap();
}
#[derive(Default)]
struct FileViewer {}

impl FileViewer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for FileViewer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let file_path = Path::new(LOGFILE);
        let file_contents = read_to_string(file_path).unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.input(|i| i.key_pressed(egui::Key::Q)) {
                panic!("BOO Pressed Q");
            }

            ui.label("new app");

            let mut scroll_delta = Vec2::ZERO;
            if ui.button("Scroll down").clicked() {
                scroll_delta.y -= 64.0; // move content up
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.scroll_with_delta(scroll_delta);
                for line in file_contents.split("\n") {
                    ui.label(format!("{line}"));
                }
            });
        });
    }
}

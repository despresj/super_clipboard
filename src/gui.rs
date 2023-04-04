use eframe::egui;
use std::fs::read_to_string;
use std::path::Path;

use crate::clipboard_logger::{copy_item, LOGFILE};

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
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for FileViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let file_path = Path::new(LOGFILE);
        let file_contents = read_to_string(file_path).unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.input(|i| i.key_pressed(egui::Key::Q)) {
                panic!("BOO Pressed Q");
            }

            if ui.button("Clear").clicked() {
                todo!();
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in file_contents.split("\n") {
                    if ui.selectable_label(false, line).clicked() {
                        copy_item(line);
                    }
                }
            });
        });
    }
}

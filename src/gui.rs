use eframe::egui;
use std::fs::read_to_string;
use std::path::Path;

pub fn launch_gui() {
    eframe::run_native(
        "Super Clipboard",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(FileViewer::new(cc))),
    );
}
#[derive(Default)]
struct FileViewer {}

impl FileViewer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for FileViewer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let file_path = Path::new("example/eg.txt");
        let file_contents = read_to_string(file_path).unwrap();

        egui::CentralPanel::default().show(ctx, |ui| ui.heading(file_contents));
    }
}

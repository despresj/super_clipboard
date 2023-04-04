use eframe::egui;
use regex::Regex;
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
        let mut file_contents = read_and_rev_file(Path::new(LOGFILE)).into_iter();

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.input(|i| i.key_pressed(egui::Key::Q)) {
                panic!("BOO Pressed Q");
            }
            if ui.button("Clear").clicked() {
                todo!();
            }
            let current_clipboard = file_contents.next().unwrap().strip_time_stamp();
            ui.add(egui::Label::new(format!(
                "Current Clipboard: {current_clipboard}"
            )));
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in file_contents {
                    if ui.selectable_label(false, &line).clicked() {
                        copy_item(line);
                    }
                }
            });
        });
    }
}
fn read_and_rev_file(file_path: &Path) -> Vec<String> {
    let file_contents = read_to_string(file_path).unwrap();
    let mut lines: Vec<String> = file_contents.lines().map(|line| line.to_owned()).collect();
    lines.reverse();
    return lines;
}

trait Clip {
    fn strip_time_stamp(&self) -> String;
}
impl Clip for String {
    fn strip_time_stamp(&self) -> String {
        let re = Regex::new(r"\d{4}-\d{2}-\d{2}-\d{2}:\d{2}:\d{2} clip: ").unwrap();
        return re.replace_all(self.as_str(), "").to_string();
    }
}

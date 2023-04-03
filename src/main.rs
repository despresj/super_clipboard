mod checks;
mod clipboard_logger;
mod gui;

fn main() {
    if !checks::is_super_clipboard_running() {
        clipboard_logger::monitor_clipboard();
    }

    let gui = false;
    if gui {
        gui::launch_gui();
    }
}

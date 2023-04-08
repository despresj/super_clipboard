mod checks;
mod clipboard_logger;
mod gui;
use rdev::{listen, Event, EventType, Key};

fn main() {
    if !checks::is_super_clipboard_running() {
        clipboard_logger::monitor_clipboard();
    }
    check_for_cmd_shift_v();
}

fn check_for_cmd_shift_v() {
    let mut cmd_pressed = false;
    let mut shift_pressed = false;

    let callback = move |event: Event| match event.event_type {
        EventType::KeyPress(key) => match key {
            Key::MetaRight | Key::MetaLeft => {
                cmd_pressed = true;
            }
            Key::ShiftRight | Key::ShiftLeft => {
                shift_pressed = true;
            }
            Key::KeyV => {
                if cmd_pressed && shift_pressed {
                    dbg!("cmd + shift + v pressed opening gui");
                    gui::launch_gui();
                }
            }
            Key::KeyQ => {
                panic!("q press")
            }
            _ => (),
        },

        _ => (),
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

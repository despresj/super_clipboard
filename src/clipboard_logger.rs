use chrono::Local;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time::Duration};

const CLIPBOARD_ERROR: &str = "ERROR PROCESSING ClipboardContext";
// TODO this should be a variable
pub const LOGFILE: &str = "/Users/josephdespres/rust/super_clipboard/.cliplog";

// TODO make this a method for types macos linux
fn get_clip(ctx: &mut cli_clipboard::macos_clipboard::MacOSClipboardContext) -> String {
    ctx.get_contents()
        .unwrap_or_else(|_| String::from(CLIPBOARD_ERROR))
}

pub fn monitor_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    loop {
        let clip = get_clip(&mut ctx);
        while clip == get_clip(&mut ctx) {
            thread::sleep(Duration::from_millis(100));
        }
        let new_clip = get_clip(&mut ctx);
        write_clip(new_clip);
    }
}

fn format_timestamp() -> String {
    Local::now().format("%Y-%m-%d-%H:%M:%S").to_string()
}

fn write_clip(clip: String) {
    if clip != *CLIPBOARD_ERROR {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(false)
            .open(LOGFILE)
            .unwrap();

        let write_str = format!("{} clip: {}", format_timestamp(), &clip);
        dbg!(&write_str);
        writeln!(file, "{write_str}").unwrap();
    }
}

pub fn copy_item(item: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}-\d{2}:\d{2}:\d{2} clip: ").unwrap();
    let replaced = re.replace_all(item, "");
    ctx.set_contents(replaced.to_string()).unwrap();
}

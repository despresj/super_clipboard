use sysinfo::{ProcessExt, System, SystemExt};

pub fn is_super_clipboard_running() -> bool {
    let mut super_clipboard = 0;
    for process in System::new_all().processes_by_exact_name("super_clipboard") {
        super_clipboard += 1;
    }

    match super_clipboard {
        0 => false,
        1 => false,
        2 => true,
        _ => panic!("I am not expecting {} processes running", super_clipboard),
    }
}

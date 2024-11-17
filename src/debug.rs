use std::fs::OpenOptions;
use std::io::Write;

pub fn debug_log(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("targ_debug.log")
    {
        let _ = writeln!(file, "{}", message);
    }
}

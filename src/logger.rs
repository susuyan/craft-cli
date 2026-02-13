use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::SeqCst);
}

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::SeqCst)
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::logger::is_verbose() {
            eprintln!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug_json {
    ($label:expr, $value:expr) => {
        if $crate::logger::is_verbose() {
            match serde_json::to_string_pretty($value) {
                Ok(json) => eprintln!("[DEBUG] {}: {}", $label, json),
                Err(_) => eprintln!("[DEBUG] {}: {:?}", $label, $value),
            }
        }
    };
}

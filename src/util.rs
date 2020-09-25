use std::process::exit;

pub const VERSION: &'static str = "0.3.0";

/// Print an error and exit with error code
pub fn fail(prompt: &'_ str) -> ! {
    leg::error(prompt, None, None);
    exit(1)
}

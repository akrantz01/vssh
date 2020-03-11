use crate::config::Config;
use std::process::exit;

pub fn delete(name: String, mut config: Config) {
    // Delete if exists
    config.profiles.remove(&name);

    // Commit changes
    match config.write() {
        Ok(_) => println!("Successfully deleted profile if it existed"),
        Err(e) => {
            println!("Failed to write configuration file: {}", e);
            exit(1);
        }
    }
}

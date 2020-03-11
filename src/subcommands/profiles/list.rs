use crate::config::Config;

pub fn list(config: Config) {
    // Check if any profiles
    if config.profiles.is_empty() {
        println!("No profiles");
        return;
    }

    // Print all keys
    for key in config.profiles.keys() {
        println!("{}", key);
    }
}

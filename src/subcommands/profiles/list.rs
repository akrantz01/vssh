use crate::config::Config;

pub fn list(config: Config) {
    // Check if any profiles
    if config.profiles.is_empty() {
        leg::success("No profiles found", None, None);
        return;
    }

    leg::success("Got list of server profiles", None, None);

    // Print all keys
    for key in config.profiles.keys() {
        println!("{}", key);
    }
}

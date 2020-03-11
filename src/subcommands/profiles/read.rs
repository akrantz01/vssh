use crate::config::Config;

pub fn read(name: String, config: Config) {
    // Print data if entry exists
    match config.profiles.get(&name) {
        Some(profile) => println!(
            "{}:\n\tUsername: {}\n\tRole: {}\n\tAddress: {}\n\tPrivate Key: {}",
            name, profile.username, profile.role, profile.address, profile.private_key
        ),
        None => println!("Profile '{}' does not exist", name),
    }
}

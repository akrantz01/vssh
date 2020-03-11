use crate::config::Config;

pub fn read(name: String, config: Config) {
    // Print data if entry exists
    match config.profiles.get(&name) {
        Some(profile) => println!(
            "{}:\n\tUsername: {}\n\tRole: {}\n\tAddress: {}\n\tPrivate Key: {}\n\tPublic Key: {}",
            name,
            profile.username,
            profile.role,
            profile.address,
            profile.private_key.as_ref().unwrap_or(&"None".to_string()),
            profile.public_key.as_ref().unwrap_or(&"None".to_string())
        ),
        None => println!("Profile '{}' does not exist", name),
    }
}

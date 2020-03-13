use crate::config::Config;
use crate::util::fail;

pub fn read(name: String, config: Config) {
    // Print data if entry exists
    match config.profiles.get(&name) {
        Some(profile) => leg::success(
            &format!(
            "{}:\n\tUsername: {}\n\tRole: {}\n\tAddress: {}\n\tPrivate Key: {}\n\tPublic Key: {}",
            name,
            profile.username,
            profile.role,
            profile.address,
            profile.private_key.as_ref().unwrap_or(&"None".to_string()),
            profile.public_key.as_ref().unwrap_or(&"None".to_string())
        ),
            None,
            None,
        ),
        None => fail(&format!("Profile '{}' does not exist", name)),
    }
}

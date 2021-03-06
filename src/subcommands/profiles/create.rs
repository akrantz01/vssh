use crate::config::{Config, Profile};
use crate::util::fail;

pub fn create(
    name: String,
    username: Option<String>,
    address: String,
    role: String,
    private_key: Option<String>,
    public_key: Option<String>,
    options: String,
    mut config: Config,
) {
    // Get default username if not provided
    let username = if let Some(username) = username {
        username
    } else {
        whoami::username()
    };

    // Ensure profile does not exist
    if config.profiles.contains_key(&name) {
        fail(&format!("Profile '{}' already exists", name));
    }

    // Add profile to configuration
    config.profiles.insert(
        name.clone(),
        Profile {
            username,
            address,
            role,
            private_key,
            public_key,
            options,
        },
    );

    // Write to file
    match config.write() {
        Ok(_) => leg::success(&format!("Created profile '{}'", name), None, None),
        Err(e) => fail(&format!("Failed to write to configuration file: {}", e)),
    }
}

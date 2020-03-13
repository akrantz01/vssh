use crate::config::Config;
use crate::util::fail;

pub fn update(
    name: String,
    username: Option<String>,
    address: Option<String>,
    role: Option<String>,
    private_key: Option<String>,
    public_key: Option<String>,
    options: Option<String>,
    mut config: Config,
) {
    // Ensure profile exists
    if !config.profiles.contains_key(&name) {
        fail(&format!("Profile '{}' does not exist", name));
    }

    // Update entry in place
    config.profiles.entry(name.clone()).and_modify(|profile| {
        // Set username if exists
        if let Some(username) = username {
            profile.username = username;
        }

        // Set address if exists
        if let Some(address) = address {
            profile.address = address;
        }

        // Set role if exists
        if let Some(role) = role {
            profile.role = role;
        }

        // Set private key if exists
        if let Some(private_key) = private_key {
            profile.private_key = Some(private_key);
        }

        // Set public key if exists
        if let Some(public_key) = public_key {
            profile.public_key = Some(public_key);
        }

        // Set options if exists
        if let Some(options) = options {
            profile.options = options;
        }
    });

    // Write to file
    match config.write() {
        Ok(_) => leg::success(
            &format!("Successfully updated profile '{}'", name),
            None,
            None,
        ),
        Err(e) => fail(&format!("Failed to write to configuration file: {}", e)),
    }
}

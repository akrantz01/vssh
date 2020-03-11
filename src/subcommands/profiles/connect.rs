use crate::api::ApiClient;
use crate::config::Config;
use crate::subcommands::connect::connect as connect_subcommand;
use std::process::exit;

pub async fn connect(name: String, client: &ApiClient, config: &Config) {
    // Ensure profile exists
    let profile = match config.profiles.get(&name) {
        Some(profile) => profile,
        None => {
            println!("Profile '{}' does not exist", name);
            exit(1);
        }
    };

    // Run using same subcommand
    connect_subcommand(
        client,
        profile.role.clone(),
        profile.private_key.clone(),
        profile.public_key.clone(),
        format!("{}@{}", profile.username, profile.address),
        profile.options.clone(),
    )
    .await;
}

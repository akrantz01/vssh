use crate::api::ApiClient;
use crate::config::Config;
use crate::subcommands::connect::connect as connect_subcommand;
use crate::util::fail;

pub async fn connect(name: String, sftp: bool, client: &ApiClient, config: &Config) {
    // Ensure profile exists
    let profile = match config.profiles.get(&name) {
        Some(profile) => profile,
        None => fail(&format!("Profile '{}' does not exist", name)),
    };

    leg::success("Retrieved selected profile", None, None);

    // Run using same subcommand
    connect_subcommand(
        client,
        profile.role.clone(),
        profile.private_key.clone(),
        profile.public_key.clone(),
        format!("{}@{}", profile.username, profile.address),
        sftp,
        profile.options.clone(),
    )
    .await;
}

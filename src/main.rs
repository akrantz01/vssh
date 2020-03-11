extern crate dirs;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate structopt;
extern crate tokio;
extern crate url;
extern crate whoami;

mod api;
mod cli;
mod config;
mod errors;
mod subcommands;
mod util;

use api::ApiClient;
use cli::{Command, Opts, Profiles};
use config::Config;
use std::process::exit;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    // Parse cli arguments and parameters
    let cli = Opts::from_args();

    match cli.cmd {
        Command::Setup {
            non_interactive,
            server,
            no_tls,
            token,
            path,
            custom_ca,
        } => {
            if non_interactive {
                subcommands::setup::noninteractive(
                    server.unwrap_or_default(),
                    !no_tls,
                    token.unwrap_or_default(),
                    path.unwrap_or_default(),
                    custom_ca.unwrap_or_default(),
                )
            } else {
                subcommands::setup::interactive()
            }
        }
        Command::List => {
            let config = load_config(cli.config);
            let client = initialize_api(&config).await;
            subcommands::list(&client).await
        }
        Command::Sign { role, key, output } => {
            let config = load_config(cli.config);
            let client = initialize_api(&config).await;
            subcommands::sign(&client, role, key, output.unwrap_or_default()).await;
        }
        Command::Connect {
            role,
            key,
            server,
            options,
        } => {
            let config = load_config(cli.config);
            let client = initialize_api(&config).await;
            subcommands::connect(&client, role, key, server, options.unwrap_or_default())
                .await;
        }
        Command::Profiles(p) => match p {
            Profiles::Create {
                name,
                username,
                address,
                role,
                private_key,
                options,
            } => {
                let config = load_config(cli.config);
                subcommands::profiles::create(
                    name,
                    username.unwrap_or_default(),
                    address,
                    role,
                    private_key,
                    options,
                    config,
                );
            }
            Profiles::Read { name } => {
                let config = load_config(cli.config);
                subcommands::profiles::read(name, config);
            }
            Profiles::List => {
                let config = load_config(cli.config);
                subcommands::profiles::list(config);
            }
            Profiles::Update {
                name,
                username,
                address,
                role,
                private_key,
                options,
            } => {
                let config = load_config(cli.config);
                subcommands::profiles::update(
                    name,
                    username,
                    address,
                    role,
                    private_key,
                    options,
                    config,
                );
            }
            Profiles::Delete { name } => {
                let config = load_config(cli.config);
                subcommands::profiles::delete(name, config);
            }
            Profiles::Connect { name } => {
                let config = load_config(cli.config);
                let client = initialize_api(&config).await;
                subcommands::profiles::connect(name, &client, &config).await;
            }
        },
    };
}

/// Load configuration file and handle errors
fn load_config(file: Option<String>) -> Config {
    // Attempt to read config file
    let config = if let Some(file) = file {
        Config::read(file)
    } else {
        Config::read_default()
    };

    // Ensure no errors when loading
    match config {
        Ok(c) => c,
        Err(e) => match e {
            errors::ConfigError::NonExistentConfigFile => {
                println!("No configuration file is present. Run vssh setup or vssh --config /path/to/file.yml");
                exit(1);
            }
            _ => {
                println!("Failed to load configuration: {}", e);
                exit(1);
            }
        },
    }
}

/// Initialize the API client to interact with Vault
async fn initialize_api(cfg: &Config) -> ApiClient {
    // Generate a client from the configuration
    let client = ApiClient::from_config(cfg);

    // Ensure able to access API
    match client.validate().await {
        Ok(status) => {
            if !status {
                println!("Invalid token, please ensure it is correct and try again");
                exit(1);
            }
        }
        Err(e) => {
            println!("Failed to validate token: {}", e);
            exit(1);
        }
    };

    client
}

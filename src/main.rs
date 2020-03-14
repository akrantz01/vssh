mod api;
mod cli;
mod config;
mod errors;
mod subcommands;
mod util;

use api::ApiClient;
use cli::{Command, Opts, Profiles};
use config::Config;
use std::path::Path;
use structopt::StructOpt;
use util::fail;

#[tokio::main]
async fn main() {
    // Parse cli arguments and parameters
    let cli = Opts::from_args();

    // Add header to command
    leg::head("vssh", Some("🔒"), Some("0.2.0"));

    match cli.cmd {
        Command::RepairConfig => {
            subcommands::repair_config(cli.config);
        }
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
            private_key,
            public_key,
            server,
            options,
        } => {
            let config = load_config(cli.config);
            let client = initialize_api(&config).await;
            subcommands::connect(
                &client,
                role,
                private_key,
                public_key,
                server,
                options.unwrap_or_default(),
            )
            .await;
        }
        Command::Profiles(p) => match p {
            Profiles::Create {
                name,
                username,
                address,
                role,
                private_key,
                public_key,
                options,
            } => {
                let config = load_config(cli.config);
                subcommands::profiles::create(
                    name,
                    username,
                    address,
                    role,
                    private_key,
                    public_key,
                    options.unwrap_or_default(),
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
                public_key,
                options,
            } => {
                let config = load_config(cli.config);
                subcommands::profiles::update(
                    name,
                    username,
                    address,
                    role,
                    private_key,
                    public_key,
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
        Config::read(Path::new(&file))
    } else {
        Config::read_default()
    };

    // Ensure no errors when loading
    match config {
        Ok(c) => c,
        Err(e) => match e {
            errors::ConfigError::NonExistentConfigFile => fail("No configuration file is present. Run vssh setup or vssh --config /path/to/file.json"),
            errors::ConfigError::JsonError(_) => fail("Invalid configuration file format. Run vssh repair-config to fix it"),
            _ => fail(&format!("Failed to load configuration: {}", e))
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
                fail("Invalid token, please ensure it is correct and try again");
            }
        }
        Err(e) => fail(&format!("Failed to validate token: {}", e)),
    };

    client
}

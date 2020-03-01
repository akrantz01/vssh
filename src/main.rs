extern crate clap;
extern crate dirs;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_yaml;
extern crate url;

mod api;
mod cli;
mod config;
mod errors;
mod subcommands;

use api::ApiClient;
use config::Config;
use std::io;
use std::process::exit;

fn main() {
    // Parse cli arguments and parameters
    let matches = cli::generate_cli().get_matches();

    // Attempt to read config file
    let cfg_result = if matches.value_of("config").unwrap_or_default() == "" {
        Config::read_default()
    } else {
        Config::read(matches.value_of("config").unwrap().to_string())
    };

    // Ensure exists
    let cfg = match cfg_result {
        Ok(c) => c,
        Err(e) => match e {
            errors::ConfigError::NonExistentConfigFile => {
                if matches.subcommand_name().unwrap_or_default() != "setup" {
                    println!("No configuration file is present. Run vssh setup or vssh --config /path/to/file.yml");
                    exit(1);
                }
                Config::new_empty()
            }
            _ => {
                println!("Failed to load configuration: {}", e);
                exit(1);
            }
        },
    };

    // Handle the setup subcommand
    if let Some(setup) = matches.subcommand_matches("setup") {
        // Determine whether to run in interactive or non-interactive mode
        if setup.is_present("non-interactive") {
            subcommands::setup::noninteractive(
                setup.value_of("server").unwrap_or_default(),
                !setup.is_present("no-tls"),
                setup.value_of("token").unwrap_or_default(),
                setup.value_of("path").unwrap_or_default(),
                setup.value_of("custom-ca").unwrap_or_default(),
            )
        } else {
            subcommands::setup::interactive();
        }
    }

    // Initialize API client
    let client = ApiClient::from_config(cfg);
    match client.validate() {
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
    }

    // Handle other/non-existent subcommands
    match matches.subcommand_name() {
        None => {
            cli::generate_cli()
                .write_help(&mut io::stdout())
                .expect("Failed to write help");
            println!();
        }
        Some(name) => match name {
            "list" => subcommands::list::list(&client),
            "sign" => {
                if let Some(sign) = matches.subcommand_matches("sign") {
                    subcommands::sign::sign(
                        &client,
                        sign.value_of("ROLE").unwrap_or_default(),
                        sign.value_of("KEY").unwrap_or_default(),
                        sign.value_of("output").unwrap_or_default(),
                    );
                }
            }
            "connect" => {
                if let Some(connect) = matches.subcommand_matches("connect") {
                    subcommands::connect::connect(
                        &client,
                        connect.value_of("ROLE").unwrap_or_default(),
                        connect.value_of("KEY").unwrap_or_default(),
                        connect.value_of("SERVER").unwrap_or_default(),
                        connect.value_of("options").unwrap_or_default(),
                    );
                }
            }
            _ => {}
        },
    }
}

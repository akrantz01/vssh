use clap::{App, Arg, SubCommand};

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "Alex Krantz <alex@alexkrantz.com>";

pub fn generate_cli<'a>() -> App<'a, 'a> {
    App::new("vssh")
        .version(VERSION)
        .author(AUTHOR)
        .about("SSH into a server requiring a certificate signed by a HashiCorp Vault instance")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(setup())
        .subcommand(list())
}

fn setup<'a>() -> App<'a, 'a> {
    SubCommand::with_name("setup")
        .about("Setup the application")
        .arg(
            Arg::with_name("non-interactive")
                .long("non-interactive")
                .help("Run setup non-interactively"),
        )
        .arg(
            Arg::with_name("server")
                .long("server")
                .value_name("SERVER")
                .help("HashiCorp Vault server to connect to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-tls")
                .long("no-tls")
                .help("Disable TLS when connecting to the server"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .value_name("TOKEN")
                .help("Token to use when authenticating")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("default-key")
                .long("default-key")
                .value_name("KEY")
                .help("Default SSH key to sign")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("Path of the SSH CA on the Vault server")
                .takes_value(true),
        )
}

fn list<'a>() -> App<'a, 'a> {
    SubCommand::with_name("list")
        .about("List available roles")
}

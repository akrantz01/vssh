use clap::{App, Arg, SubCommand};

static VERSION: &str = "0.1.0";
static AUTHOR: &str = "Alex Krantz <alex@alexkrantz.com>";

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
        .subcommand(sign())
        .subcommand(connect())
}

fn setup<'a>() -> App<'a, 'a> {
    SubCommand::with_name("setup")
        .about("Generate a configuration file")
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
                .help("Disable TLS verification when connecting to the server"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .value_name("TOKEN")
                .help("Token to use when authenticating")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("PATH")
                .help("Path of the SSH CA on the Vault server")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("custom-ca")
                .long("custom-ca")
                .value_name("PATH")
                .help("Path to the public part of the custom CA")
                .takes_value(true),
        )
}

fn list<'a>() -> App<'a, 'a> {
    SubCommand::with_name("list").about("List available roles")
}

fn sign<'a>() -> App<'a, 'a> {
    SubCommand::with_name("sign")
        .about("Sign an SSH public key")
        .arg(
            Arg::with_name("ROLE")
                .help("Role to sign public key with")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("KEY")
                .help("Public key to be signed")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("File to write the signed certificate to")
                .takes_value(true),
        )
}

fn connect<'a>() -> App<'a, 'a> {
    SubCommand::with_name("connect")
        .about("Connect to a server with an automatically generated signed certificate")
        .arg(
            Arg::with_name("ROLE")
                .help("Role to sign public key with")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("KEY")
                .help("Private key to authenticate with")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("SERVER")
                .help("SSH server connection string")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("options")
                .short("o")
                .long("options")
                .value_name("OPTIONS")
                .help("Extra SSH client options")
                .takes_value(true),
        )
}

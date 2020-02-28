use clap::{Arg, App, SubCommand};

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "Alex Krantz <alex@alexkrantz.com>";

pub fn generate_cli() -> App<'static, 'static> {
    App::new("vssh")
        .version(VERSION)
        .author(AUTHOR)
        .about("SSH into a server requiring a certificate signed by a HashiCorp Vault instance")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
}

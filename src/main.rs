extern crate clap;

mod cli;
mod subcommands;

fn main() {
    // Parse cli arguments and parameters
    let matches = cli::generate_cli().get_matches();

    // Handle the setup subcommand
    if let Some(setup) = matches.subcommand_matches("setup") {
        // Determine whether to run in interactive or non-interactive mode
        if setup.is_present("non-interactive") {
            subcommands::setup::noninteractive(
                setup.value_of("server").unwrap_or_default(),
                !setup.is_present("no-tls"),
                setup.value_of("token").unwrap_or_default(),
                setup.value_of("default-key").unwrap_or_default(),
                setup.value_of("path").unwrap_or_default(),
            )
        } else {
            subcommands::setup::interactive();
        }
    }
}

extern crate clap;

mod cli;

fn main() {
    // Parse cli arguments and parameters
    let matches = cli::generate_cli().get_matches();
}

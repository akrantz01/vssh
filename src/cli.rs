use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vssh",
    author = "Alex Krantz <alex@alexkrantz.com>",
    version = "0.1.0"
)]
/// SSH into a server requiring a certificate signed by a Hashicorp Vault instance
pub struct Opts {
    /// Sets a custom config file
    #[structopt(short, long, env)]
    pub config: Option<String>,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Generate a configuration file
    Setup {
        #[structopt(long)]
        /// Read configuration values from command line flags
        non_interactive: bool,
        #[structopt(long)]
        /// HashiCorp Vault server to connect to
        server: Option<String>,
        #[structopt(long)]
        /// Disable TLS verification when connecting to the server
        no_tls: bool,
        #[structopt(long)]
        /// Token to use when authenticating
        token: Option<String>,
        #[structopt(long)]
        /// Path of the SSH CA on the Vault server
        path: Option<String>,
        #[structopt(long)]
        /// Path to the public part of the custom certificate authority
        custom_ca: Option<String>,
    },
    /// List available roles
    List,
    /// Sign an SSH public key
    Sign {
        /// Role to sign public key with
        role: String,
        /// Public key to be signed
        key: String,
        /// File to write the signed certificate to
        #[structopt(short, long)]
        output: Option<String>,
    },
    /// Connect to a server with an automatically generated signed certificate
    Connect {
        /// Role to sign public key with
        role: String,
        /// Private key to authenticate with
        key: String,
        /// SSH server connection string
        server: String,
        #[structopt(short, long)]
        /// Extra SSH client options
        options: Option<String>,
    },
}

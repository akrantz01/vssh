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
    /// Repair a user's configuration file
    RepairConfig,
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
    #[structopt(alias = "ls")]
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
        #[structopt(short, long)]
        /// Private key to authenticate with
        private_key: Option<String>,
        #[structopt(short = "k", long)]
        /// Alternative public key to use
        public_key: Option<String>,
        /// SSH server connection string
        server: String,
        #[structopt(short, long)]
        /// Extra SSH client options
        options: Option<String>,
    },
    /// Manage server profiles
    Profiles(Profiles),
}

#[derive(Debug, StructOpt)]
pub enum Profiles {
    #[structopt(alias = "new")]
    /// Create a new profile for a server
    ///
    /// Uses the current user's username if none is specified
    Create {
        /// Name for the profile
        name: String,
        #[structopt(short, long)]
        /// Username to connect as, defaults to current user
        username: Option<String>,
        /// IP address or FQDN of the server
        address: String,
        /// Role to sign the public key as
        role: String,
        #[structopt(short, long)]
        /// Private key to use for authentication
        private_key: Option<String>,
        #[structopt(short = "k", long)]
        /// Public key to be signed
        public_key: Option<String>,
        #[structopt(short, long)]
        /// Other options to pass to the ssh command
        options: Option<String>,
    },
    #[structopt(alias = "inspect")]
    /// Get the details about a profile
    Read {
        /// Name of the profile
        name: String,
    },
    #[structopt(alias = "ls")]
    /// Get a list of all the profiles
    List,
    /// Update a profile
    Update {
        /// Name of the profile
        name: String,
        #[structopt(short, long)]
        /// New username to use
        username: Option<String>,
        #[structopt(short, long)]
        /// New IP address or FQDN to use
        address: Option<String>,
        #[structopt(short, long)]
        /// New role to use for signing
        role: Option<String>,
        #[structopt(short, long)]
        /// New private key to authenticate with
        private_key: Option<String>,
        #[structopt(short = "k", long)]
        /// New public key to sign
        public_key: Option<String>,
        #[structopt(short, long)]
        /// New options to pass to ssh
        options: Option<String>,
    },
    /// Delete a profile
    Delete {
        /// Name of the profile
        name: String,
    },
    /// Connect to a profile
    Connect {
        /// Name of the profile
        name: String,
    },
}

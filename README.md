# Vault SSH
Easily connect to a server that requires a certificate signed by a HashiCorp Vault server.

## Features
- List signable roles
- Sign an SSH public key
- Automatically sign a key and SSH into a server

## CLI Reference
Usage: `vssh [OPTIONS] [SUBCOMMAND`

Options:
  - `-c, --config <FILE>`: Sets a custom config file

Subcommands:
  - `connect`: Connect to a server with a signed certificate
  - `list`: List available roles
  - `setup`: Setup the application
  - `sign`: Sign an SSH public key

### Connect
Connect to a server with an automatically generated signed certificate

Usage: `vssh connect [OPTIONS] <ROLE> <KEY> <SERVER>`

Options:
  - `-o, --options <OPTIONS>`: Extra SSH client options

Arguments:
  - `<ROLE>`: Role to sign public key with
  - `<KEY>`: Private key to authenticate with
  - `<SERVER>`: SSH server connection string

### List
List available roles

Usage: `vssh list`

### Setup
Generate a configuration file

Usage: `vssh setup [FLAGS] [OPTIONS]`

Options:
  - `--no-tls`: Disable verification TLS when connecting to the server
  - `--non-interactive`: Run setup non-interactively
  - `--path <PATH>`: Path of the SSH CA on the Vault server
  - `--server <SERVER>`: HashiCorp Vault server to connect to
  - `--token <TOKEN>`: Token to use when authenticating
  - `--custom-ca <PATH>`: Path to the public part of the custom CA

### Sign
Sign an SSH public key

Usage: `vssh sign [OPTIONS] <ROLE> <KEY>`

Options:
  - `-o, --output <FILE>`: File to write the signed certificate to

Arguments:
  - `<ROLE>`: Role to sign public key with
  - `<KEY>`: Public key to be signed

## TODO:
  - [ ] add profiles
  - [x] custom CAs
  - [x] self-signed certificates
  - [x] use non-blocking version of `reqwest`

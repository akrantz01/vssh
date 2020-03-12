![Crates.io](https://img.shields.io/crates/d/vssh)
![Crates.io](https://img.shields.io/crates/v/vssh)

# Vault SSH
Easily SSH into a server that requires a certificate signed by HashiCorp Vault.

## Features
- List signable roles
- Sign an SSH public key
- Automatically sign a key and SSH into a server

## Installation
`vssh` can be installed from [crates.io](https://crates.io) using `cargo install vssh`.
Then run `vssh setup` to configure it.

## TODO:
  - [x] add profiles
  - [x] custom CAs
  - [x] self-signed certificates
  - [x] use non-blocking version of `reqwest`

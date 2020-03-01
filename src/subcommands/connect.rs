use crate::api::ApiClient;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::{canonicalize, OpenOptions, read_to_string, remove_file};
use std::io::{ErrorKind, Write};
use std::iter;
use std::process::{Command, exit};

pub fn connect<'a>(client: &ApiClient, role: &'a str, key: &'a str, server: &'a str, options: &'a str) {
    // Convert relative to absolute path and ensure exists
    let path = match canonicalize(key) {
        Ok(path) => path,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => println!("Private key '{}' does not exist", key),
                _ => println!("Failed to convert relative to absolute path: {}", e)
            }
            exit(1);
        }
    };

    // Read public key to file
    let contents = match read_to_string(format!("{}.pub", path.as_path().to_str().unwrap())) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Failed to read public key '{}': {}", path.as_path().to_str().unwrap(), e);
            exit(1);
        }
    };

    // Sign the public key
    let signed = match client.sign(role.to_string(), contents) {
        Ok(signed) => signed,
        Err(e) => {
            println!("Failed to sign public key: {}", e);
            exit(1);
        }
    };

    // Create output file
    let name = random_string(16);
    let mut file = match OpenOptions::new().write(true).create(true).open(format!("/tmp/{}", name)) {
        Ok(file) => file,
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => println!("Cannot write signed public key: permission denied"),
                _ => println!("Failed to open output file: {}", e)
            };
            exit(1);
        }
    };

    // Write to file
    match file.write_all(signed.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to write to output file: {}", e);
            exit(1);
        }
    }

    // Run command
    let mut child = match Command::new("ssh").arg("-i").arg(key).arg("-i").arg(format!("/tmp/{}", name)).arg(server).args(options.split_whitespace().collect::<Vec<&str>>()).spawn() {
        Ok(child) => child,
        Err(e) => {
            println!("Failed to start ssh command: {}", e);
            exit(1);
        }
    };

    // Bring to foreground
    child.wait().expect("Failed to wait on child");

    // Remove signed certificate
    match remove_file(format!("/tmp/{}", name)) {
        Ok(_) => {},
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => println!("Cannot remove signed certificate: permission denied"),
                _ => println!("Failed to remove signed certificate: {}", e)
            };
            exit(1);
        }
    };
}

/// Generate a random string of alphanumeric characters
fn random_string(n: usize) -> String {
    iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .take(n)
        .collect()
}

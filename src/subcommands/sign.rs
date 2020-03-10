use crate::api::ApiClient;
use std::fs::{canonicalize, read_to_string, OpenOptions};
use std::io::{ErrorKind, Write};
use std::process::exit;

pub async fn sign<'a>(client: &ApiClient, role: String, key: String, output: String) {
    // Convert relative to absolute path and ensure exists
    let path = match canonicalize(&key) {
        Ok(path) => path,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => println!("Key '{}' does not exist", key),
                _ => println!("Failed to convert relative to absolute path: {}", e),
            }
            exit(1);
        }
    };

    // Read file
    let contents = match read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Failed to read public key: {}", e);
            exit(1);
        }
    };

    // Sign the public key
    let signed = match client.sign(role.to_string(), contents).await {
        Ok(signed) => signed,
        Err(e) => {
            println!("Failed to sign public key: {}", e);
            exit(1);
        }
    };

    // Output to stdout if no file
    if output == "" {
        println!("{}", signed);
    }

    // Create output file if not exists
    let mut file = match OpenOptions::new().write(true).create(true).open(&output) {
        Ok(file) => file,
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    println!("Cannot write signed public key: permission denied")
                }
                _ => println!("Failed to open output file: {}", e),
            };
            exit(1);
        }
    };

    // Write to file
    match file.write_all(signed.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to write to output file: {}", e);
            exit(1);
        }
    };

    println!("Wrote signed public key to: {}", output);
}

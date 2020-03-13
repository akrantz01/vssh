use crate::api::ApiClient;
use crate::util::fail;
use std::fs::{canonicalize, read_to_string, OpenOptions};
use std::io::{ErrorKind, Write};

pub async fn sign<'a>(client: &ApiClient, role: String, key: String, output: String) {
    // Convert relative to absolute path and ensure exists
    let path = match canonicalize(&key) {
        Ok(path) => path,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => fail(&format!("Key '{}' does not exist", key)),
            _ => fail(&format!(
                "Failed to convert relative to absolute path: {}",
                e
            )),
        },
    };

    // Read file
    let contents = match read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => fail(&format!("Failed to read public key: {}", e)),
    };

    // Sign the public key
    let signed = match client.sign(role.to_string(), contents).await {
        Ok(signed) => signed,
        Err(e) => fail(&format!("Failed to sign public key: {}", e)),
    };

    leg::success("Signed public key with role", None, None);

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
                    fail("Cannot write signed public key: permission denied")
                }
                _ => fail(&format!("Failed to open output file: {}", e)),
            };
        }
    };

    // Write to file
    match file.write_all(signed.as_bytes()) {
        Ok(_) => {}
        Err(e) => fail(&format!("Failed to write to output file: {}", e)),
    };

    leg::success(
        &format!("Wrote signed public key to: {}", output),
        None,
        None,
    );
}

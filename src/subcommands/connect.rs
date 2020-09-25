use crate::api::ApiClient;
use crate::util::fail;
use std::fs::{canonicalize, read_to_string};
use std::io::{ErrorKind, Write};
use std::process::Command;

pub async fn connect<'a>(
    client: &ApiClient,
    role: String,
    private_key: Option<String>,
    public_key: Option<String>,
    server: String,
    sftp: bool,
    options: String,
) {
    // Use provided private key or default
    let private_key = if let Some(private_key) = private_key {
        private_key
    } else {
        String::from("~/.ssh/id_rsa")
    };

    // Use provided public key or default
    let public_key = if let Some(public_key) = public_key {
        public_key
    } else {
        format!("{}.pub", private_key)
    };

    // Ensure private key exists
    let private_path = ensure_exists(&private_key);

    // Ensure public key exists
    let public_path = ensure_exists(&public_key);

    // Read public key to file
    let contents = match read_to_string(&public_path) {
        Ok(contents) => contents,
        Err(e) => fail(&format!(
            "Failed to read public key '{}': {}",
            public_path.as_path().to_str().unwrap(),
            e
        )),
    };

    // Sign the public key
    let signed = match client.sign(role.to_string(), contents).await {
        Ok(signed) => signed,
        Err(e) => fail(&format!("Failed to sign public key: {}", e)),
    };

    leg::success("Signed public key with role", None, None);

    // Create output file
    let mut file = match tempfile::NamedTempFile::new() {
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
    }

    leg::success("Wrote signed public key to temporary file", None, None);

    // Close the temporary file, but don't remove it to be passed to ssh/sftp
    let file_path = file.into_temp_path();

    // Check whether to run SSH or SFTP
    let command = if sftp { "sftp" } else { "ssh" };

    // Run command
    let mut child = match Command::new(command)
        .arg("-i")
        .arg(private_path)
        .arg("-i")
        .arg(&file_path)
        .arg(server)
        .args(options.split_whitespace().collect::<Vec<&str>>())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => fail(&format!("Failed to start ssh command: {}", e)),
    };

    leg::wait("Running SSH command...", None, None);

    // Bring to foreground
    child.wait().expect("Failed to wait on child");

    // Remove signed certificate
    match file_path.close() {
        Ok(_) => leg::success("Cleaned up signed public key", None, None),
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => {
                    fail("Cannot remove signed certificate: permission denied")
                }
                _ => fail(&format!("Failed to remove signed certificate: {}", e)),
            };
        }
    };
}

/// Ensure a file exists and also convert it to an absolute path if it was not
fn ensure_exists(path: &str) -> std::path::PathBuf {
    match canonicalize(path) {
        Ok(path) => path,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => fail(&format!("File '{}' does not exist", path)),
                _ => fail(&format!(
                    "Failed to convert relative to absolute path: {}",
                    e
                )),
            };
        }
    }
}

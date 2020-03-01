use crate::api::ApiClient;
use std::process::exit;

pub async fn list(client: &ApiClient) {
    // Request roles
    let roles = match client.list_roles().await {
        Ok(roles) => roles,
        Err(e) => {
            println!("Failed to list roles: {}", e);
            exit(1);
        }
    };

    // Display roles
    for role in roles.iter() {
        println!("{}", role);
    }
}

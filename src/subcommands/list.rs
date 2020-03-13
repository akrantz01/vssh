use crate::api::ApiClient;
use crate::util::fail;

pub async fn list(client: &ApiClient) {
    // Request roles
    let roles = match client.list_roles().await {
        Ok(roles) => roles,
        Err(e) => fail(&format!("Failed to list roles: {}", e)),
    };

    leg::success("Retrieved list of roles to sign as", None, None);

    // Display roles
    for role in roles.iter() {
        println!("{}", role);
    }
}

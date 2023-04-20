use crate::oauth::{AuthClient, AuthConfig};

mod oauth;
mod photos;

// #[tokio::main]
fn main() {
    let auth_cfg = AuthConfig::new();
    let auth_client = AuthClient::new(auth_cfg);
    let token = auth_client
        .oauth(photos::READ_SCOPE.to_string())
        .expect("Authentication failed");

    println!("Authentication done: {:?}", token);

    //TODO add listing albums and photos here
    //TODO download photos
    // photos::example().;
}

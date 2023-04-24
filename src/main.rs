use crate::oauth::{AuthClient, AuthConfig};
use crate::photos::Client;

mod media;
mod oauth;
mod photos;

#[tokio::main]
async fn main() {
    let auth_cfg = AuthConfig::new();
    let auth_client = AuthClient::new(auth_cfg);
    let token = auth_client
        .oauth(photos::READ_SCOPE.to_string())
        .expect("Authentication failed");

    let photos = Client::new(token);

    let result = photos.list_media().await;

    println!("{:?}", result)
}

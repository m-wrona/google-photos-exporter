use anyhow;
use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, DeviceAuthorizationUrl,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use url::Url;

use crate::oauth::{AuthClient, AuthConfig};

mod oauth;
mod photos;

// #[tokio::main]
fn main() {
    // example().await;
    let auth_cfg = AuthConfig::new();
    let auth_client = AuthClient::new(auth_cfg);
    let token = auth_client
        .oauth(photos::READ_SCOPE.to_string())
        .expect("Authentication failed");
}

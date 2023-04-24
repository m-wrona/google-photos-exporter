use oauth2::basic::{BasicTokenResponse, BasicTokenType};
use oauth2::TokenResponse;
use reqwest::header::AUTHORIZATION;
use reqwest::Client as HttpClient;

use crate::media::MediaItems;

pub const READ_SCOPE: &str = "https://www.googleapis.com/auth/photoslibrary.readonly";

const ROOT_URL: &str = "https://photoslibrary.googleapis.com/v1";

pub struct Client {
    token: BasicTokenResponse,
    client: HttpClient,
}

impl Client {
    pub fn new(token: BasicTokenResponse) -> Self {
        let client = HttpClient::new();
        Self { token, client }
    }

    fn auth_header(&self) -> String {
        format!(
            "{} {}",
            BasicTokenType::Bearer.as_ref(),
            self.token.access_token().secret()
        )
    }

    pub async fn list_media(&self) -> Result<MediaItems, Box<dyn std::error::Error + Send + Sync>> {
        let result = self
            .client
            .get(format!("{}/mediaItems", ROOT_URL))
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?
            .json::<MediaItems>()
            .await?;

        Ok(result)
    }
}

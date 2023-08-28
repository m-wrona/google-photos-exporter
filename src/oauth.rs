use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::string::ToString;

use oauth2::{basic::BasicClient, PkceCodeVerifier};
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use oauth2::basic::BasicTokenResponse;
use oauth2::reqwest::http_client;
use oauth2::reqwest::async_http_client;
use url::Url;

const AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const REVOKE_URL: &str = "https://oauth2.googleapis.com/revoke";
const CALLBACK_URL: &str = "127.0.0.1:8080";
const REDIRECT_URL: &str = "http://localhost:8080";

#[derive(Debug)]
pub struct AuthConfig {
    client_id: ClientId,
    client_secret: ClientSecret,
}

pub struct AuthClient {
    client: BasicClient,
}

impl AuthConfig {
    pub fn new() -> Self {
        let client_id = ClientId::new(
            env::var("GOOGLE_CLIENT_ID")
                .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
        );
        let client_secret = ClientSecret::new(
            env::var("GOOGLE_CLIENT_SECRET")
                .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );

        Self {
            client_id,
            client_secret,
        }
    }
}

impl AuthClient {
    pub fn new(cfg: AuthConfig) -> Self {
        let auth_url =
            AuthUrl::new(AUTH_URL.to_string()).expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new(TOKEN_URL.to_string()).expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            cfg.client_id,
            Some(cfg.client_secret),
            auth_url,
            Some(token_url),
        )
            .set_redirect_uri(RedirectUrl::new(REDIRECT_URL.to_string()).expect("Invalid redirect URL"))
            .set_revocation_uri(
                RevocationUrl::new(REVOKE_URL.to_string()).expect("Invalid revocation endpoint URL"),
            );

        Self { client }
    }

    pub async fn oauth(&self, scope: String) -> Result<BasicTokenResponse, String> {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, _) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(scope))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        println!("Open this URL in your browser:\n{}\n", authorize_url);

        self.wait_for_callback(pkce_code_verifier).await
    }

    async fn wait_for_callback(
        &self,
        pkce_code_verifier: PkceCodeVerifier,
    ) -> Result<BasicTokenResponse, String> {
        let listener = TcpListener::bind(CALLBACK_URL.to_string()).unwrap();
        let connection = listener.incoming().next().expect("No new connection");

        return match connection {
            Ok(mut stream) => {
                let code;
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let (key, _) = pair;
                            key == "code"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    code = AuthorizationCode::new(value.into_owned());
                }

                let message = "Auth done. You can close this window now.";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();

                let token_response = self
                    .client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_code_verifier)
                    .request_async(async_http_client)
                    .await;

                println!("Authentication done - success: {}", token_response.is_ok());

                token_response.map_err(|err| err.to_string())
            }

            Err(err) => Err(format!("new connection issue: {:?}", err)),
        };
    }
}

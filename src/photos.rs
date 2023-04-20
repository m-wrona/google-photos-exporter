use std::string::ToString;

use hyper::body::HttpBody as _;
use hyper::Client;
use tokio::io::{stdout, AsyncWriteExt as _};

pub const READ_SCOPE: &'static str = "https://www.googleapis.com/auth/photoslibrary.readonly";

pub async fn example() {
    println!("Hello, world!");
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse().unwrap();

    // Await the response...
    let mut resp = client.get(uri).await.unwrap();

    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk.unwrap()).await.unwrap();
    }
}

use crate::oauth::example;

mod oauth;

#[tokio::main]
async fn main() {
    example().await;
}

use twitch_api::{helix::streams::GetStreamsRequest, HelixClient};
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {}", err);
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {}", cause);
            e = cause;
        }
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenvy::dotenv();
    let mut args = std::env::args().skip(1);
    let client: HelixClient<reqwest::Client> = HelixClient::new();
    let token = UserToken::from_existing(
        &client,
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(AccessToken::new)
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
        None,
    )
    .await
    .unwrap();

    let req = GetStreamsRequest::user_login(args.next().unwrap());

    let response = client.req_get(req, &token).await.unwrap();

    println!("Stream information:\n\t{:?}", response.data);
    Ok(())
}

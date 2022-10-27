use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, TwitchToken, UserToken};

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
    .await?;

    let broadcaster_id = token.validate_token(&client).await?.user_id.unwrap();

    let req = twitch_api::helix::channels::ModifyChannelInformationRequest::broadcaster_id(
        &*broadcaster_id,
    );

    println!("scopes: {:?}", token.scopes());
    let response = client
        .req_patch(
            req,
            twitch_api::helix::channels::ModifyChannelInformationBody::builder()
                .title("Hello World!")
                .build(),
            &token,
        )
        .await?;
    println!("{:?}", response);

    Ok(())
}

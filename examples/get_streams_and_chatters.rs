use twitch_api::helix::streams::GetStreamsRequest;
use twitch_api::TwitchClient;
use twitch_oauth2::{AccessToken, UserToken};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let mut args = std::env::args().skip(1);
    let client: TwitchClient<reqwest::Client> = TwitchClient::new();
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

    let streams: Vec<String> = args.collect();
    let req = GetStreamsRequest::default();

    let response = client.helix.req_get(req, &token).await.unwrap();

    // Note: This will fetch chatters in the current most viewed stream, might spam your console a bit.
    println!("GetStreams:\n\t{:?}", response.data);
    if let Some(stream) = streams.get(0) {
        println!(
            "{:?}",
            client.tmi.get_chatters(stream.as_str().into()).await
        );
    } else if let Some(stream) = response.data.get(0).map(|stream| &stream.user_login) {
        println!("{:?}", client.tmi.get_chatters(stream).await);
    }
}

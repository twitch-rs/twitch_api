use twitch_api2::TMIClient;
use twitch_api2::{helix::streams::GetStreamsRequest, HelixClient};
use twitch_oauth2::{AccessToken, UserToken};

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let mut args = std::env::args().skip(1);
    let token = UserToken::from_existing(
        twitch_oauth2::client::surf_http_client,
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

    let client = reqwest::Client::new();
    let client_tmi = TMIClient::with_client(client.clone());
    let client_helix = HelixClient::with_client(client);

    let streams: Vec<String> = args.collect();
    let req = GetStreamsRequest::builder().build();

    let response = client_helix.req_get(req, &token).await.unwrap();

    // Note: This will fetch chatters in the current most viewed stream, might spam your console a bit.
    println!("GetStreams:\n\t{:?}", response.data);
    if let Some(stream) = streams.get(0) {
        println!("{:?}", client_tmi.get_chatters(stream).await);
    } else if let Some(stream) = response.data.get(0).map(|stream| &stream.user_name) {
        println!("{:?}", client_tmi.get_chatters(&stream).await);
    }
}

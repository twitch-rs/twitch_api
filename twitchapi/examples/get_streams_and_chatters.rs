use twitch_api2::{helix::streams::GetStreamsRequest, HelixClient, TMIClient};
use twitch_oauth2::{AccessToken, Scope, UserToken};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let mut args = std::env::args().skip(1);
    let scopes = Scope::all();
    let token = UserToken::from_existing(
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(|t| AccessToken::new(t))
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
    )
    .await
    .unwrap();

    let client = HelixClient::new();
    let client_tmi = TMIClient::with_client(client.clone_client());

    let mut streams: Vec<String> = args.map(|a| a.to_string()).collect();
    let req = GetStreamsRequest::builder().build();

    let response = client.req_get(req, &token).await.unwrap();

    println!("GetStreams:\n\t{:?}", response.data);
    if let Some(stream) = streams.get(0) {
        println!("{:?}", client_tmi.get_chatters(stream).await);
    } else if let Some(ref stream) = response.data.get(0).map(|stream| &stream.user_name) {
        println!("{:?}", client_tmi.get_chatters(&stream).await);
    }
}

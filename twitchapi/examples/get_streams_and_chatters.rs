#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let mut args = std::env::args().skip(1);
    let scopes = twitch_oauth2::Scope::all();
    let token = twitch_oauth2::UserToken::from_existing(
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next()).map(|t| twitch_oauth2::AccessToken::new(t))
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
    )
    .await
    .unwrap();

    let client = twitch_api2::HelixClient::new(Box::new(token));
    let client_tmi = twitch_api2::TMIClient::new_with_client(client.clone_client());

    let mut streams: Vec<String> = args.map(|a| a.to_string()).collect();

    let response = client
        .get_streams(|b| b.user_login(streams.clone()).build())
        .await
        .unwrap();

    println!("GetStreams:\n\t{:?}", response.data);
    println!("\t{:?}", response.get_next(&client).await);
    if let Some(stream) = streams.get(0) {
        println!("{:?}", client_tmi.get_chatters(stream).await);
    } else if let Some(ref stream) = response.data.get(0).map(|stream| &stream.user_name) {
        println!("{:?}", client_tmi.get_chatters(&stream).await);
    }
}

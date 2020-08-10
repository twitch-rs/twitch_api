#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let scopes = twitch_oauth2::Scope::all();
    let token = twitch_oauth2::AppAccessToken::get_app_access_token(
        std::env::var("HELIX_CLIENT_ID").expect("Please set env: HELIX_CLIENT_ID"),
        std::env::var("HELIX_CLIENT_SECRET").expect("Please set env: HELIX_CLIENT_SECRET"),
        scopes,
    )
    .await
    .unwrap();
    let client = twitch_api2::HelixClient::new(Box::new(token));
    let response = client
        .get_streams(|b| {
            b.user_login(vec!["LCS".to_string(), "asmongold".to_string()])
                .build()
        })
        .await
        .unwrap();
    println!("{:?}", response.data);
    println!("{:?}", response.get_next(&client).await);

    let client_tmi = twitch_api2::TMIClient::new_with_client(client.clone_client());
    println!("{:?}", client_tmi.get_chatters("desertheartsrecords").await);
}

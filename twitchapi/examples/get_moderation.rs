#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let mut args = std::env::args().skip(1);
    let scopes = twitch_oauth2::Scope::all();
    let token = twitch_oauth2::UserToken::from_existing(
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(|t| twitch_oauth2::AccessToken::new(t))
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
    )
    .await
    .unwrap();

    let client = twitch_api2::HelixClient::new(Box::new(token));
    let client_tmi = twitch_api2::TMIClient::new_with_client(client.clone_client());

    let moderators_req = twitch_api2::helix::moderation::get_moderators::GetModeratorsRequest {
        broadcaster_id: client.validate_token().await.unwrap().user_id.unwrap(),
        after: None,
    };

    let mut response = client.req_get(moderators_req).await.unwrap();

    println!("====Moderators====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let moderator_events_req = twitch_api2::helix::moderation::GetModeratorsEventsRequest {
        broadcaster_id: client.validate_token().await.unwrap().user_id.unwrap(),
        user_id: vec![],
        after: None,
    };

    let mut response = client.req_get(moderator_events_req).await.unwrap();

    // /mod and /unmod events
    println!("====Moderator Events====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let banned_users_req = twitch_api2::helix::moderation::GetBannedUsersRequest {
        broadcaster_id: client.validate_token().await.unwrap().user_id.unwrap(),
        user_id: vec![],
        after: None,
    };
    let mut response = client.req_get(banned_users_req).await.unwrap();

    println!("====Banned users====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let banned_users_req = twitch_api2::helix::moderation::GetBannedEventsRequest {
        broadcaster_id: client.validate_token().await.unwrap().user_id.unwrap(),
        user_id: vec![],
        after: None,
        first: Some(10),
    };
    let mut response = client.req_get(banned_users_req).await.unwrap();

    println!("====Last 10 Banned Events====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }
}

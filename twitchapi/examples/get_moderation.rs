use twitch_api2::{
    helix::moderation::{
        get_moderators::GetModeratorsRequest, GetBannedEventsRequest, GetBannedUsersRequest,
        GetModeratorsEventsRequest,
    },
    HelixClient, TMIClient,
};
use twitch_oauth2::{AccessToken, Scope, UserToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
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
    .await?;

    let client = HelixClient::new(Box::new(token));
    let client_tmi = TMIClient::new_with_client(client.clone_client());

    let moderators_req = GetModeratorsRequest {
        broadcaster_id: client.validate_token().await?.user_id,
        after: None,
    };

    let mut response = client.req_get(moderators_req).await?;

    println!("====Moderators====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let moderator_events_req = GetModeratorsEventsRequest {
        broadcaster_id: client.validate_token().await?.user_id,
        user_id: vec![],
        after: None,
    };

    let mut response = client.req_get(moderator_events_req).await?;

    // /mod and /unmod events
    println!("====Moderator Events====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let banned_users_req = GetBannedUsersRequest {
        broadcaster_id: client.validate_token().await?.user_id,
        user_id: vec![],
        after: None,
    };
    let mut response = client.req_get(banned_users_req).await?;

    println!("====Banned users====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    let banned_users_req = GetBannedEventsRequest {
        broadcaster_id: client.validate_token().await?.user_id,
        user_id: vec![],
        after: None,
        first: Some(10),
    };
    let mut response = client.req_get(banned_users_req).await?;

    println!("====Last 10 Banned Events====\n{:?}", response.data);
    while let Ok(Some(new_response)) = response.get_next(&client).await {
        response = new_response;
        println!("{:?}", response.data);
    }
    Ok(())
}

use futures::TryStreamExt;
use twitch_api2::{
    helix::moderation::{GetBannedEventsRequest, GetBannedUsersRequest, GetModeratorEventsRequest},
    HelixClient,
};
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
    let _ = dotenv::dotenv();
    let mut args = std::env::args().skip(1);
    let client: HelixClient<surf::Client> = HelixClient::new();
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

    let broadcaster_id = token.user_id.as_str();

    println!("====Moderators====");
    println!(
        "{:?}",
        client
            .get_moderators_in_channel_from_id(broadcaster_id, &token)
            .try_collect::<Vec<_>>()
            .await?,
    );

    println!("====Last 20 Moderator Events====");
    let moderator_events_req = GetModeratorEventsRequest::builder()
        .broadcaster_id(broadcaster_id)
        .build();

    let mut response = client.req_get(moderator_events_req, &token).await?;
    println!("{:?}", response.data);

    // /mod and /unmod events
    while let Ok(Some(new_response)) = response.get_next(&client, &token).await {
        response = new_response;
        println!("{:?}", response.data);
    }

    println!("====Banned users====");
    let banned_users_req = GetBannedUsersRequest::builder()
        .broadcaster_id(broadcaster_id)
        .build();
    let mut response = client.req_get(banned_users_req, &token).await?;
    println!(
        "{:?}",
        response
            .data
            .iter()
            .map(|user| &user.user_name)
            .collect::<Vec<_>>()
    );

    while let Ok(Some(new_response)) = response.get_next(&client, &token).await {
        response = new_response;
        println!(
            "{:?}",
            response
                .data
                .iter()
                .map(|user| &user.user_name)
                .collect::<Vec<_>>()
        );
    }

    println!("====Last 10 Banned Events====");
    let banned_events_req = GetBannedEventsRequest::builder()
        .broadcaster_id(broadcaster_id)
        .first(Some(10))
        .build();
    let mut response = client.req_get(banned_events_req, &token).await?;
    println!("{:?}", response.data);

    while let Ok(Some(new_response)) = response.get_next(&client, &token).await {
        response = new_response;
        println!("{:?}", response.data);
    }
    Ok(())
}

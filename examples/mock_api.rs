use futures::TryStreamExt;
use twitch_api::helix;
use twitch_api::types;
use twitch_api::HelixClient;
use twitch_oauth2::Scope;
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
    let _ = dotenv::dotenv(); // Eat error

    let client: HelixClient<reqwest::Client> = HelixClient::default();

    let mut args = std::env::args().skip(1);
    std::env::var("TWITCH_OAUTH2_URL")
        .ok()
        .or_else(|| args.next())
        .map(|t| std::env::set_var("TWITCH_OAUTH2_URL", &t))
        .expect("Please set env: TWITCH_OAUTH2_URL or pass url as first argument");

    let client_id = std::env::var("MOCK_CLIENT_ID")
        .ok()
        .or_else(|| args.next())
        .map(twitch_oauth2::ClientId::new)
        .expect("Please set env: MOCK_CLIENT_ID or pass client id as an argument");

    let client_secret = std::env::var("MOCK_CLIENT_SECRET")
        .ok()
        .or_else(|| args.next())
        .map(twitch_oauth2::ClientSecret::new)
        .expect("Please set env: MOCK_CLIENT_SECRET or pass client secret as an argument");

    let user_id = std::env::var("MOCK_USER_ID")
        .ok()
        .or_else(|| args.next())
        .map(types::UserId::new)
        .expect("Please set env: MOCK_USER_ID or pass user_id as an argument");

    let token = twitch_oauth2::UserToken::mock_token(
        &client,
        client_id,
        client_secret,
        &user_id,
        vec![
            Scope::ModerationRead,
            Scope::UserReadFollows,
            Scope::ChannelReadSubscriptions,
        ],
    )
    .await?;

    let user = client
        .get_user_from_id(&*user_id, &token)
        .await?
        .expect("no user found");

    let _channel = client
        .get_channel_from_id(&*user_id, &token)
        .await?
        .expect("no channel found");
    let _channel = client
        .get_channel_from_id(user.id.clone(), &token)
        .await?
        .expect("no channel found");

    let _s: Vec<_> = client
        .search_categories("Just", &token)
        .try_collect()
        .await?;
    let _s: Vec<_> = client
        .search_channels("Sample", true, &token)
        .try_collect()
        .await?;
    let search: Vec<_> = client
        .search_channels("e", false, &token)
        .try_collect()
        .await?;
    dbg!(search.get(0));
    let _total = client
        .get_total_followers_from_id(search.get(0).unwrap().id.clone(), &token)
        .await?;
    dbg!(_total);
    let streams: Vec<_> = client.get_followed_streams(&token).try_collect().await?;
    dbg!(streams);
    let subs: Vec<_> = client
        .get_broadcaster_subscriptions(&token)
        .try_collect()
        .await?;
    dbg!(subs);
    moderation(&client, &user_id, &token).await?;
    Ok(())
}

pub async fn moderation<'a, C: twitch_api::HttpClient<'a> + Sync>(
    client: &'a twitch_api::HelixClient<'a, C>,
    broadcaster_id: &'a types::UserIdRef,
    token: &'a twitch_oauth2::UserToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use twitch_api::helix::moderation::*;
    println!("====Moderators====");
    println!(
        "{:?}",
        client
            .get_moderators_in_channel_from_id(broadcaster_id, token)
            .try_collect::<Vec<_>>()
            .await?,
    );

    println!("====Banned users====");
    let banned_users_req = GetBannedUsersRequest::builder()
        .broadcaster_id(broadcaster_id)
        .build();
    let mut response = client.req_get(banned_users_req, token).await?;
    println!(
        "{:?}",
        response
            .data
            .iter()
            .map(|user| &user.user_name)
            .collect::<Vec<_>>()
    );

    while let Ok(Some(new_response)) = response.get_next(client, token).await {
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

    Ok(())
}

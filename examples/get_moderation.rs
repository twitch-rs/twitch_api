use futures::TryStreamExt;
use twitch_api::{helix::moderation::GetBannedUsersRequest, HelixClient};
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

    println!("====Banned users====");
    println!(
        "{:?}",
        client
            .get_banned_users_in_channel_from_id(broadcaster_id, &token)
            .try_collect::<Vec<_>>()
            .await?,
    );

    Ok(())
}

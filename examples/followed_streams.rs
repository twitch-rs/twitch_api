use twitch_api2::helix::HelixClient;
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {}", err);
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {:?}", cause);
            e = cause;
        }
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenv::dotenv();
    let mut args = std::env::args().skip(1);
    let token = UserToken::from_existing(
        twitch_oauth2::client::reqwest_http_client,
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

    let client: HelixClient<'static, reqwest::Client> = HelixClient::new();

    let streams = client.get_followed_streams(&token).await?;
    let games = client
        .get_games_by_id(
            streams
                .iter()
                .map(|s| s.game_id.clone())
                .collect::<Vec<_>>()
                .as_slice(),
            &token,
        )
        .await?;

    println!(
        "{}",
        streams
            .iter()
            .map(|s| format!(
                "{user_name}: [{game}] | {title}",
                user_name = s.user_name,
                game = games.get(&s.game_id).map(|c| c.name.as_str()).unwrap_or(""),
                title = s.title
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}

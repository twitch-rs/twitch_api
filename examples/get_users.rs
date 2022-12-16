use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {err}");
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {cause}");
            e = cause;
        }
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenvy::dotenv();
    let mut args = std::env::args().skip(1);
    let client: HelixClient<reqwest::Client> = HelixClient::default();
    let token = std::env::var("TWITCH_TOKEN")
        .ok()
        .or_else(|| args.next())
        .map(AccessToken::new)
        .expect("Please set env: TWITCH_TOKEN or pass token as first argument");
    let token = UserToken::from_existing(&client, token, None, None).await?;

    let user = client
        .get_user_from_id(&*args.next().unwrap(), &token)
        .await?
        .expect("no user found");

    println!("User information:\n\t{:#?}", user.get());
    let u = user.get();
    assert!(matches!(u.display_name, std::borrow::Cow::Borrowed(_)));
    Ok(())
}

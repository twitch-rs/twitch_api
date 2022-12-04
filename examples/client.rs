use twitch_api::{helix::streams::GetStreamsRequest, types, TwitchClient};
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

#[derive(Default)]
pub struct FooClient {
    client: TwitchClient<'static, reqwest::Client>,
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenvy::dotenv();
    let mut args = std::env::args().skip(1);

    let foo_client = FooClient::default();

    let token = UserToken::from_existing(
        &foo_client.client,
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(AccessToken::new)
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
        None,
    )
    .await?;

    foo_client.client.helix.clone_client();
    let response = foo_client
        .client
        .helix
        .req_get(
            GetStreamsRequest::user_logins(
                &[types::UserNameRef::from_str(
                    &args.next().expect("please provide an username"),
                )][..],
            ),
            &token,
        )
        .await?
        .data;
    println!("{response:?}");
    Ok(())
}

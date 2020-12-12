use twitch_api2::{helix::streams::GetStreamsRequest, TwitchClient};
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
    .await?;

    let client = Foo::default();

    let req = GetStreamsRequest::builder()
        .user_login(vec![args.next().expect("please provide an username")])
        .build();
    client.client.helix.clone_client();
    let response = client.client.helix.req_get(req, &token).await?;
    println!("{:?}", response);
    Ok(())
}

#[derive(Default)]
pub struct Foo {
    client: TwitchClient<'static, reqwest::Client>,
}

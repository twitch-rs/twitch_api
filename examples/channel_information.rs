use twitch_api2::{helix, HelixClient};
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

    let req = helix::users::GetUsersRequest::builder()
        .login(vec![args.next().unwrap()])
        .build();

    let user = client
        .req_get(req, &token)
        .await?
        .data
        .into_iter()
        .next()
        .unwrap();

    let id = user.id.clone();

    let req = helix::channels::GetChannelInformationRequest::builder()
        .broadcaster_id(id)
        .build();

    let response2 = client.req_get(req, &token).await?;

    println!("User information:\n\t{:#?}", user);
    println!("Stream information:\n\t{:#?}", response2.data);
    Ok(())
}

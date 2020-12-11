use twitch_api2::HelixClient;
use twitch_oauth2::{AccessToken, TwitchToken, UserToken};

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
        twitch_oauth2::client::surf_http_client,
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(AccessToken::new)
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
        None,
    )
    .await?;

    let broadcaster_id = dbg!(
        token
            .validate_token(twitch_oauth2::client::surf_http_client)
            .await?
    )
    .user_id
    .unwrap();

    let client = HelixClient::with_client(surf::Client::new());
    for user in args {
        let user_id = match client
            .req_get(
                twitch_api2::helix::users::GetUsersRequest::builder()
                    .login(vec![user.to_string()])
                    .build(),
                &token,
            )
            .await?
            .data
            .first()
        {
            Some(user) => user.id.clone(),
            None => continue,
        };
        let req = twitch_api2::helix::users::CreateUserFollowsRequest::default();
        let body = twitch_api2::helix::users::CreateUserFollowsBody::builder()
            .to_id(user_id)
            .from_id(broadcaster_id.clone())
            .build();
        let response = client.req_post(req, body, &token).await?;
        println!("{:?}", response);
    }

    println!("scopes: {:?}", token.scopes());

    Ok(())
}

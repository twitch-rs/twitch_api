use twitch_api::{twitch_oauth2::*, *};

pub async fn get_auth_token_request(
    client: &TwitchClient<'_, reqwest::Client>,
) -> Result<AppAccessToken, ()> {
    let client_id = ClientId::from("aaa");
    let client_secret = ClientSecret::from("aaaa");

    let token =
            // here we can use the TwitchClient as a client for twitch_oauth2
        AppAccessToken::get_app_access_token(client, client_id, client_secret, Scope::all())
            .await
            .unwrap();

    Ok(token)
}

pub fn tokio() {
    let client: TwitchClient<'static, reqwest::Client> = TwitchClient::new();

    tokio::spawn({
        async move {
            get_auth_token_request(&client).await.unwrap();
        }
    });
}

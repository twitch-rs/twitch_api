use twitch_api::{helix, types, HelixClient};
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
    let client: HelixClient<reqwest::Client> = HelixClient::default();
    let token = std::env::var("TWITCH_TOKEN")
        .ok()
        .or_else(|| args.next())
        .map(AccessToken::new)
        .expect("Please set env: TWITCH_TOKEN or pass token as first argument");
    let token = UserToken::from_existing(&client, token, None, None).await?;
    let id = token.user_id.clone();

    let resp = client
        .req_get_custom(
            helix::channels::GetChannelInformationRequest::builder()
                .broadcaster_id(id)
                .build(),
            &token,
        )
        .await
        .expect("oops");

    let channel: Vec<CustomChannelInformation> = resp.data()?;

    println!("Stream information:\n{:#?}", channel);
    Ok(())
}

/// Return Values for [Get Channel Information](super::get_channel_information)
///
/// [`get-channel-information`](https://dev.twitch.tv/docs/api/reference#get-channel-information)
#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CustomChannelInformation<'a> {
    /// Twitch User ID of this channel owner
    pub broadcaster_id: &'a types::UserIdRef,
    /// Twitch User login of this channel owner
    pub broadcaster_login: &'a types::UserNameRef,
    /// Twitch user display name of this channel owner
    pub broadcaster_name: &'a types::DisplayNameRef,
    /// Current game ID being played on the channel
    pub game_id: &'a types::CategoryIdRef,
    /// Name of the game being played on the channel
    pub game_name: types::CategoryId,
    /// Language of the channel
    pub broadcaster_language: &'a str,
    /// Title of the stream
    pub title: &'a str,
    /// Description of the stream
    #[serde(default)]
    pub description: &'a str,
    /// Stream delay in seconds
    pub delay: i64,
}

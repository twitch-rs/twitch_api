//! Rust library for talking with the new twitch API aka "Helix" and TMI.
//!
//! ---

pub use helix::{clips, streams, users, HelixClient};
pub use tmi::TMIClient;

pub use twitch_oauth2;

pub mod helix;
pub mod tmi;

static TWITCH_HELIX_URL: &str = "https://api.twitch.tv/helix/";
static TWITCH_TMI_URL: &str = "https://tmi.twitch.tv/";

#[derive(Clone)]
pub struct Client {
    pub helix: HelixClient,
    pub tmi: TMIClient,
}

impl Client {
    pub async fn new(
        client_id: String,
        client_secret: String,
        scopes: Vec<twitch_oauth2::Scope>,
    ) -> Result<Client, Error>
    {
        let token =
            twitch_oauth2::AppAccessToken::get_app_access_token(client_id, client_secret, scopes)
                .await?;
        let helix = HelixClient::new(token.into());
        Ok(Client {
            tmi: TMIClient::new_with_client(helix.clone_client()),
            helix,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not make token")]
    TokenError(#[from] twitch_oauth2::TokenError),
}

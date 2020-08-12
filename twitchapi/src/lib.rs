#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/twitch_api2/0.2.3")]
//! Rust library for talking with the new Twitch API aka. "Helix" and TMI.
//!
//! [![github]](https://github.com/emilgardis/twitch_utils)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs]](https://docs.rs/twitch_api2/0.2.3/twitch_api2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__utils-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs]: https://img.shields.io/badge/docs.rs-twitch__api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>Use Twitch endpoints fearlessly</h5>
//!
//! ```rust,no_run
//! use twitch_api2::{Client, helix::channel::GetChannelRequest};
//! use twitch_oauth2::{AppAccessToken, Scope, TokenError, TwitchToken};
//!
//! # #[tokio::main]
//! # async fn run() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! # let client_id = twitch_oauth2::ClientId::new("validclientid".to_string());
//! # let client_secret = twitch_oauth2::ClientSecret::new("validclientsecret".to_string());
//! let client =
//!     match AppAccessToken::get_app_access_token(client_id, client_secret, Scope::all()).await {
//!         Ok(t) => Client::with_token(t).unwrap(),
//!         Err(TokenError::RequestError(e)) => panic!("got error: {:?}", e),
//!         Err(e) => panic!(e),
//!     };
//!
//! let req = GetChannelRequest {
//!     broadcaster_id: client.helix.token().await.as_ref().validate_token().await?.user_id,
//! };
//!
//! println!("{:?}", &client.helix.req_get(req).await?.data[0].title);
//! # Ok(())
//! # }
//! # fn main() {run().unwrap();}
//! ```
//!
//!
#[doc(no_inline)]
pub use crate::helix::HelixClient;
#[doc(no_inline)]
pub use crate::tmi::TMIClient;

#[doc(no_inline)]
pub use twitch_oauth2;

pub mod helix;
pub mod tmi;

static TWITCH_HELIX_URL: &str = "https://api.twitch.tv/helix/";
static TWITCH_TMI_URL: &str = "https://tmi.twitch.tv/";

/// Client for Twitch APIs.
#[derive(Clone)]
pub struct Client {
    /// Helix endpoint. See [helix]
    pub helix: HelixClient,
    /// TMI endpoint. See [tmi]
    pub tmi: TMIClient,
}

impl Client {
    /// Create a new [Client] with an [twitch_oauth2::AppAccessToken]
    pub async fn new(
        client_id: twitch_oauth2::ClientId,
        client_secret: twitch_oauth2::ClientSecret,
        scopes: Vec<twitch_oauth2::Scope>,
    ) -> Result<Client, Error>
    {
        let token =
            twitch_oauth2::AppAccessToken::get_app_access_token(client_id, client_secret, scopes)
                .await?;
        Client::with_token(token)
    }

    /// Create a new [Client] with a generic [twitch_oauth2::TwitchToken]
    pub fn with_token<T>(token: T) -> Result<Client, Error>
    where T: twitch_oauth2::TwitchToken + Sized + Send + Sync + 'static {
        let helix = HelixClient::new(token.into());
        Ok(Client {
            tmi: TMIClient::new_with_client(helix.clone_client()),
            helix,
        })
    }
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not make token")]
    TokenError(#[from] twitch_oauth2::TokenError),
}

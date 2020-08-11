#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/twitch_api2/0.1.1")]
//! Rust library for talking with the new Twitch API aka. "Helix" and TMI.
//!
//! [![github]](https://github.com/emilgardis/twitch_api2)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs]](https://docs.rs/twitch_api2/0.1.1/twitch_api2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__utils-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs]: https://img.shields.io/badge/docs.rs-twitch_api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>OAuth2 for Twitch endpoints</h5>
//!
#[doc(inline)]
pub use helix::{channel, clips, streams, users, HelixClient};
#[doc(inline)]
pub use tmi::TMIClient;

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

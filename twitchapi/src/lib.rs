#![allow(unknown_lints)] // remove once broken_intra_doc_links is on stable
#![deny(missing_docs, broken_intra_doc_links)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/twitch_api2/0.4.1")]
//! [![github]](https://github.com/emilgardis/twitch_utils)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs]](https://docs.rs/twitch_api2/0.4.1/twitch_api2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__utils-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs]: https://img.shields.io/badge/docs.rs-twitch__api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>Rust library for talking with the new Twitch API aka. "Helix" and TMI. Use Twitch endpoints fearlessly!</h5>
//!
//! # Examples
//!
//! ```rust,no_run
//! use twitch_api2::{TwitchClient, helix::channels::GetChannelInformationRequest};
//! use twitch_oauth2::{AppAccessToken, Scope, TokenError, TwitchToken};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! # let surf_http_client = twitch_oauth2::dummy_http_client; // This is only here to fool doc tests
//! # let client_id = twitch_oauth2::ClientId::new("validclientid".to_string());
//! # let client_secret = twitch_oauth2::ClientSecret::new("validclientsecret".to_string());
//! let token =
//!     match AppAccessToken::get_app_access_token(surf_http_client, client_id, client_secret, Scope::all()).await {
//!         Ok(t) => t,
//!         Err(TokenError::RequestError(e)) => panic!("got error: {:?}", e),
//!         Err(e) => panic!(e),
//!     };
//! let client = TwitchClient::new();
//! # let _: &TwitchClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetChannelInformationRequest::builder()
//!     .broadcaster_id("27620241")
//!     .build();
//!
//! println!("{:?}", &client.helix.req_get(req, &token).await?.data[0].title);
//! # Ok(())
//! # }
//! ```
//!

#[cfg(feature = "helix")]
pub mod helix;
#[cfg(feature = "tmi")]
pub mod tmi;

#[cfg(feature = "helix")]
#[doc(inline)]
pub use crate::helix::HelixClient;
#[cfg(feature = "tmi")]
#[doc(inline)]
pub use crate::tmi::TMIClient;

#[cfg(feature = "helix")]
#[doc(no_inline)]
pub use twitch_oauth2;

pub mod client;
pub use client::Client;

#[doc(hidden)]
pub use client::DummyHttpClient;

#[cfg(feature = "helix")]
static TWITCH_HELIX_URL: &str = "https://api.twitch.tv/helix/";
#[cfg(feature = "tmi")]
static TWITCH_TMI_URL: &str = "https://tmi.twitch.tv/";

/// Client for Twitch APIs.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TwitchClient<'a, C>
where C: Client<'a> {
    /// Helix endpoint. See [helix]
    #[cfg(feature = "helix")]
    pub helix: HelixClient<'a, C>,
    /// TMI endpoint. See [tmi]
    #[cfg(feature = "tmi")]
    pub tmi: TMIClient<'a, C>,
}

impl<'a, C: Client<'a>> TwitchClient<'a, C> {
    /// Create a new [Client]
    #[cfg(all(feature = "helix", feature = "tmi"))]
    pub fn new() -> TwitchClient<'a, C>
    where C: Clone + Default {
        let helix = HelixClient::new();
        TwitchClient {
            tmi: TMIClient::with_client(helix.clone_client()),
            helix,
        }
    }
}

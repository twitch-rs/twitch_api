#![allow(unknown_lints, renamed_and_removed_lints)]
#![deny(missing_docs, broken_intra_doc_links)] // This will be weird until 1.52, see https://github.com/rust-lang/rust/pull/80527
#![cfg_attr(nightly, deny(rustdoc::broken_intra_doc_links))]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/twitch_api2/0.5.0")]
#![cfg_attr(all(nightly, doctest), feature(external_doc))]
//! [![github]](https://github.com/emilgardis/twitch_api2)&ensp;[![crates-io]](https://crates.io/crates/twitch_api2)&ensp;[![docs-rs-big]](https://docs.rs/twitch_api2/0.5.0/twitch_api2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__api2-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_api2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs-big]: https://img.shields.io/badge/docs.rs-twitch__api2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>Rust library for talking with the new Twitch API aka. "Helix", TMI and more! Use Twitch endpoints fearlessly!</h5>
//!
//! # Examples
//!
//! Get information about a channel with the [`Get Channel Information`](crate::helix::channels::get_channel_information) helix endpoint.
//!
//! ```rust,no_run
//! use twitch_api2::{TwitchClient, helix::channels::GetChannelInformationRequest};
//! use twitch_oauth2::client::surf_http_client;
//! use twitch_oauth2::{AppAccessToken, Scope, TwitchToken, tokens::errors::TokenError};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let surf_http_client = twitch_oauth2::dummy_http_client; // This is only here to fool doc tests
//! # let client_id = twitch_oauth2::ClientId::new("validclientid".to_string());
//! # let client_secret = twitch_oauth2::ClientSecret::new("validclientsecret".to_string());
//! let token =
//!     match AppAccessToken::get_app_access_token(surf_http_client, client_id, client_secret, Scope::all()).await {
//!         Ok(t) => t,
//!         Err(TokenError::Request(e)) => panic!("got error: {:?}", e),
//!         Err(e) => panic!(e),
//!     };
//! let client = TwitchClient::new();
//! # let _: &TwitchClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetChannelInformationRequest::builder()
//!     .broadcaster_id("27620241")
//!     .build();
//!
//! println!("{:?}", &client.helix.req_get(req, &token).await?.data.unwrap().title);
//! # Ok(())
//! # }
//! ```
//!
//! There is also a convenience function for accesssing channel information with a specified login name
//!
//! ```rust,no_run
//! # use twitch_api2::{TwitchClient, helix::channels::GetChannelInformationRequest};
//! # use twitch_oauth2::{AppAccessToken, Scope, TwitchToken, tokens::errors::TokenError};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let surf_http_client = twitch_oauth2::dummy_http_client; // This is only here to fool doc tests
//! # let client_id = twitch_oauth2::ClientId::new("validclientid".to_string());
//! # let client_secret = twitch_oauth2::ClientSecret::new("validclientsecret".to_string());
//! # let token =
//! #   match AppAccessToken::get_app_access_token(surf_http_client, client_id, client_secret, Scope::all()).await {
//! #       Ok(t) => t,
//! #       Err(TokenError::Request(e)) => panic!("got error: {:?}", e),
//! #       Err(e) => panic!(e),
//! #   };
//! # let client = TwitchClient::new();
//! # let _: &TwitchClient<twitch_api2::DummyHttpClient> = &client;
//!
//! println!("{:?}", &client.helix.get_channel_from_login("twitch".to_string(), &token).await?.unwrap().title);
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! This crate provides almost no functionality by default, only exposing [`types`]. To enable more features, refer to below table.
//!
//! | Feature |         |
//! | -------: | :------- |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>twitch_oauth2</code></span> | Gives [scopes](twitch_oauth2::Scope) for endpoints and topics that are needed to call them. |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>client</code></span> | Gives a [client abstraction](HttpClient) for endpoints. See for example [`TmiClient`] and [`HelixClient`] |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>helix</code></span> | Enables [Helix](helix) endpoints |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>tmi</code></span> | Enables [TMI](tmi) endpoints |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>eventsub</code></span> | Enables deserializable structs for [EventSub](eventsub) |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>pubsub</code></span> | Enables deserializable structs for [PubSub](pubsub) |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>surf_client</code></span> | Enables surf for [`HttpClient`]. Note that this does not enable any default client backend, if you get a compile error, specify `surf` in your `Cargo.toml`. By default, `surf` uses feature `curl-client` |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>reqwest_client</code></span> | Enables reqwest for [`HttpClient`]. Note that this does not enable any default TLS backend, if you get `invalid URL, scheme is not http`, specify `reqwest` in your Cargo.toml. By default, `reqwest` uses feature `default-tls` |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>hmac</code></span> | Enable [message authentication](eventsub::Payload::verify_payload) using HMAC on [EventSub](eventsub) |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>all</code></span> | Enables all above features. Including reqwest and surf. Do not use this in production, it's better if you specify exactly what you need |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>unsupported</code></span> | Enables undocumented or experimental endpoints or topics. Breakage may occur |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>deny_unknown_fields</code></span> | Adds `#[serde(deny_unknown_fields)]` on all applicable structs/enums. Please consider using this and filing an issue or PR when a new field has been added to the endpoint but not added to this  library. |
//!

#[doc(include = "../README.md")]
#[cfg(all(doctest, feature = "all"))]
pub struct ReadmeDoctests;

pub mod types;

#[cfg(feature = "helix")]
#[cfg_attr(nightly, doc(cfg(feature = "helix")))]
pub mod helix;
#[cfg(feature = "tmi")]
#[cfg_attr(nightly, doc(cfg(feature = "tmi")))]
pub mod tmi;

#[cfg(feature = "pubsub")]
#[cfg_attr(nightly, doc(cfg(feature = "pubsub")))]
pub mod pubsub;

#[cfg(feature = "eventsub")]
#[cfg_attr(nightly, doc(cfg(feature = "eventsub")))]
pub mod eventsub;

#[cfg(all(feature = "helix", feature = "client"))]
#[doc(inline)]
pub use crate::helix::HelixClient;
#[cfg(all(feature = "tmi", feature = "client"))]
#[doc(inline)]
pub use crate::tmi::TmiClient;

#[cfg(any(feature = "twitch_oauth2", all(feature = "helix", feature = "client")))]
#[doc(no_inline)]
pub use twitch_oauth2;

#[cfg(feature = "client")]
#[cfg_attr(nightly, doc(cfg(feature = "client")))]
pub mod client;
#[cfg(feature = "client")]
#[cfg_attr(nightly, doc(cfg(feature = "client")))]
pub use client::Client as HttpClient;

#[doc(hidden)]
#[cfg(feature = "client")]
pub use client::DummyHttpClient;

/// Location of Twitch Helix
#[cfg(feature = "helix")]
#[cfg_attr(nightly, doc(cfg(feature = "helix")))]
pub static TWITCH_HELIX_URL: &str = "https://api.twitch.tv/helix/";
/// Location of Twitch TMI
#[cfg(feature = "tmi")]
#[cfg_attr(nightly, doc(cfg(feature = "tmi")))]
pub static TWITCH_TMI_URL: &str = "https://tmi.twitch.tv/";
/// Location to twitch PubSub
#[cfg(feature = "pubsub")]
#[cfg_attr(nightly, doc(cfg(feature = "pubsub")))]
pub static TWITCH_PUBSUB_URL: &str = "wss://pubsub-edge.twitch.tv";

/// Client for Twitch APIs.
///
/// Most [http clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api2::{TwitchClient}; pub mod reqwest {pub type Client = twitch_api2::client::DummyHttpClient;}
/// pub struct MyStruct {
///     twitch: TwitchClient<'static, reqwest::Client>,
///     token: twitch_oauth2::AppAccessToken,
/// }
/// // etc
/// ```
///
/// See [`client`] for implemented clients, you can also define your own if needed.
#[cfg(all(feature = "client", any(feature = "helix", feature = "tmi")))]
#[cfg_attr(
    nightly,
    doc(cfg(all(feature = "client", any(feature = "helix", feature = "tmi"))))
)]
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TwitchClient<'a, C>
where C: HttpClient<'a> {
    /// Helix endpoint. See [`helix`]
    #[cfg(feature = "helix")]
    pub helix: HelixClient<'a, C>,
    /// TMI endpoint. See [`tmi`]
    #[cfg(feature = "tmi")]
    pub tmi: TmiClient<'a, C>,
}

#[cfg(all(feature = "client", any(feature = "helix", feature = "tmi")))]
impl<C: HttpClient<'static>> TwitchClient<'static, C> {
    /// Create a new [`TwitchClient`]
    #[cfg(all(feature = "helix", feature = "tmi"))]
    pub fn new() -> TwitchClient<'static, C>
    where C: Clone + Default {
        let helix = HelixClient::new();
        TwitchClient {
            tmi: TmiClient::with_client(helix.clone_client()),
            helix,
        }
    }
}

#[cfg(all(feature = "client", any(feature = "helix", feature = "tmi")))]
impl<'a, C: HttpClient<'a>> TwitchClient<'a, C> {
    /// Create a new [`TwitchClient`] with an existing [`HttpClient`]
    #[cfg_attr(
        nightly,
        doc(cfg(all(feature = "client", any(feature = "helix", feature = "tmi"))))
    )]
    #[cfg(any(feature = "helix", feature = "tmi"))]
    pub fn with_client(client: C) -> TwitchClient<'a, C>
    where C: Clone {
        let helix = HelixClient::with_client(client);
        TwitchClient {
            #[cfg(feature = "tmi")]
            tmi: TmiClient::with_client(helix.clone_client()),
            #[cfg(feature = "helix")]
            helix,
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[track_caller]
    pub fn roundtrip<T: serde::de::DeserializeOwned + serde::Serialize>(val: &T) {
        serde_json::from_slice::<T>(&serde_json::to_vec(val).expect("could not make into json"))
            .expect("could not convert back from json");
        serde_cbor::from_slice::<T>(
            &serde_cbor::to_vec(val).expect("could not make into cbor bytes"),
        )
        .expect("could not convert back from cbor");
    }
}

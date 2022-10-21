#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![doc(html_root_url = "https://docs.rs/twitch_api/0.7.0-rc.2")]
//! [![github]](https://github.com/twitch-rs/twitch_api)&ensp;[![crates-io]](https://crates.io/crates/twitch_api)&ensp;[![docs-rs-big]](https://docs.rs/twitch_api/0.7.0-rc.2/twitch_api)
//!
//! [github]: https://img.shields.io/badge/github-twitch--rs/twitch__api-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_api.svg?style=for-the-badge&color=fc8d62&logo=rust"
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
//! use twitch_api::{helix::channels::GetChannelInformationRequest, TwitchClient};
//! use twitch_oauth2::{tokens::errors::AppAccessTokenError, AppAccessToken, Scope, TwitchToken};
//! # pub mod reqwest {pub type Client = twitch_api::client::DummyHttpClient;}
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client_id = twitch_oauth2::ClientId::from("validclientid");
//! # let client_secret = twitch_oauth2::ClientSecret::from("validclientsecret");
//!
//! let client: TwitchClient<reqwest::Client> = TwitchClient::default();
//! let token =
//!     AppAccessToken::get_app_access_token(&client, client_id, client_secret, Scope::all())
//!         .await?;
//! let req = GetChannelInformationRequest::broadcaster_id("27620241");
//! println!(
//!     "{:?}",
//!     &client.helix.req_get(req, &token).await?.data.unwrap().title
//! );
//! # Ok(())
//! # }
//! ```
//!
//! There is also a convenience function for accessing channel information with a specified login name
//!
//! ```rust,no_run
//! # use twitch_api::{TwitchClient, helix::channels::GetChannelInformationRequest};
//! # use twitch_oauth2::{AppAccessToken, Scope, TwitchToken, tokens::errors::AppAccessTokenError};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = TwitchClient::default();
//! # let _:&TwitchClient<twitch_api::DummyHttpClient> = &client;
//! # let client_id = twitch_oauth2::ClientId::new("validclientid".to_string());
//! # let client_secret = twitch_oauth2::ClientSecret::new("validclientsecret".to_string());
//! # let token =
//! #   match AppAccessToken::get_app_access_token(&client, client_id, client_secret, Scope::all()).await {
//! #       Ok(t) => t,
//! #       Err(AppAccessTokenError::Request(e)) => panic!("got error: {:?}", e),
//! #       Err(e) => panic!(e),
//! #   };
//!
//! println!("{:?}", &client.helix.get_channel_from_login("twitch", &token).await?.unwrap().title);
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
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>hmac</code></span> | Enable [message authentication](eventsub::Event::verify_payload) using HMAC on [EventSub](eventsub) |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>time</code></span> | Enable time utilities on [Timestamp](types::Timestamp) |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>all</code></span> | Enables all above features. Including reqwest and surf. Do not use this in production, it's better if you specify exactly what you need |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>ureq</code></span> | Enables ureq for [`HttpClient`]. |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>surf</code></span> | Enables surf for [`HttpClient`]. Note that this does not enable any default client backend, if you get a compile error, specify `surf` in your `Cargo.toml`. By default, `surf` uses feature `curl-client` |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>reqwest</code></span> | Enables reqwest for [`HttpClient`]. Note that this does not enable any default TLS backend, if you get `invalid URL, scheme is not http`, specify `reqwest` in your Cargo.toml. By default, `reqwest` uses feature `default-tls` |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>unsupported</code></span> | Enables undocumented or experimental endpoints, topics or features. Breakage may occur, semver compatibility not guaranteed. |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>trace_unknown_fields</code></span> | Logs ignored fields as `WARN` log messages where  applicable. Please consider using this and filing an issue or PR when a new field has been added to the endpoint but not added to this library. |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>deny_unknown_fields</code></span> | Adds `#[serde(deny_unknown_fields)]` on all applicable structs/enums. Please consider using this and filing an issue or PR when a new field has been added to the endpoint but not added to this library. |
//! | <span class="module-item stab portability" style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"><code>deser_borrow</code></span> | Makes fields on [`Deserialize`](serde::Deserialize)-able structs borrow if they can be borrowed, this feature is enabled by default, but exists to enable using [`serde::de::DeserializeOwned`] or [`for<'de> serde::Deserialize<'de>`](serde::Deserialize) by disabling this feature. |

/// Doc test for README
#[doc = include_str!("../README.md")]
#[doc(hidden)]
pub struct ReadmeDoctests;

pub use twitch_types as types;

#[cfg(feature = "helix")]
pub mod helix;
#[cfg(feature = "tmi")]
pub mod tmi;

#[cfg(feature = "pubsub")]
pub mod pubsub;

#[cfg(feature = "eventsub")]
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
pub mod client;
#[cfg(feature = "client")]
pub use client::Client as HttpClient;

#[doc(hidden)]
#[cfg(feature = "client")]
pub use client::DummyHttpClient;

#[cfg(any(feature = "helix", feature = "tmi", feature = "pubsub"))]
/// Generate a url with a default if `mock_api` feature is disabled, or env var is not defined or is invalid utf8
macro_rules! mock_env_url {
    ($var:literal, $default:expr $(,)?) => {
        once_cell::sync::Lazy::new(move || {
            #[cfg(feature = "mock_api")]
            if let Ok(url) = std::env::var($var) {
                return url::Url::parse(&url).expect(concat!(
                    "URL could not be made from `env:",
                    $var,
                    "`."
                ));
            };
            url::Url::parse(&$default).unwrap()
        })
    };
}

/// Location of Twitch Helix
///
/// Can be overridden when feature `mock_api` is enabled with environment variable `TWITCH_HELIX_URL`.
///
/// # Examples
///
/// Set the environment variable `TWITCH_HELIX_URL` to `http://localhost:8080/mock/` to use [`twitch-cli` mock](https://github.com/twitchdev/twitch-cli/blob/main/docs/mock-api.md) endpoints.
#[cfg(feature = "helix")]
pub static TWITCH_HELIX_URL: once_cell::sync::Lazy<url::Url> =
    mock_env_url!("TWITCH_HELIX_URL", "https://api.twitch.tv/helix/");
/// Location of Twitch TMI
///
/// Can be overriden when feature `mock_api` is enabled with environment variable `TWITCH_TMI_URL`.
#[cfg(feature = "tmi")]
pub static TWITCH_TMI_URL: once_cell::sync::Lazy<url::Url> =
    mock_env_url!("TWITCH_TMI_URL", "https://tmi.twitch.tv/");
/// Location to twitch PubSub
///
/// Can be overriden when feature `mock_api` is enabled with environment variable `TWITCH_PUBSUB_URL`.
#[cfg(feature = "pubsub")]
pub static TWITCH_PUBSUB_URL: once_cell::sync::Lazy<url::Url> =
    mock_env_url!("TWITCH_PUBSUB_URL", "wss://pubsub-edge.twitch.tv");

/// Client for Twitch APIs.
///
/// Most [http clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api::{TwitchClient}; pub mod reqwest {pub type Client = twitch_api::client::DummyHttpClient;}
/// pub struct MyStruct {
///     twitch: TwitchClient<'static, reqwest::Client>,
///     token: twitch_oauth2::AppAccessToken,
/// }
/// // etc
/// ```
///
/// See [`client`] for implemented clients, you can also define your own if needed.
#[cfg(all(feature = "client", any(feature = "helix", feature = "tmi")))]
#[derive(Clone)]
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
    #[cfg(any(feature = "helix", feature = "tmi"))]
    pub fn new() -> TwitchClient<'static, C>
    where C: Clone + client::ClientDefault<'static> {
        let client = C::default_client();
        Self::with_client(client)
    }
}

#[cfg(all(feature = "client", any(feature = "helix", feature = "tmi")))]
impl<C: HttpClient<'static> + client::ClientDefault<'static>> Default for TwitchClient<'static, C> {
    fn default() -> Self { Self::new() }
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
        TwitchClient {
            #[cfg(all(feature = "tmi", not(feature = "helix")))]
            tmi: TmiClient::with_client(client),
            #[cfg(all(feature = "tmi", feature = "helix"))]
            tmi: TmiClient::with_client(client.clone()),
            #[cfg(feature = "helix")]
            helix: HelixClient::with_client(client),
        }
    }

    /// Retrieve a reference of the [`HttpClient`][crate::HttpClient] inside this [`TwitchClient`]
    pub fn get_client(&self) -> &C {
        #[cfg(feature = "helix")]
        {
            self.helix.get_client()
        }
        #[cfg(not(feature = "helix"))]
        {
            #[cfg(feature = "tmi")]
            {
                self.tmi.get_client()
            }
        }
    }
}

/// A deserialization error
#[cfg(feature = "serde_json")]
#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum DeserError {
    /// could not deserialize, error on [{path}]. {error}
    PathError {
        /// Path to where the erroring key/value is
        path: String,
        /// Error for the key/value
        #[source]
        error: serde_json::Error,
    },
}

/// Parse a string as `T`, logging ignored fields and giving a more detailed error message on parse errors
///
/// The log_ignored argument decides if a trace of ignored value should be emitted
#[cfg(feature = "serde_json")]
pub fn parse_json<'a, T: serde::Deserialize<'a>>(
    s: &'a str,
    #[allow(unused_variables)] log_ignored: bool,
) -> Result<T, DeserError> {
    #[cfg(feature = "trace_unknown_fields")]
    {
        let jd = &mut serde_json::Deserializer::from_str(s);
        let mut track = serde_path_to_error::Track::new();
        let pathd = serde_path_to_error::Deserializer::new(jd, &mut track);
        if log_ignored {
            let mut fun = |path: serde_ignored::Path| {
                tracing::warn!(key=%path,"Found ignored key");
            };
            serde_ignored::deserialize(pathd, &mut fun).map_err(|e| DeserError::PathError {
                path: track.path().to_string(),
                error: e,
            })
        } else {
            T::deserialize(pathd).map_err(|e| DeserError::PathError {
                path: track.path().to_string(),
                error: e,
            })
        }
    }
    #[cfg(not(feature = "trace_unknown_fields"))]
    {
        let jd = &mut serde_json::Deserializer::from_str(s);
        serde_path_to_error::deserialize(jd).map_err(|e| DeserError::PathError {
            path: e.path().to_string(),
            error: e.into_inner(),
        })
    }
}

/// Parse a json Value as `T`, logging ignored fields and giving a more detailed error message on parse errors
#[cfg(feature = "serde_json")]
pub fn parse_json_value<'a, T: serde::Deserialize<'a>>(
    value: serde_json::Value,
    #[allow(unused_variables)] log_ignored: bool,
) -> Result<T, DeserError> {
    #[cfg(feature = "trace_unknown_fields")]
    {
        let de = serde::de::IntoDeserializer::into_deserializer(value);
        let mut track = serde_path_to_error::Track::new();
        let pathd = serde_path_to_error::Deserializer::new(de, &mut track);
        if log_ignored {
            let mut fun = |path: serde_ignored::Path| {
                tracing::warn!(key=%path,"Found ignored key");
            };
            serde_ignored::deserialize(pathd, &mut fun).map_err(|e| DeserError::PathError {
                path: track.path().to_string(),
                error: e,
            })
        } else {
            T::deserialize(pathd).map_err(|e| DeserError::PathError {
                path: track.path().to_string(),
                error: e,
            })
        }
    }
    #[cfg(not(feature = "trace_unknown_fields"))]
    {
        let de = serde::de::IntoDeserializer::into_deserializer(value);
        serde_path_to_error::deserialize(de).map_err(|e| DeserError::PathError {
            path: e.path().to_string(),
            error: e.into_inner(),
        })
    }
}

#[cfg(any(feature = "helix", feature = "pubsub", feature = "eventsub"))]
#[allow(dead_code)]
/// Deserialize 'null' as <T as Default>::Default
fn deserialize_default_from_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default, {
    use serde::Deserialize;
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
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

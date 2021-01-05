#![doc(alias = "TMI")]
//! TMI Endpoint, twitch's unsupported api for better chatters retrieval
use crate::types;
use serde::{Deserialize, Serialize};
/// Client for the twitch TMI endpoint, almost entirely undocumented and certainly not supported.
///
/// # Examples
///
/// ```rust,no_run
/// # use twitch_api2::tmi::TMIClient; use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = TMIClient::new();
/// # let _: &TMIClient<twitch_api2::DummyHttpClient> = &client;
/// println!("{:?}", client.get_chatters("justinfan10").await?);
/// # Ok(())
/// # }
/// ```
///
/// Most [clients][crate::HttpClient] will be able to use the `'static` lifetime
///
/// ```rust,no_run
/// # use twitch_api2::{TMIClient}; pub mod surf {pub type Client = twitch_api2::client::DummyHttpClient;}
/// pub struct MyStruct {
///     twitch: TMIClient<'static, surf::Client>,
///     token: twitch_oauth2::AppAccessToken,
/// }
/// // etc
/// ```
///
/// See [`HttpClient`][crate::HttpClient] for implemented http clients, you can also define your own if needed.
#[cfg(all(feature = "client", feature = "tmi"))]
#[cfg_attr(nightly, doc(cfg(all(feature = "client", feature = "tmi"))))] // FIXME: This doc_cfg does nothing
#[derive(Default, Clone)]
pub struct TMIClient<'a, C: crate::HttpClient<'a>> {
    client: C,
    _pd: std::marker::PhantomData<&'a ()>,
}

#[cfg(all(feature = "tmi", feature = "client"))]
impl<'a, C: crate::HttpClient<'a>> TMIClient<'a, C> {
    /// Create a new client with a default
    pub fn new() -> TMIClient<'a, C>
    where C: Default {
        TMIClient::with_client(C::default())
    }

    /// Create a new [`TMIClient`] with an existing [`HttpClient`][crate::HttpClient]
    pub fn with_client(client: C) -> TMIClient<'a, C> {
        TMIClient {
            client,
            _pd: std::marker::PhantomData::default(),
        }
    }

    /// Retrieve a clone of the [`HttpClient`][crate::HttpClient] inside this [`TMIClient`]
    pub fn clone_client(&self) -> C
    where C: Clone {
        self.client.clone()
    }

    /// Get all the chatters in the chat
    ///
    /// # Notes
    ///
    /// This function will aside from url sanitize the broadcasters username, will also remove any `#` and make it lowercase ascii
    pub async fn get_chatters(
        &'a self,
        broadcaster: &str,
    ) -> Result<GetChatters, RequestError<<C as crate::HttpClient<'a>>::Error>> {
        let url = format!(
            "{}{}{}{}",
            crate::TWITCH_TMI_URL,
            "group/user/",
            broadcaster.replace('#', "").to_ascii_lowercase(),
            "/chatters"
        );
        let req = http::Request::builder()
            .uri(url)
            .body(Vec::with_capacity(0))?;
        let req = self
            .client
            .req(req)
            .await
            .map_err(|e| RequestError::RequestError(Box::new(e)))?;
        let text = std::str::from_utf8(&req.body())
            .map_err(|e| RequestError::Utf8Error(req.body().clone(), e))?;
        serde_json::from_str(text).map_err(Into::into)
    }

    /// Get the broadcaster that a given channel is hosting, or
    /// the list of channels hosting a given target broadcaster.
    ///
    /// # Notes
    /// This endpoint requires `host={id}` XOR `target={id}` in the query
    /// (providing both will result in an error, therefore this function takes
    /// a [`HostsRequestId`] enum).
    pub async fn get_hosts(
        &'a self,
        include_logins: bool,
        channel_id: HostsRequestId,
    ) -> Result<GetHosts, RequestError<<C as crate::HttpClient<'a>>::Error>> {
        let url = format!(
            "{}{}{}{}",
            crate::TWITCH_TMI_URL,
            "hosts?",
            if include_logins {
                "include_logins=1&"
            } else {
                ""
            },
            match channel_id {
                HostsRequestId::Host(id) => format!("host={}", id),
                HostsRequestId::Target(id) => format!("target={}", id),
            }
        );
        let req = http::Request::builder()
            .uri(url)
            .body(Vec::with_capacity(0))?;
        let req = self
            .client
            .req(req)
            .await
            .map_err(|e| RequestError::RequestError(Box::new(e)))?;
        let text = std::str::from_utf8(&req.body())
            .map_err(|e| RequestError::Utf8Error(req.body().clone(), e))?;
        serde_json::from_str(text).map_err(Into::into)
    }
}

/// Returned by TMI at `https://tmi.twitch.tv/group/user/{broadcaster}/chatters`
///
/// See [`TMIClient::get_chatters`]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChatters {
    /// Amount of connected users
    pub chatter_count: u64,
    /// Lists of users in their "rank"
    pub chatters: Chatters,
}

/// List of "rank"s and what users are in them. A user can only be in one
#[derive(Debug, Serialize, Deserialize)]
pub struct Chatters {
    /// Broadcaster, can (probably) only be one
    pub broadcaster: Vec<types::Nickname>,
    /// VIPS in the chat, have the VIP badge and are set with `/vip username`
    pub vips: Vec<types::Nickname>,
    /// Moderators in the chat, have a moderator badge and are set with `/mod username`
    pub moderators: Vec<types::Nickname>,
    /// Twitch Staff in the chat, have a staff badge.
    pub staff: Vec<types::Nickname>,
    /// Twitch Admins in the chat, have an admin badge, akin to [Chatters::global_mods].
    pub admins: Vec<types::Nickname>,
    /// Twitch Global Moderators in the chat, have an admin badge, akin to [Chatters::global_mods].
    pub global_mods: Vec<types::Nickname>,
    /// Regular viewer in the chat, includes followers and subscribers.
    pub viewers: Vec<types::Nickname>,
}

/// Possible options for a [`TMIClient::get_hosts`] request.
#[derive(Debug)]
pub enum HostsRequestId {
    /// Request the broadcaster that a given channel is hosting.
    Host(UserId),
    /// Request a list of channels hosting a target broadcaster.
    Target(UserId),
}

/// Returned by TMI at `https://tmi.twitch.tv/hosts`
///
/// See [`TMIClient::get_hosts`]
#[derive(Debug, Serialize, Deserialize)]
pub struct GetHosts {
    /// List of host records. `len()` will be 1 if successfully requested for a
    /// [HostsRequestId::Host], in which case `target_id` may be missing if the
    /// channel is not hosting anyone.
    pub hosts: Vec<Host>,
}

/// A host record returned by TMI at `https://tmi.twitch.tv/hosts`
///
/// See [`TMIClient::get_hosts`]
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    /// User ID of the hosting channel
    pub host_id: UserId,
    /// User ID of the hosted channel. Will be missing if the given channel is not hosting anyone.
    pub target_id: Option<UserId>,
    /// Login of the hosting channel, if requested with `include_logins = true`
    pub host_login: Option<types::Nickname>,
    /// Login of the hosted channel, if requested with `include_logins = true`
    pub target_login: Option<types::Nickname>,
    /// Display name of the hosting channel, if requested with `include_logins = true`
    pub host_display_name: Option<types::Nickname>,
    /// Display name of the hosted channel, if requested with `include_logins = true`
    pub target_display_name: Option<types::Nickname>,
}

/// User ID
pub type UserId = u64; // TMI user ID's appear to still be ints, even though Helix uses strings.

/// Errors for [`TMIClient`] requests
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum RequestError<RE: std::error::Error + Send + Sync + 'static> {
    /// http crate returned an error
    HttpError(#[from] http::Error),
    /// deserialization failed
    DeserializeError(#[from] serde_json::Error),
    /// request failed
    RequestError(#[from] Box<RE>),
    /// could not parse body as utf8: {1}
    Utf8Error(Vec<u8>, std::str::Utf8Error),
}

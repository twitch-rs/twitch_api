//! TMI Endpoint, twitch's unsupported api for better chatters retrieval
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
/// println!("{:?}", client.get_chatters("justinfan10").await?);
/// # Ok(())
/// # }
/// ```
#[derive(Default, Clone)]
pub struct TMIClient {
    client: reqwest::Client,
}

impl TMIClient {
    /// Create a new client with a default [reqwest::Client]
    pub fn new() -> TMIClient {
        let client = reqwest::Client::new();
        TMIClient::with_client(client)
    }

    /// Create a new client with an existing [reqwest::Client]
    pub fn with_client(client: reqwest::Client) -> TMIClient { TMIClient { client } }

    /// Get all the chatters in the chat
    /// 
    /// # Notes
    pub async fn get_chatters(&self, broadcaster: &str) -> Result<GetChatters, RequestError> {
        let url = format!(
            "{}{}{}{}",
            crate::TWITCH_TMI_URL,
            "group/user/",
            broadcaster.replace('#', "").to_ascii_lowercase(),
            "/chatters"
        );
        let req = self.client.get(&url).send().await?;
        let text = req.text().await;
        serde_json::from_str(&text?).map_err(Into::into)
    }
}

/// Returned by TMI at `https://tmi.twitch.tv/group/user/{broadcaster}/chatters`
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
    pub broadcaster: Vec<Nickname>,
    /// VIPS in the chat, have the VIP badge and are set with `/vip username`
    pub vips: Vec<Nickname>,
    /// Moderators in the chat, have a moderator badge and are set with `/mod username`
    pub moderators: Vec<Nickname>,
    /// Twitch Staff in the chat, have a staff badge.
    pub staff: Vec<Nickname>,
    /// Twitch Admins in the chat, have an admin badge, akin to [Chatters::global_mods].
    pub admins: Vec<Nickname>,
    /// Twitch Global Moderators in the chat, have an admin badge, akin to [Chatters::global_mods].
    pub global_mods: Vec<Nickname>,
    /// Regular viewer in the chat, includes followers and subscribers.
    pub viewers: Vec<Nickname>,
}

/// Nickname
pub type Nickname = String;

/// Errors for [TMIClient] requests
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum RequestError {
    /// deserialization failed
    DeserializeError(#[from] serde_json::Error),
    /// request failed
    RequestError(#[from] reqwest::Error),
}

use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub struct TMIClient {
    client: reqwest::Client,
}

impl TMIClient {
    pub fn new() -> TMIClient {
        let client = reqwest::Client::new();
        TMIClient::new_with_client(client)
    }

    pub fn new_with_client(client: reqwest::Client) -> TMIClient { TMIClient { client } }

    pub async fn get_chatters(&self, broadcaster: &str) -> Result<GetChatters, RequestError> {
        let url = format!(
            "{}{}{}{}",
            crate::TWITCH_TMI_URL,
            "group/user/",
            broadcaster.replace('#', ""),
            "/chatters"
        );
        let req = self.client.get(&url).send().await?;
        let text = req.text().await;
        serde_json::from_str(&text?).map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChatters {
    pub chatter_count: u64,
    pub chatters: Chatters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chatters {
    pub broadcaster: Vec<Nickname>,
    pub vips: Vec<Nickname>,
    pub moderators: Vec<Nickname>,
    pub staff: Vec<Nickname>,
    pub admins: Vec<Nickname>,
    pub global_mods: Vec<Nickname>,
    pub viewers: Vec<Nickname>,
}

pub type Nickname = String;

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("deserialization failed")]
    DeserializeError(#[from] serde_json::Error),
    #[error("request failed")]
    RequestError(#[from] reqwest::Error),
    #[error("something happened")]
    Other,
}

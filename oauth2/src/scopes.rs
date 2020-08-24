//! Module for all possible scopes in twitch.

use serde::{Deserialize, Serialize};

/// Scopes for twitch.
///
/// <https://dev.twitch.tv/docs/authentication/#scopes>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Scope {
    /// View analytics data for your extensions.
    #[serde(rename = "analytics:read:extensions")]
    AnalyticsReadExtensions,
    /// Manage a user object.
    #[serde(rename = "user:edit")]
    UserEdit,
    /// Read authorized user's email address.
    #[serde(rename = "user:read:email")]
    UserReadEmail,
    /// Read authorized user’s stream key.
    ///
    /// # Note:
    /// This scope seems to not work, even though it is documented.
    #[serde(rename = "user:read:stream_key")]
    UserReadStreamKey,
    /// Create and edit clips as a specific user.
    #[serde(rename = "clips:edit")]
    ClipsEdit,
    /// View bits information for your channel.
    #[serde(rename = "bits:read")]
    BitsRead,
    /// View analytics data for your games.
    #[serde(rename = "analytics:read:games")]
    AnalyticsReadGames,
    /// Edit your channel's broadcast configuration, including extension configuration. (This scope implies user:read:broadcast capability.)
    #[serde(rename = "user:edit:broadcast")]
    UserEditBroadcast,
    /// View your broadcasting configuration, including extension configurations.
    #[serde(rename = "user:read:broadcast")]
    UserReadBroadcast,
    /// View live Stream Chat and Rooms messages
    #[serde(rename = "chat:read")]
    ChatRead,
    /// Send live Stream Chat and Rooms messages
    #[serde(rename = "chat:edit")]
    ChatEdit,
    /// Perform moderation actions in a channel
    #[serde(rename = "channel:moderate")]
    ChannelModerate,
    /// Get a list of all subscribers to your channel and check if a user is subscribed to your channel
    #[serde(rename = "channel:read:subscriptions")]
    ChannelReadSubscriptions,
    // FIXME: Documentation.
    ///
    #[serde(rename = "channel:read:hype_train")]
    ChannelReadHypeTrain,
    /// View your whisper messages.
    #[serde(rename = "whispers:read")]
    WhispersRead,
    /// Send whisper messages.
    #[serde(rename = "whispers:edit")]
    WhispersEdit,
    /// View your channel's moderation data including Moderators, Bans, Timeouts and Automod settings
    #[serde(rename = "moderation:read")]
    ModerationRead,
    /// View your channel points custom reward redemptions
    #[serde(rename = "channel:read:redemptions")]
    ChannelReadRedemptions,
    /// Start a commercial on authorized channels
    #[serde(rename = "channel:edit:commercial")]
    ChannelEditCommercial,
    /// Other scope that is not implemented.
    Other(String),
}

impl Scope {
    /// Get [Scope] as [oauth2::Scope]
    pub fn as_oauth_scope(&self) -> oauth2::Scope {
        use self::Scope::*;
        let s = match self {
            AnalyticsReadExtensions => "analytics:read:extensions".to_string(),
            UserEdit => "user:edit".to_string(),
            UserReadEmail => "user:read:email".to_string(),
            UserReadStreamKey => "user:read:stream_key".to_string(),
            ClipsEdit => "clips:edit".to_string(),
            BitsRead => "bits:read".to_string(),
            AnalyticsReadGames => "analytics:read:games".to_string(),
            UserEditBroadcast => "user:edit:broadcast".to_string(),
            UserReadBroadcast => "user:read:broadcast".to_string(),
            ChatRead => "chat:read".to_string(),
            ChatEdit => "chat:edit".to_string(),
            ChannelModerate => "channel:moderate".to_string(),
            ChannelReadSubscriptions => "channel:read:subscriptions".to_string(),
            ChannelReadHypeTrain => "channel:read:hype_train".to_string(),
            WhispersRead => "whispers:read".to_string(),
            WhispersEdit => "whispers:edit".to_string(),
            ModerationRead => "moderation:read".to_string(),
            ChannelReadRedemptions => "channel:read:redemptions".to_string(),
            ChannelEditCommercial => "channel:edit:commercial".to_string(),
            Other(s) => s.clone(),
        };
        oauth2::Scope::new(s)
    }

    /// Get a vec of all defined twitch [Scopes][Scope]
    pub fn all() -> Vec<Scope> {
        vec![
            Scope::AnalyticsReadExtensions,
            Scope::UserEdit,
            Scope::UserReadEmail,
            //Scope::UserReadStreamKey, // Broken?
            Scope::ClipsEdit,
            Scope::BitsRead,
            Scope::AnalyticsReadGames,
            Scope::UserEditBroadcast,
            Scope::UserReadBroadcast,
            Scope::ChatRead,
            Scope::ChatEdit,
            Scope::ChannelModerate,
            Scope::ChannelReadSubscriptions,
            Scope::ChannelReadHypeTrain,
            Scope::WhispersRead,
            Scope::WhispersEdit,
            Scope::ModerationRead,
            Scope::ChannelReadRedemptions,
            Scope::ChannelEditCommercial,
        ]
    }
}

impl From<oauth2::Scope> for Scope {
    fn from(scope: oauth2::Scope) -> Self {
        use self::Scope::*;
        match scope.as_str() {
            "analytics:read:extensions" => AnalyticsReadExtensions,
            "user:edit" => UserEdit,
            "user:read:email" => UserReadEmail,
            "user:read:stream_key" => Scope::UserReadStreamKey,
            "clips:edit" => ClipsEdit,
            "bits:read" => BitsRead,
            "analytics:read:games" => AnalyticsReadGames,
            "user:edit:broadcast" => UserEditBroadcast,
            "user:read:broadcast" => UserReadBroadcast,
            "chat:read" => ChatRead,
            "chat:edit" => ChatEdit,
            "channel:moderate" => ChannelModerate,
            "channel:read:subscriptions" => ChannelReadSubscriptions,
            "channel:read:hype_train" => ChannelReadHypeTrain,
            "whispers:read" => WhispersRead,
            "whispers:edit" => WhispersEdit,
            "moderation:read" => ModerationRead,
            "channel:read:redemptions" => ChannelReadRedemptions,
            "channel:edit::commercial" => ChannelEditCommercial,
            s => Other(s.to_string()),
        }
    }
}
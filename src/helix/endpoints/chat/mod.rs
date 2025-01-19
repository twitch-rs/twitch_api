//! Helix endpoints regarding chat
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Chat 🟢 15/15</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Chatters](https://dev.twitch.tv/docs/api/reference#get-chatters) | [`HelixClient::get_chatters`](crate::helix::HelixClient::get_chatters) | [`get_chatters`] |
//! | [Get Channel Emotes](https://dev.twitch.tv/docs/api/reference#get-channel-emotes) | [`HelixClient::get_channel_emotes_from_id`](crate::helix::HelixClient::get_channel_emotes_from_id), [`HelixClient::get_channel_emotes_from_login`](crate::helix::HelixClient::get_channel_emotes_from_login) | [`get_channel_emotes`] |
//! | [Get Global Emotes](https://dev.twitch.tv/docs/api/reference#get-global-emotes) | [`HelixClient::get_global_emotes`](crate::helix::HelixClient::get_global_emotes) | [`get_global_emotes`] |
//! | [Get Emote Sets](https://dev.twitch.tv/docs/api/reference#get-emote-sets) | [`HelixClient::get_emote_sets`](crate::helix::HelixClient::get_emote_sets) | [`get_emote_sets`] |
//! | [Get Channel Chat Badges](https://dev.twitch.tv/docs/api/reference#get-channel-chat-badges) | - | [`get_channel_chat_badges`] |
//! | [Get Global Chat Badges](https://dev.twitch.tv/docs/api/reference#get-global-chat-badges) | - | [`get_global_chat_badges`] |
//! | [Get Chat Settings](https://dev.twitch.tv/docs/api/reference#get-chat-settings) | [`HelixClient::get_chat_settings`](crate::helix::HelixClient::get_chat_settings) | [`get_chat_settings`] |
//! | [Get Shared Chat Session](https://dev.twitch.tv/docs/api/reference#get-shared-chat-session) | [`HelixClient::get_shared_chat_session`](crate::helix::HelixClient::get_shared_chat_session) | [`get_shared_chat_session`] |
//! | [Get User Emotes](https://dev.twitch.tv/docs/api/reference#get-user-emotes) | [`HelixClient::get_user_emotes`](crate::helix::HelixClient::get_user_emotes), [`HelixClient::get_user_emotes_in_channel`](crate::helix::HelixClient::get_user_emotes_in_channel) | [`get_user_emotes`] |
//! | [Update Chat Settings](https://dev.twitch.tv/docs/api/reference#update-chat-settings) | - | [`update_chat_settings`] |
//! | [Send Chat Announcement](https://dev.twitch.tv/docs/api/reference#send-chat-announcement) | [`HelixClient::send_chat_announcement`](crate::helix::HelixClient::send_chat_announcement) | [`send_chat_announcement`] |
//! | [Send a Shoutout](https://dev.twitch.tv/docs/api/reference#send-a-shoutout) | - | [`send_a_shoutout`] |
//! | [Send Chat Message](https://dev.twitch.tv/docs/api/reference#send-chat-message) | [`HelixClient::send_chat_message`](crate::helix::HelixClient::send_chat_message), [`HelixClient::send_chat_message_reply`](crate::helix::HelixClient::send_chat_message_reply) | [`send_chat_message`] |
//! | [Get User Chat Color](https://dev.twitch.tv/docs/api/reference#get-user-chat-color) | [`HelixClient::get_user_chat_color`](crate::helix::HelixClient::get_user_chat_color), [`HelixClient::get_users_chat_colors`](crate::helix::HelixClient::get_users_chat_colors) | [`get_user_chat_color`] |
//! | [Update User Chat Color](https://dev.twitch.tv/docs/api/reference#update-user-chat-color) | [`HelixClient::update_user_chat_color`](crate::helix::HelixClient::update_user_chat_color) | [`update_user_chat_color`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types::{self, EmoteUrlBuilder},
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod get_channel_chat_badges;
pub mod get_channel_emotes;
pub mod get_chat_settings;
pub mod get_chatters;
pub mod get_emote_sets;
pub mod get_global_chat_badges;
pub mod get_global_emotes;
pub mod get_shared_chat_session;
pub mod get_user_chat_color;
pub mod get_user_emotes;
pub mod send_a_shoutout;
pub mod send_chat_announcement;
pub mod send_chat_message;
pub mod update_chat_settings;
pub mod update_user_chat_color;

#[doc(inline)]
pub use get_channel_chat_badges::GetChannelChatBadgesRequest;
#[doc(inline)]
pub use get_channel_emotes::GetChannelEmotesRequest;
#[doc(inline)]
pub use get_chat_settings::GetChatSettingsRequest;
#[doc(inline)]
pub use get_chatters::{Chatter, GetChattersRequest};
#[doc(inline)]
pub use get_emote_sets::GetEmoteSetsRequest;
#[doc(inline)]
pub use get_global_chat_badges::GetGlobalChatBadgesRequest;
#[doc(inline)]
pub use get_global_emotes::GetGlobalEmotesRequest;
#[doc(inline)]
pub use get_shared_chat_session::{
    GetSharedChatSessionRequest, SharedChatParticipant, SharedChatSession,
};
#[doc(inline)]
pub use get_user_chat_color::{GetUserChatColorRequest, UserChatColor};
#[doc(inline)]
pub use get_user_emotes::{GetUserEmotesRequest, UserEmote};
#[doc(inline)]
pub use send_a_shoutout::{SendAShoutoutRequest, SendAShoutoutResponse};
#[doc(inline)]
pub use send_chat_announcement::{
    SendChatAnnouncementBody, SendChatAnnouncementRequest, SendChatAnnouncementResponse,
};
#[doc(inline)]
pub use send_chat_message::{
    ChatMessageDropCode, ChatMessageDropReason, SendChatMessageBody, SendChatMessageRequest,
    SendChatMessageResponse,
};
#[doc(inline)]
pub use update_chat_settings::{UpdateChatSettingsBody, UpdateChatSettingsRequest};
#[doc(inline)]
pub use update_user_chat_color::{UpdateUserChatColorRequest, UpdateUserChatColorResponse};

#[doc(inline)]
pub use crate::extra::AnnouncementColor;

/// A set of badges
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BadgeSet {
    /// ID for the chat badge set.
    pub set_id: types::BadgeSetId,
    /// Contains chat badge objects for the set.
    pub versions: Vec<ChatBadge>,
}

/// A chat Badge
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChatBadge {
    /// ID of the chat badge version.
    pub id: types::ChatBadgeId,
    // FIXME: Use types::Image, see https://github.com/serde-rs/serde/issues/1504
    /// URL to png of size 28x28
    pub image_url_1x: String,
    /// URL to png of size 56x56
    pub image_url_2x: String,
    /// URL to png of size 112x112
    pub image_url_4x: String,
    /// Title of the badge
    pub title: String,
    /// Descrition of the badge
    pub description: String,
}

/// A chat emote
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelEmote {
    /// ID of the emote.
    pub id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    pub name: String,
    /// Object of image URLs for the emote.
    pub images: types::Image,
    /// If the emote_type is "subscriptions", this indicates the subscriber tier at which the emote is unlocked. Set to an empty string otherwise.
    #[serde(
        default,
        deserialize_with = "crate::deserialize_none_from_empty_string"
    )]
    pub tier: Option<types::SubscriptionTier>,
    // FIXME: Enumify?
    /// The type of emote.
    ///
    /// The most common values for custom channel emotes are
    ///
    /// `subscriptions`: Indicates a custom subscriber emote.
    ///
    /// `bitstier`: Indicates a custom Bits tier emote.
    ///
    /// `follower`: Indicates a custom follower emote.
    pub emote_type: String,
    /// ID of the emote set the emote belongs to.
    pub emote_set_id: types::EmoteSetId,
    /// The formats that the emote is available in.
    pub format: Vec<types::EmoteAnimationSetting>,
    /// The sizes that the emote is available in.
    pub scale: Vec<types::EmoteScale>,
    /// The background themes that the emote is available in.
    pub theme_mode: Vec<types::EmoteThemeMode>,
}

impl ChannelEmote {
    /// Create an emote builder for this emote.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// # use twitch_api::{client, helix, types};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
    /// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
    /// # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
    /// let emotes = client.get_channel_emotes_from_login("twitchdev", &token).await?.expect("user not found");
    /// assert_eq!(emotes[0].url().size_3x().dark_mode().render(), "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_dc24652ada1e4c84a5e3ceebae4de709/default/dark/3.0");
    /// # Ok(())
    /// # }
    /// ```
    pub fn url(&self) -> types::EmoteUrlBuilder<'_> { EmoteUrlBuilder::new(&self.id) }
}

/// A chat emote
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalEmote {
    /// ID of the emote.
    pub id: types::EmoteId,
    /// Name of the emote a viewer types into Twitch chat for the image to appear.
    pub name: String,
    /// Object of image URLs for the emote.
    pub images: types::Image,
    /// The formats that the emote is available in.
    pub format: Vec<types::EmoteAnimationSetting>,
    /// The sizes that the emote is available in.
    pub scale: Vec<types::EmoteScale>,
    /// The background themes that the emote is available in.
    pub theme_mode: Vec<types::EmoteThemeMode>,
}

/// Chat settings
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChatSettings {
    /// The ID of the broadcaster specified in the request.
    pub broadcaster_id: types::UserId,
    /// A Boolean value that determines whether chat messages must contain only emotes. Is true, if only messages that are 100% emotes are allowed; otherwise, false.
    pub emote_mode: bool,
    /// A Boolean value that determines whether the broadcaster restricts the chat room to followers only, based on how long they’ve followed.
    ///
    /// Is true, if the broadcaster restricts the chat room to followers only; otherwise, false.
    /// See [`follower_mode_duration`](Self::follower_mode_duration) for how long the followers must have followed the broadcaster to participate in the chat room.
    pub follower_mode: bool,
    /// The length of time, in minutes, that the followers must have followed the broadcaster to participate in the chat room. See [`follower_mode`](Self::follower_mode).
    ///
    /// Is null if [`follower_mode`](Self::follower_mode) is false.
    pub follower_mode_duration: Option<u64>,
    /// The ID of the moderator specified in the request for chat settings.
    pub moderator_id: Option<types::UserId>,
    /// A Boolean value that determines whether the broadcaster adds a short delay before chat messages appear in the chat room. This gives chat moderators and bots a chance to remove them before viewers can see the message.
    ///
    /// Is true, if the broadcaster applies a delay; otherwise, false.
    /// See [`non_moderator_chat_delay_duration`](Self::non_moderator_chat_delay_duration) for the length of the delay.
    ///
    /// # Notes
    ///
    /// This field and [`non_moderator_chat_delay_duration`](Self::non_moderator_chat_delay_duration) are not received when the request is made without a specified `moderator_id`.
    pub non_moderator_chat_delay: Option<bool>,
    /// The amount of time, in seconds, that messages are delayed from appearing in chat. See [`non_moderator_chat_delay`](Self::non_moderator_chat_delay).
    ///
    /// Is null if [`non_moderator_chat_delay`](Self::non_moderator_chat_delay) is false.
    pub non_moderator_chat_delay_duration: Option<u64>,
    /// A Boolean value that determines whether the broadcaster limits how often users in the chat room are allowed to send messages.
    ///
    /// Is true, if the broadcaster applies a delay; otherwise, false.
    /// See [`slow_mode_wait_time`](Self::slow_mode_wait_time) for the delay.
    pub slow_mode: bool,
    /// The amount of time, in seconds, that users need to wait between sending messages. See slow_mode.
    ///
    /// Is null if slow_mode is false.
    pub slow_mode_wait_time: Option<u64>,
    /// A Boolean value that determines whether only users that subscribe to the broadcaster’s channel can talk in the chat room.
    ///
    /// Is true, if the broadcaster restricts the chat room to subscribers only; otherwise, false.
    pub subscriber_mode: bool,
    /// A Boolean value that determines whether the broadcaster requires users to post only unique messages in the chat room.
    ///
    /// Is true, if the broadcaster requires unique messages only; otherwise, false.
    pub unique_chat_mode: bool,
}

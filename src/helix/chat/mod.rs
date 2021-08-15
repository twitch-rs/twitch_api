//! Helix endpoints regarding chat

use crate::{
    helix::{self, Request},
    types::{self, EmoteUrlBuilder},
};
use serde::{Deserialize, Serialize};

pub mod get_channel_chat_badges;
pub mod get_channel_emotes;
pub mod get_emote_sets;
pub mod get_global_chat_badges;
pub mod get_global_emotes;

#[doc(inline)]
pub use get_channel_chat_badges::GetChannelChatBadgesRequest;

#[doc(inline)]
pub use get_global_chat_badges::GetGlobalChatBadgesRequest;

#[doc(inline)]
pub use get_channel_emotes::GetChannelEmotesRequest;

#[doc(inline)]
pub use get_global_emotes::GetGlobalEmotesRequest;

#[doc(inline)]
pub use get_emote_sets::GetEmoteSetsRequest;

/// A set of badges
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BadgeSet {
    /// ID for the chat badge set.
    pub set_id: types::BadgeSetId,
    /// Contains chat badge objects for the set.
    pub versions: Vec<ChatBadge>,
}

/// A chat Badge
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
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
}

/// A chat emote
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
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
        deserialize_with = "helix::deserialize_none_from_empty_string"
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
    /// # use twitch_api2::{client, helix};
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
    pub fn url(&self) -> types::EmoteUrlBuilder<'_> {
        EmoteUrlBuilder {
            id: std::borrow::Cow::Borrowed(&self.id),
            animation_setting: <_>::default(),
            theme_mode: <_>::default(),
            scale: <_>::default(),
            template: types::EMOTE_V2_URL_TEMPLATE.into(),
        }
    }
}

/// A chat emote
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
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

#![doc(alias = "channel.goal")]
//! Chat events
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod clear;
pub mod clear_user_messages;
pub mod message_delete;
pub mod notification;

#[doc(inline)]
pub use clear::{ChannelChatClearV1, ChannelChatClearV1Payload};
#[doc(inline)]
pub use clear_user_messages::{
    ChannelChatClearUserMessagesV1, ChannelChatClearUserMessagesV1Payload,
};
#[doc(inline)]
pub use message_delete::{ChannelChatMessageDeleteV1, ChannelChatMessageDeleteV1Payload};
#[doc(inline)]
pub use notification::{ChannelChatNotificationV1, ChannelChatNotificationV1Payload};

/// A chat message fragment
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Fragment {
    /// A Cheermote.
    Cheermote {
        /// Message text in fragment
        text: String,
        /// A Cheermote.
        cheermote: Cheermote,
    },
    /// A Emote.
    Emote {
        /// Message text in fragment
        text: String,
        /// A Emote.
        emote: Emote,
    },
    /// A Mention.
    Mention {
        /// Message text in fragment
        text: String,
        /// A Mention.
        mention: Mention,
    },
    /// A text fragment, see [`Fragment::text`].
    Text {
        /// Message text in fragment
        text: String,
    },
}

impl Fragment {
    /// Get the text data
    pub fn text(&self) -> &str {
        match self {
            Fragment::Cheermote { text, .. } => text,
            Fragment::Emote { text, .. } => text,
            Fragment::Mention { text, .. } => text,
            Fragment::Text { text } => text,
        }
    }
}

/// A cheermote fragment
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Cheermote {
    /// The name portion of the Cheermote string that you use in chat to cheer Bits.
    ///
    /// The full Cheermote string is the concatenation of {prefix} + {number of Bits}.
    /// For example, if the prefix is “Cheer” and you want to cheer 100 Bits, the full Cheermote string is Cheer100.
    /// When the Cheermote string is entered in chat, Twitch converts it to the image associated with the Bits tier that was cheered.
    pub prefix: String,
    /// The amount of bits cheered.
    pub bits: i32,
    /// The tier level of the cheermote.
    pub tier: i32,
}

/// An emote fragment
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Emote {
    /// An ID that uniquely identifies this emote.
    pub id: types::EmoteId,
    /// An ID that identifies the emote set that the emote belongs to.
    pub emote_set_id: types::EmoteSetId,
    /// The ID of the broadcaster who owns the emote.
    pub owner_id: types::UserId,
    /// The formats that the emote is available in. For example, if the emote is available only as a static PNG, the array contains only static. But if the emote is available as a static PNG and an animated GIF, the array contains static and animated. The possible formats are:
    ///
    /// * `animated` — An animated GIF is available for this emote.
    /// * `static` — A static PNG file is available for this emote.
    pub format: Vec<types::EmoteAnimationSetting>,
}

/// A user mention fragment
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Mention {
    /// The user ID of the mentioned user.
    pub user_id: types::UserId,
    /// The user name of the mentioned user.
    pub user_name: types::DisplayName,
    /// The user login of the mentioned user.
    pub user_login: types::UserName,
}

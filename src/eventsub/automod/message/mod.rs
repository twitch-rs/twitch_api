#![doc(alias = "automod.message")]
//! Events for Automod actions on messages
use super::{AutomodCategory, EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod hold;
pub mod update;

#[doc(inline)]
pub use hold::{AutomodMessageHoldV1, AutomodMessageHoldV1Payload};
#[doc(inline)]
pub use hold::{AutomodMessageHoldV2, AutomodMessageHoldV2Payload};
#[doc(inline)]
pub use update::{AutomodMessageUpdateV1, AutomodMessageUpdateV1Payload};
#[doc(inline)]
pub use update::{AutomodMessageUpdateV2, AutomodMessageUpdateV2Payload};

/// A message's Automod status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutomodMessageStatus {
    /// The message was approved (shown in chat)
    Approved,
    /// The message was denied (not shown in chat)
    Denied,
    /// The message is too old, it can't be acted upon anymore.
    Expired,
    /// The message was invalid
    Invalid,
    /// An unknown Automod message status, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// A message sent in automod events
///
/// This message is different to the one from [channel.chat.message]
/// in that it doesn't contain "mention" fragments and that the "emote" fragment
/// doesn't contain the owner nor the format.
// XXX: this struct can never be deny_unknown_fields
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutomodMessage {
    /// The chat message in plain text.
    pub text: String,
    /// Ordered list of chat message fragments.
    pub fragments: Vec<AutomodMessageFragment>,
}

/// A chat message fragment
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AutomodMessageFragment {
    /// A Cheermote.
    Cheermote {
        /// Message text in a fragment
        text: String,
        /// A Cheermote.
        cheermote: AutomodMessageCheermote,
    },
    /// A Emote.
    Emote {
        /// Message text in a fragment
        text: String,
        /// A Emote.
        emote: AutomodMessageEmote,
    },
    /// A text fragment, see [`AutomodMessageFragment::text`].
    Text {
        /// Message text in a fragment
        text: String,
    },
}

impl AutomodMessageFragment {
    /// Get the text data
    pub fn text(&self) -> &str {
        match self {
            Self::Cheermote { text, .. } => text,
            Self::Emote { text, .. } => text,
            Self::Text { text } => text,
        }
    }
}

/// A cheermote in a message filtered by automod
pub type AutomodMessageCheermote = crate::eventsub::channel::chat::Cheermote;

/// An emote in a message filtered by automod
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageEmote {
    /// An ID that uniquely identifies this emote.
    pub id: types::EmoteId,
    /// An ID that identifies the emote set that the emote belongs to.
    pub emote_set_id: types::EmoteSetId,
}

/// The reason why a message was held
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
#[non_exhaustive]
pub enum AutomodHeldReason {
    /// The message was caught by automod's rules
    Automod {
        /// Information on why a message was caught by automod
        automod: AutomodMessageInfo,
    },
    /// The message was caught because of one or more blocked terms
    BlockedTerm {
        /// Information on which blocked terms were matched in a message
        blocked_term: AutomodBlockedTermInfo,
    },
}

/// Information on why a message was caught by automod
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageInfo {
    /// The category of the caught message.
    pub category: AutomodCategory,
    /// The level of severity (1-4).
    pub level: u8,
    /// The bounds of the text that caused the message to be caught.
    pub boundaries: Vec<AutomodMessageBoundary>,
}

/// Information on which blocked terms were matched in a message
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodBlockedTermInfo {
    /// The list of blocked terms found in the message.
    pub terms_found: Vec<AutomodBlockedTerm>,
}

/// The bounds of the text that caused the message to be caught.
///
/// These bounds are given in Unicode code points not in bytes.
/// See [char] and [str::chars].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodMessageBoundary {
    /// Index in the message for the start of the problem (0 indexed, inclusive).
    pub start_pos: usize,
    /// Index in the message for the end of the problem (0 indexed, inclusive).
    pub end_pos: usize,
}

/// Information about the blocked terms that caused a message to be caught by automod.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodBlockedTermsInfo {
    /// The list of blocked terms found in the message.
    pub terms_found: Vec<AutomodBlockedTerm>,
}

/// A blocked term that was found in a message
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct AutomodBlockedTerm {
    /// The id of the blocked term found.
    pub term_id: types::BlockedTermId,
    /// The bounds of the text that caused the message to be caught.
    pub boundary: AutomodMessageBoundary,
    /// The id of the broadcaster that owns the blocked term.
    pub owner_broadcaster_user_id: types::UserId,
    /// The login of the broadcaster that owns the blocked term.
    pub owner_broadcaster_user_login: types::UserName,
    /// The username of the broadcaster that owns the blocked term.
    pub owner_broadcaster_user_name: types::DisplayName,
}

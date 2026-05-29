#![doc(alias = "channel.goal")]
//! Chat events
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod clear;
pub mod clear_user_messages;
pub mod message;
pub mod message_delete;
pub mod notification;
pub mod user_message_hold;
pub mod user_message_update;

#[doc(inline)]
pub use clear::{ChannelChatClearV1, ChannelChatClearV1Payload};
#[doc(inline)]
pub use clear_user_messages::{
    ChannelChatClearUserMessagesV1, ChannelChatClearUserMessagesV1Payload,
};
#[doc(inline)]
pub use message::{ChannelChatMessageV1, ChannelChatMessageV1Payload};
#[doc(inline)]
pub use message_delete::{ChannelChatMessageDeleteV1, ChannelChatMessageDeleteV1Payload};
#[doc(inline)]
pub use notification::{ChannelChatNotificationV1, ChannelChatNotificationV1Payload};
#[doc(inline)]
pub use user_message_hold::{ChannelChatUserMessageHoldV1, ChannelChatUserMessageHoldV1Payload};
#[doc(inline)]
pub use user_message_update::{
    ChannelChatUserMessageUpdateV1, ChannelChatUserMessageUpdateV1Payload,
};

#[doc(inline)]
pub use crate::common::chat::{Cheermote, Emote, Fragment, Mention, Message};

/// A badge
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Badge {
    /// An ID that identifies this set of chat badges. For example, Bits or Subscriber.
    pub set_id: types::BadgeSetId,
    /// An ID that identifies this version of the badge. The ID can be any value. For example, for Bits, the ID is the Bits tier level, but for World of Warcraft, it could be Alliance or Horde.
    pub id: types::ChatBadgeId,
    /// Contains metadata related to the chat badges in the badges tag. Currently, this tag contains metadata only for subscriber badges, to indicate the number of months the user has been a subscriber.
    pub info: String,
}

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

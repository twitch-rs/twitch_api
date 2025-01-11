#![doc(alias = "channel.chat_settings")]
//! A broadcaster's chat settings are updated
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod update;

#[doc(inline)]
pub use update::{ChannelChatSettingsUpdateV1, ChannelChatSettingsUpdateV1Payload};

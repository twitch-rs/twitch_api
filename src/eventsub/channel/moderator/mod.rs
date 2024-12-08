#![doc(alias = "channel.moderator")]
//! A user's moderator privileges on a specified channel are changed.
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod add;
pub mod remove;

#[doc(inline)]
pub use add::{ChannelModeratorAddV1, ChannelModeratorAddV1Payload};
#[doc(inline)]
pub use remove::{ChannelModeratorRemoveV1, ChannelModeratorRemoveV1Payload};

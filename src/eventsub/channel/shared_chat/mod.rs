#![doc(alias = "channel.shared_chat")]
//! Events related to shared chat
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod begin;
pub mod end;
pub mod update;

#[doc(inline)]
pub use begin::{ChannelSharedChatBeginV1, ChannelSharedChatBeginV1Payload};
#[doc(inline)]
pub use end::{ChannelSharedChatEndV1, ChannelSharedChatEndV1Payload};
#[doc(inline)]
pub use update::{ChannelSharedChatUpdateV1, ChannelSharedChatUpdateV1Payload};

/// A participant in a shared chat session
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Participant {
    /// The User ID of the participant channel.
    pub broadcaster_user_id: types::UserId,
    /// The display name of the participant channel.
    pub broadcaster_user_name: types::UserName,
    /// The user login of the participant channel.
    pub broadcaster_user_login: types::DisplayName,
}

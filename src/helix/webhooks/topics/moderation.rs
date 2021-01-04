//! Topics for streams
use crate::helix::webhooks::Topic;

pub mod moderator_change_events;

#[doc(inline)]
pub use moderator_change_events::ModeratorChangedTopic;

pub mod channel_ban_change_events;

#[doc(inline)]
pub use channel_ban_change_events::ChannelBanChangeEventsTopic;

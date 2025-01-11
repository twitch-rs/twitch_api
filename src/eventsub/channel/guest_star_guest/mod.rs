#![doc(alias = "channel.guest_star_guest")]
//! Events regarding guests of guest star sessions
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod update;

#[doc(inline)]
pub use update::{ChannelGuestStarGuestUpdateBeta, ChannelGuestStarGuestUpdateBetaPayload};

/// The current state of a user in a guest star session
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum GuestState {
    /// The guest has transitioned to the invite queue.
    ///
    /// This can take place when the guest was previously assigned a slot, but have been removed from the call and are sent back to the invite queue.
    Invited,
    /// The guest has accepted the invite and is currently in the process of setting up to join the session.
    Accepted,
    /// The guest has signaled they are ready and can be assigned a slot.
    Ready,
    /// The guest has been assigned a slot in the session, but is not currently seen live in the broadcasting software.
    Backstage,
    /// The guest is now live in the host's broadcasting software.
    Live,
    /// The guest was removed from the call or queue.
    Removed,
    /// An unknown state, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

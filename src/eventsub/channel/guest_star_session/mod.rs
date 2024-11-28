#![doc(alias = "channel.guest_star_session")]
//! Events regarding guest star sessions
use super::{EventSubscription, EventType};
use crate::types;
use serde_derive::{Deserialize, Serialize};

pub mod begin;
pub mod end;

#[doc(inline)]
pub use begin::{ChannelGuestStarSessionBeginBeta, ChannelGuestStarSessionBeginBetaPayload};
#[doc(inline)]
pub use end::{ChannelGuestStarSessionEndBeta, ChannelGuestStarSessionEndBetaPayload};

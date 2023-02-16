#![doc(alias = "channel.poll")]
//! Poll on a specific channel has been begun, ended or progressed.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod donate;

#[doc(inline)]
pub use donate::{ChannelCharityCampaignDonateV1, ChannelCharityCampaignDonateV1Payload};

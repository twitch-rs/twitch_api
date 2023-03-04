#![doc(alias = "channel.poll")]
//! Poll on a specific channel has been begun, ended or progressed.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

pub mod donate;
pub mod progress;
pub mod start;
pub mod stop;

#[doc(inline)]
pub use donate::{ChannelCharityCampaignDonateV1, ChannelCharityCampaignDonateV1Payload};
#[doc(inline)]
pub use progress::{ChannelCharityCampaignProgressV1, ChannelCharityCampaignProgressV1Payload};
#[doc(inline)]
pub use start::{ChannelCharityCampaignStartV1, ChannelCharityCampaignStartV1Payload};
#[doc(inline)]
pub use stop::{ChannelCharityCampaignStopV1, ChannelCharityCampaignStopV1Payload};

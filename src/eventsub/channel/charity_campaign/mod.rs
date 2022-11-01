#![doc(alias = "channel.poll")]
//! Poll on a specific channel has been begun, ended or progressed.
use super::{EventSubscription, EventType};
use crate::types;
use serde::{Deserialize, Serialize};

#[cfg(feature = "unsupported")]
pub mod donate;

#[doc(inline)]
#[cfg(feature = "unsupported")]
pub use donate::{ChannelCharityCampaignDonateBeta, ChannelCharityCampaignDonateBetaPayload};

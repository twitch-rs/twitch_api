//! Helix endpoints regarding charities
use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod get_charity_campaign;
pub mod get_charity_campaign_donations;

#[doc(inline)]
pub use get_charity_campaign::{CharityCampaign, GetCharityCampaignRequest};
#[doc(inline)]
pub use get_charity_campaign_donations::{
    CharityCampaignDonation, GetCharityCampaignDonationsRequest,
};

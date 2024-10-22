//! Helix endpoints regarding charities
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Charity 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Charity Campaign](https://dev.twitch.tv/docs/api/reference#get-charity-campaign) | - | [`get_charity_campaign`] |
//! | [Get Charity Campaign Donations](https://dev.twitch.tv/docs/api/reference#get-charity-campaign-donations) | - | [`get_charity_campaign_donations`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->
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

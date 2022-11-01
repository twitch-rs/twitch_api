//! Helix endpoints regarding charities
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[cfg(feature = "unsupported")]
pub mod get_charity_campaign;

#[doc(inline)]
#[cfg(feature = "unsupported")]
pub use get_charity_campaign::{CharityCampaign, GetCharityCampaignRequest};

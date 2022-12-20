//! Helix endpoints regarding hype trains
//!
//! See also [PubSub hypetrain](crate::pubsub::hypetrain)
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod get_hypetrain_events;

#[doc(inline)]
pub use get_hypetrain_events::GetHypeTrainEventsRequest;

/// Type of contribution to a hype train
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "UPPERCASE")]
pub enum ContributionType {
    /// Bits
    Bits,
    /// Channel Subscriptions. Either gifted or not.
    Subscription,
    /// Covers other contribution methods not listed.
    Other,
}

/// A contribution to a hype train
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Contribution {
    /// The total contributed.
    pub total: i64,
    #[serde(rename = "type")]
    /// Type of contribution. Valid values include bits, subscription.
    pub type_: ContributionType,
    /// The ID of the user.
    pub user: types::UserId,
}

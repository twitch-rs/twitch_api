//! Helix endpoints regarding hype trains
//!
//! See also [PubSub hypetrain](crate::pubsub::hypetrain)
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Hype Train 🟡 1/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Hype Train Events](https://dev.twitch.tv/docs/api/reference#get-hype-train-events) | - | [`get_hypetrain_events`] |
//! | [Get Hype Train Status](https://dev.twitch.tv/docs/api/reference#get-hype-train-status) | - | - |
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

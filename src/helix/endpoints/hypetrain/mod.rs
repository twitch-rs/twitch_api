//! Helix endpoints regarding hype trains
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Hype Train 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Hype Train Status](https://dev.twitch.tv/docs/api/reference#get-hype-train-status) | [`HelixClient::get_hype_train_status`](crate::helix::HelixClient::get_hype_train_status) | [`get_hype_train_status`] |
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

pub mod get_hype_train_status;

// Re-exported from common for backwards compatability
#[doc(inline)]
pub use crate::common::hypetrain::{
    HypeTrainContributionType, HypeTrainType, SharedTrainParticipant,
};

#[doc(inline)]
pub use get_hype_train_status::{
    GetHypeTrainStatusRequest, HypeTrain, HypeTrainRecord, HypeTrainStatus,
};

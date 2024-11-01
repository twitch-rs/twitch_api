//! Helix endpoints regarding channel raids
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Raids 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Start a raid](https://dev.twitch.tv/docs/api/reference#start-a-raid) | [`HelixClient::start_a_raid`](crate::helix::HelixClient::start_a_raid) | [`start_a_raid`] |
//! | [Cancel a raid](https://dev.twitch.tv/docs/api/reference#cancel-a-raid) | [`HelixClient::cancel_a_raid`](crate::helix::HelixClient::cancel_a_raid) | [`cancel_a_raid`] |
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

pub mod cancel_a_raid;
pub mod start_a_raid;

#[doc(inline)]
pub use cancel_a_raid::{CancelARaidRequest, CancelARaidResponse};
#[doc(inline)]
pub use start_a_raid::{StartARaidRequest, StartARaidResponse};

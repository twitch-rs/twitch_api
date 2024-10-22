//! Helix endpoints regarding creator goals
//!
//! See also [EventSub Creator goals](crate::eventsub::channel::goal)
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Goals 🟢 1/1</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Creator Goals](https://dev.twitch.tv/docs/api/reference#get-creator-goals) | - | [`get_creator_goals`] |
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

pub mod get_creator_goals;

#[doc(inline)]
pub use get_creator_goals::{CreatorGoal, GetCreatorGoalsRequest};

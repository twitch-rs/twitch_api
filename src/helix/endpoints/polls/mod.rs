//! Helix endpoints regarding channel polls
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Polls 🟢 3/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Polls](https://dev.twitch.tv/docs/api/reference#get-polls) | - | [`get_polls`] |
//! | [Create Poll](https://dev.twitch.tv/docs/api/reference#create-poll) | - | [`create_poll`] |
//! | [End Poll](https://dev.twitch.tv/docs/api/reference#end-poll) | - | [`end_poll`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->
#![allow(deprecated, deprecated_in_future)]

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod create_poll;
pub mod end_poll;
pub mod get_polls;

#[doc(inline)]
pub use create_poll::{CreatePollBody, CreatePollRequest, NewPollChoice};
#[doc(inline)]
pub use end_poll::{EndPollBody, EndPollRequest};
#[doc(inline)]
pub use get_polls::{GetPollsRequest, Poll};

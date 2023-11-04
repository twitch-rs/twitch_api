//! Helix endpoints regarding channel polls
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

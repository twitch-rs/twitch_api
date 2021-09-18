//! Helix endpoints regarding creator goals
//!
//! See also [EventSub Creator goals](crate::eventsub::channel::goal)
use crate::{
    helix::{self, Request},
    types,
};

use serde::{Deserialize, Serialize};

pub mod get_creator_goals;

#[doc(inline)]
pub use get_creator_goals::{CreatorGoal, GetCreatorGoalsRequest};

//! Helix endpoints regarding games
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod get_games;
pub mod get_top_games;

#[doc(inline)]
pub use get_games::GetGamesRequest;
#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;
pub use types::TwitchCategory as Game;

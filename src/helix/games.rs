#![doc(alias = "games")]
//! Endpoints regarding games
use crate::{helix, types};

use serde::{Deserialize, Serialize};

pub mod get_games;
pub mod get_top_games;

#[doc(inline)]
pub use get_games::GetGamesRequest;
#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;
pub use types::TwitchCategory as Game;

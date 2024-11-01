//! Helix endpoints regarding games
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Games 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Top Games](https://dev.twitch.tv/docs/api/reference#get-top-games) | - | [`get_top_games`] |
//! | [Get Games](https://dev.twitch.tv/docs/api/reference#get-games) | [`HelixClient::get_games_by_id`](crate::helix::HelixClient::get_games_by_id) | [`get_games`] |
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

pub mod get_games;
pub mod get_top_games;

#[doc(inline)]
pub use get_games::GetGamesRequest;
#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;
pub use types::TwitchCategory as Game;

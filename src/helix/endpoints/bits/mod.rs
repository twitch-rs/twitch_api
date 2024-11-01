//! Helix endpoints regarding bits
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, bits::GetCheermotesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::default();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetCheermotesRequest::broadcaster_id("1234");
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Bits 🟡 2/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Bits Leaderboard](https://dev.twitch.tv/docs/api/reference#get-bits-leaderboard) | - | [`get_bits_leaderboard`] |
//! | [Get Cheermotes](https://dev.twitch.tv/docs/api/reference#get-cheermotes) | - | [`get_cheermotes`] |
//! | [Get Extension Transactions](https://dev.twitch.tv/docs/api/reference#get-extension-transactions) | - | - |
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

pub mod get_bits_leaderboard;
pub mod get_cheermotes;

#[doc(inline)]
pub use get_bits_leaderboard::{BitsLeaderboard, GetBitsLeaderboardRequest};
#[doc(inline)]
pub use get_cheermotes::{Cheermote, GetCheermotesRequest};

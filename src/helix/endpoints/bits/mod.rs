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

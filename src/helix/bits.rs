//! Helix endpoints regarding bits
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, bits::GetCheermotesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetCheermotesRequest::builder()
//!     .broadcaster_id("1234".to_string())
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```

use crate::{helix, types};
use serde::{Deserialize, Serialize};

pub mod get_bits_leaderboard;
pub mod get_cheermotes;

#[doc(inline)]
pub use get_bits_leaderboard::{BitsLeaderboard, GetBitsLeaderboardRequest};
#[doc(inline)]
pub use get_cheermotes::{Cheermote, GetCheermotesRequest};

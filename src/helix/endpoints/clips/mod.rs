//! Helix endpoints regarding clips
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::helix::{HelixClient, clips::GetClipsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetClipsRequest::builder()
//!     .game_id(Some("1234".into()))
//!     .first(100) // max 100, 20 if left unspecified
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```
use crate::{
    helix::{self, Request},
    types,
};
use serde::{Deserialize, Serialize};

pub mod get_clips;

#[doc(inline)]
pub use get_clips::{Clip, GetClipsRequest};

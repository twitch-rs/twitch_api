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
//! let req = GetClipsRequest::game_id("1234").first(100); // max 100, 20 if left unspecified
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data.first());
//! # Ok(())
//! # }
//! ```
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Clips 🟡 2/3</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create Clip](https://dev.twitch.tv/docs/api/reference#create-clip) | - | [`create_clip`] |
//! | [Get Clips](https://dev.twitch.tv/docs/api/reference#get-clips) | - | [`get_clips`] |
//! | [Get Clips Download](https://dev.twitch.tv/docs/api/reference#get-clips-download) | - | - |
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

pub mod get_clips;

pub mod create_clip;

#[doc(inline)]
pub use get_clips::{Clip, GetClipsRequest};

#[doc(inline)]
pub use create_clip::{CreateClipRequest, CreatedClip};

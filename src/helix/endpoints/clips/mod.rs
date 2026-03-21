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
//! <details open><summary style="cursor: pointer">Clips 🟡 3/4</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Create Clip](https://dev.twitch.tv/docs/api/reference#create-clip) | - | [`create_clip`] |
//! | [Create Clip From VOD](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod) | [`HelixClient::create_clip_from_vod`](crate::helix::HelixClient::create_clip_from_vod) | [`create_clip_from_vod`] |
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

pub mod create_clip_from_vod;

#[doc(inline)]
pub use get_clips::{Clip, GetClipsRequest};

#[doc(inline)]
pub use create_clip::CreateClipRequest;

#[doc(inline)]
pub use create_clip_from_vod::CreateClipFromVodRequest;

/// Return Value for [Create Clip](create_clip) and [Create Clip From VOD](create_clip_from_vod)
///
/// [`create-clip`](https://dev.twitch.tv/docs/api/reference#create-clip)
/// [`create-clip-from-vod`](https://dev.twitch.tv/docs/api/reference#create-clip-from-vod)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CreatedClip {
    /// An ID that uniquely identifies the clip.
    pub id: types::ClipId,
    /// A URL that you can use to edit the clip’s title, identify the part of the clip to publish, and publish the clip.
    ///
    /// The URL is valid for up to 24 hours or until the clip is published, whichever comes first.
    pub edit_url: String,
}

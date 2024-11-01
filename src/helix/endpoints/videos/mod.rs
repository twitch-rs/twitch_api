#![doc(alias = "vod")]
//! Helix endpoints regarding videos
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api::{helix::{HelixClient, videos::GetVideosRequest}, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api::DummyHttpClient> = &client;
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let req = GetVideosRequest::ids(&"1234");
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
//! <details open><summary style="cursor: pointer">Videos 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Videos](https://dev.twitch.tv/docs/api/reference#get-videos) | - | [`get_videos`] |
//! | [Delete Videos](https://dev.twitch.tv/docs/api/reference#delete-videos) | - | [`delete_videos`] |
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

pub mod delete_videos;
pub mod get_videos;

#[doc(inline)]
pub use get_videos::{GetVideosRequest, Video};

/// Sort order of the videos
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Sort {
    /// Sort by time
    Time,
    /// Sort by trending
    Trending,
    /// Sort by views
    Views,
}

/// Period during which the video was created
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video.
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum VideoTypeFilter {
    /// All video types
    All,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
}

//! Provides URLs to download the video file(s) for the specified clips.
//! [`get-clips-download`](https://dev.twitch.tv/docs/api/reference#get-clips-download)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetClipsDownloadRequest]
//!
//! To use this endpoint, construct a [`GetClipsDownloadRequest`] with the [`GetClipsDownloadRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::clips::get_clips_download;
//! let request = get_clips_download::GetClipsDownloadRequest::new(
//!     "1234",
//!     "5678",
//!     &["ID1", "ID2"],
//! );
//! ```
//!
//! ## Response: [DownloadableClip]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, clips::get_clips_download};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_clips_download::GetClipsDownloadRequest::new(
//!     "141981764",
//!     "141981765",
//!     vec![
//!         "InexpensiveDistinctFoxChefFrank",
//!         "SpinelessCloudyLeopardMcaT",
//!     ],
//! );
//! let response: Vec<get_clips_download::DownloadableClip> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetClipsDownloadRequest::parse_response(None, &request.get_uri(), response)`](GetClipsDownloadRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Clips Download](super::get_clips_download)
///
/// [`get-clips-download`](https://dev.twitch.tv/docs/api/reference#get-clips-download)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetClipsDownloadRequest<'a> {
    /// The User ID of the editor for the channel you want to download a clip for.
    /// If using the broadcaster’s auth token, this is the same as [`broadcaster_id`][GetClipsDownloadRequest::broadcaster_id].
    /// This must match the `user_id`` in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub editor_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster you want to download clips for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID that identifies the clip you want to download.
    /// Include this parameter for each clip you want to download, up to a maximum of 10 clips.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub clip_id: types::Collection<'a, types::ClipId>,
}

impl<'a> GetClipsDownloadRequest<'a> {
    /// Create a new clips download request
    pub fn new(
        editor_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        ids: impl Into<types::Collection<'a, types::ClipId>>,
    ) -> Self {
        Self {
            editor_id: editor_id.into_cow(),
            broadcaster_id: broadcaster_id.into_cow(),
            clip_id: ids.into(),
        }
    }
}

/// Return Values for [Get Clips Download](super::get_clips_download)
///
/// [`get-clips-download`](https://dev.twitch.tv/docs/api/reference#get-clips-download)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct DownloadableClip {
    /// An ID that uniquely identifies the clip.
    pub clip_id: types::ClipId,
    /// The landscape URL to download the clip. This field is [None] if the URL is not available.
    pub landscape_download_url: Option<String>,
    /// The portrait URL to download the clip. This field is [None] if the URL is not available.
    pub portrait_download_url: Option<String>,
}

impl Request for GetClipsDownloadRequest<'_> {
    type PaginationData = ();
    type Response = Vec<DownloadableClip>;

    const PATH: &'static str = "clips/downloads";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::EditorManageClips,
        twitch_oauth2::Scope::ChannelManageClips
    )];
}

impl RequestGet for GetClipsDownloadRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;

    let req = GetClipsDownloadRequest::new(
        "141981764",
        "141981765",
        vec![
            "InexpensiveDistinctFoxChefFrank",
            "SpinelessCloudyLeopardMcaT",
        ],
    );

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "clip_id": "InexpensiveDistinctFoxChefFrank",
                "landscape_download_url": "https://production.assets.clips.twitchcdn.net/yFZG...",
                "portrait_download_url": null
            },
            {
                "clip_id": "SpinelessCloudyLeopardMcaT",
                "landscape_download_url": "https://production.assets.clips.twitchcdn.net/542j...",
                "portrait_download_url": null
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/clips/downloads?editor_id=141981764&broadcaster_id=141981765&clip_id=InexpensiveDistinctFoxChefFrank&clip_id=SpinelessCloudyLeopardMcaT"
    );

    dbg!(GetClipsDownloadRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

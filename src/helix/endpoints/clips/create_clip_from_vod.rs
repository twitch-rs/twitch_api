//! Create a clip from the broadcaster’s VOD
//! [`create-clip-from-vod`](https://dev.twitch.tv/docs/api/reference/#create-clip-from-vod)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateClipFromVodRequest]
//!
//! To use this endpoint, construct a [`CreateClipFromVodRequest`] with the [`CreateClipFromVodRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::clips::create_clip_from_vod;
//! let request = create_clip_from_vod::CreateClipFromVodRequest::new(
//!     "12345", "67890", "abcde", 64, "title",
//! );
//! ```
//!
//! ## Response: [CreatedClip]
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, clips::{self, create_clip_from_vod}};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_clip_from_vod::CreateClipFromVodRequest::new("12345", "67890", "abcde", 64, "title");
//! let body = helix::EmptyBody;
//! let response: clips::CreatedClip = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CreateClipFromVodRequest::parse_response(None, &request.get_uri(), response)`](CreateClipFromVodRequest::parse_response)

use super::*;
use helix::RequestPost;

/// Query Parameters for [Create Clip From VOD](super::create_clip_from_vod)
///
/// The duration of a clip can be from 5 seconds to 60 seconds in length, with a default of 30 seconds if not specified.
///
/// [`vod_offset`][CreateClipFromVodRequest::vod_offset] indicates where the clip will end.
/// In other words, the clip will start at
/// ([`vod_offset`][CreateClipFromVodRequest::vod_offset] - [`duration`][CreateClipFromVodRequest::duration])
/// and end at [`vod_offset`][CreateClipFromVodRequest::vod_offset].
/// This means that the value of [`vod_offset`][CreateClipFromVodRequest::vod_offset]
/// must greater than or equal to the value of duration.
///
/// [`create-clip-from-vod`](https://dev.twitch.tv/docs/api/reference/#create-clip-from-vod)
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct CreateClipFromVodRequest<'a> {
    /// The user ID of the editor for the channel you want to create a clip for.
    /// If using the broadcaster’s auth token, this is the same as broadcaster_id.
    /// This must match the user_id in the user access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub editor_id: Cow<'a, types::UserIdRef>,

    /// The user ID for the channel you want to create a clip for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,

    /// ID of the VOD the user wants to clip.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub vod_id: Cow<'a, types::VideoIdRef>,

    /// Offset in the VOD to create the clip. See [CreateClipFromVodRequest].
    pub vod_offset: u64,

    /// The length of the clip, in seconds. Precision is 0.1. Defaults to 30. Min: 5 seconds, Max: 60 seconds.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub duration: Option<f32>,

    /// The title of the clip.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub title: Cow<'a, str>,
}

impl<'a> CreateClipFromVodRequest<'a> {
    /// Create a new [`CreateClipFromVodRequest`]
    pub fn new(
        editor_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        vod_id: impl types::IntoCow<'a, types::VideoIdRef> + 'a,
        vod_offset: u64,
        title: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            editor_id: editor_id.into_cow(),
            broadcaster_id: broadcaster_id.into_cow(),
            vod_id: vod_id.into_cow(),
            vod_offset,
            duration: None,
            title: title.into(),
        }
    }

    /// Sets the duration parameter
    pub const fn duration(mut self, duration: f32) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Alias for [super::CreatedClip]
pub type CreatedClip = super::CreatedClip;

impl Request for CreateClipFromVodRequest<'_> {
    type PaginationData = ();
    type Response = CreatedClip;

    const PATH: &'static str = "videos/clips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::EditorManageClips,
        twitch_oauth2::Scope::ChannelManageClips
    )];
}

impl RequestPost for CreateClipFromVodRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CreateClipFromVodRequest::new("12345", "67890", "abcde", 64, "title");

    let data = br#"
    {
        "data": 
        [{
            "id": "FiveWordsForClipSlug",
            "edit_url": "https://www.twitch.tv/twitchdev/clip/FiveWordsForClipSlug"
        }]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();

    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/videos/clips?editor_id=12345&broadcaster_id=67890&vod_id=abcde&vod_offset=64&title=title"
    );

    dbg!(CreateClipFromVodRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

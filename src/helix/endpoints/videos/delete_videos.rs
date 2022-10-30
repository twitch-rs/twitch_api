//! Deletes one or more videos. Videos are past broadcasts, Highlights, or uploads.
//! [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
//!
//! # Accessing the endpoint
//!
//! ## Request: [DeleteVideosRequest]
//!
//! To use this endpoint, construct a [`DeleteVideosRequest`] with the [`DeleteVideosRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::videos::delete_videos;
//! let request = delete_videos::DeleteVideosRequest::builder()
//!     .id(&["1234".into()][..])
//!     .build();
//! ```
//!
//! ## Response: [DeleteVideo]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, videos::delete_videos};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let ids: &[&types::VideoIdRef] = &["1234".into()];
//! let request = delete_videos::DeleteVideosRequest::builder()
//!     .id(ids)
//!     .build();
//! let response: delete_videos::DeleteVideo = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`DeleteVideosRequest::parse_response(None, &request.get_uri(), response)`](DeleteVideosRequest::parse_response)

use super::*;
use helix::RequestDelete;

// FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
/// Query Parameters for [Delete Videos](super::delete_videos)
///
/// [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct DeleteVideosRequest<'a> {
    /// ID of the video(s) to be deleted. Limit: 5.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[serde(borrow)]
    pub id: Cow<'a, [&'a types::VideoIdRef]>,
}

impl<'a> DeleteVideosRequest<'a> {
    /// ID of the videos to be deleted
    pub fn ids(ids: impl Into<Cow<'a, [&'a types::VideoIdRef]>>) -> Self { Self { id: ids.into() } }
}
// FIXME: Should return VideoIds
/// Return Values for [Delete Videos](super::delete_videos)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
/// [`delete-videos`](https://dev.twitch.tv/docs/api/reference#delete-videos)
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum DeleteVideo {
    /// Video(s) deleted.
    Success,
}

impl Request for DeleteVideosRequest<'_> {
    type Response = DeleteVideo;

    const PATH: &'static str = "videos";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageVideos];
}

impl RequestDelete for DeleteVideosRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT | http::StatusCode::OK => Ok(helix::Response {
                data: DeleteVideo::Success,
                pagination: None,
                request,
                total: None,
                other: None,
            }),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = DeleteVideosRequest::ids(vec!["234482848".into()]);

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/videos?id=234482848"
    );

    dbg!(DeleteVideosRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

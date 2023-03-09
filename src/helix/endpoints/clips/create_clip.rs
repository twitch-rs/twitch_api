//! Create Clip using Broadcaster ID (one only)
//! [`create-clip`](https://dev.twitch.tv/docs/api/reference/#create-clip)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CreateClipRequest]
//!
//! To use this endpoint, construct a [`CreateClipRequest`] with the [`CreateClipRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::clips::create_clip;
//! let request = create_clip::CreateClipRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [CreatedClip]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, clips::create_clip};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = create_clip::CreateClipRequest::broadcaster_id("1234");
//! let response: Vec<create_clip::CreatedClip> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`CreateClipRequest::parse_response(None, &request.get_uri(), response)`](CreateClipRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Create Clip](super::create_clip)
///
/// [`create-clip`](https://dev.twitch.tv/docs/api/reference/#create-clip)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CreateClipRequest<'a> {
    /// The ID of the broadcaster whose stream you want to create a clip from.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// A Boolean value that determines whether the API captures the clip at the moment the viewer requests it or after a delay. If false (default), Twitch captures the clip at the moment the viewer requests it (this is the same clip experience as the Twitch UX). If true, Twitch adds a delay before capturing the clip
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub has_delay: Option<bool>,
}

impl<'a> CreateClipRequest<'a> {
    /// Create a new [`CreateClipRequest`] with the given broadcaster_id
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            has_delay: None,
        }
    }

    /// Sets the has_delay parameter
    pub fn has_delay(mut self, has_delay: bool) -> Self {
        self.has_delay = Some(has_delay);
        self
    }
}

/// Return Value for [Create Clip](super::create_clip)
///
/// [`create-clip`](https://dev.twitch.tv/docs/api/reference#create-clip)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CreatedClip {
    /// ID of the created clip.
    pub id: types::ClipId,
    /// A URL that you can use to edit the clip’s title, identify the part of the clip to publish, and publish the clip.
    pub edit_url: String,
}

impl Request for CreateClipRequest<'_> {
    type Response = Vec<CreatedClip>;

    const PATH: &'static str = "clips";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ClipsEdit];
}

impl RequestGet for CreateClipRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CreateClipRequest::broadcaster_id("44322889");

    let data = br#"
    {
        "data":
        [{
           "id": "FiveWordsForClipSlug",
           "edit_url": "http://clips.twitch.tv/FiveWordsForClipSlug/edit"
        }]
     }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();

    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/clips?broadcaster_id=44322889"
    );

    dbg!(CreateClipRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

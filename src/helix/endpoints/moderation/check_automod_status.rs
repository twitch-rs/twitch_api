#![allow(deprecated_in_future, deprecated)]
//! Determines whether a string message meets the channel’s AutoMod requirements.
//! [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CheckAutoModStatusRequest]
//!
//! To use this endpoint, construct a [`CheckAutoModStatusRequest`] with the [`CheckAutoModStatusRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::moderation::check_automod_status;
//! let request =
//!     check_automod_status::CheckAutoModStatusRequest::broadcaster_id("1234");
//! ```
//!
//! ## Body: [CheckAutoModStatusBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api::helix::moderation::check_automod_status;
//! let body = check_automod_status::CheckAutoModStatusBody::new(
//!     "test1",
//!     "automod please approve this!",
//! );
//! ```
//!
//! ## Response: [CheckAutoModStatus]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, moderation::check_automod_status};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = check_automod_status::CheckAutoModStatusRequest::broadcaster_id("1234");
//! let body =
//!     check_automod_status::CheckAutoModStatusBody::new("test1", "automod please approve this!");
//! let response: Vec<check_automod_status::CheckAutoModStatus> = client.req_post(request, &[&body].as_slice(), &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`CheckAutoModStatusRequest::parse_response(None, &request.get_uri(), response)`](CheckAutoModStatusRequest::parse_response)

use super::*;
use helix::RequestPost;

/// Query Parameters for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CheckAutoModStatusRequest<'a> {
    /// Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> CheckAutoModStatusRequest<'a> {
    /// Check automod status in this broadcasters channel.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
        }
    }
}

/// Body Parameters for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct CheckAutoModStatusBody<'a> {
    /// Developer-generated identifier for mapping messages to results.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub msg_id: Cow<'a, types::MsgIdRef>,
    /// Message text.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub msg_text: Cow<'a, str>,
    /// User ID of the sender.
    #[deprecated(since = "0.7.0", note = "user_id in automod check is no longer read")]
    #[cfg_attr(
        feature = "typed-builder",
        builder(setter(into, strip_option), default)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> CheckAutoModStatusBody<'a> {
    /// Create a new [`CheckAutoModStatusBody`]
    pub fn new(
        msg_id: impl types::IntoCow<'a, types::MsgIdRef> + 'a,
        msg_text: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            msg_id: msg_id.to_cow(),
            msg_text: msg_text.into(),
            user_id: None,
        }
    }
}

impl<'a> helix::HelixRequestBody for [CheckAutoModStatusBody<'a>] {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a [CheckAutoModStatusBody<'a>],
        }

        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CheckAutoModStatus {
    /// The msg_id passed in the body of the POST message. Maps each message to its status.
    pub msg_id: types::MsgId,
    /// Indicates if this message meets AutoMod requirements.
    pub is_permitted: bool,
}

impl Request for CheckAutoModStatusRequest<'_> {
    type Response = Vec<CheckAutoModStatus>;

    const PATH: &'static str = "moderation/enforcements/status";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl<'a> RequestPost for CheckAutoModStatusRequest<'a> {
    type Body = &'a [&'a CheckAutoModStatusBody<'a>];
}

impl<'a> helix::private::SealedSerialize for &'a [&'a CheckAutoModStatusBody<'a>] {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CheckAutoModStatusRequest::broadcaster_id("198704263");

    dbg!(req
        .create_request(
            &[
                &CheckAutoModStatusBody::new("123", "hello world"),
                &CheckAutoModStatusBody::new("393", "automoded word"),
            ],
            "token",
            "clientid"
        )
        .unwrap());

    // From twitch docs
    let data = br#"
{
   "data": [
     {
       "msg_id": "123",
       "is_permitted": true
     },
     {
       "msg_id": "393",
       "is_permitted": false
     }
   ]
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=198704263"
    );

    dbg!(CheckAutoModStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

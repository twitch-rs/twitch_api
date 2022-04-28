//! Determines whether a string message meets the channel’s AutoMod requirements.
//! [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CheckAutoModStatusRequest]
//!
//! To use this endpoint, construct a [`CheckAutoModStatusRequest`] with the [`CheckAutoModStatusRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::moderation::check_automod_status;
//! let request = check_automod_status::CheckAutoModStatusRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Body: [CheckAutoModStatusBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::check_automod_status;
//! let body = check_automod_status::CheckAutoModStatusBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [CheckAutoModStatus]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::check_automod_status};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = check_automod_status::CheckAutoModStatusRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let body = vec![check_automod_status::CheckAutoModStatusBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build()];
//! let response: Vec<check_automod_status::CheckAutoModStatus> = client.req_post(request, body, &token).await?.data;
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
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CheckAutoModStatusRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Body Parameters for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CheckAutoModStatusBody {
    /// Developer-generated identifier for mapping messages to results.
    #[builder(setter(into))]
    pub msg_id: types::MsgId,
    /// Message text.
    #[builder(setter(into))]
    pub msg_text: String,
    /// User ID of the sender.
    #[builder(setter(into))]
    pub user_id: types::UserId,
}

impl CheckAutoModStatusBody {
    /// Create a new [`CheckAutoModStatusBody`]
    pub fn new(msg_id: types::MsgId, msg_text: String, user_id: types::UserId) -> Self {
        Self {
            msg_id,
            msg_text,
            user_id,
        }
    }
}

impl helix::HelixRequestBody for Vec<CheckAutoModStatusBody> {
    fn try_to_body(&self) -> Result<hyper::body::Bytes, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<CheckAutoModStatusBody>,
        }

        serde_json::to_vec(&InnerBody { data: self })
            .map_err(Into::into)
            .map(Into::into)
    }
}

/// Return Values for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CheckAutoModStatus {
    /// The msg_id passed in the body of the POST message. Maps each message to its status.
    pub msg_id: types::MsgId,
    /// Indicates if this message meets AutoMod requirements.
    pub is_permitted: bool,
}

impl Request for CheckAutoModStatusRequest {
    type Response = Vec<CheckAutoModStatus>;

    const PATH: &'static str = "moderation/enforcements/status";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl RequestPost for CheckAutoModStatusRequest {
    type Body = Vec<CheckAutoModStatusBody>;
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CheckAutoModStatusRequest::builder()
        .broadcaster_id("198704263")
        .build();

    let body = vec![
        CheckAutoModStatusBody::new("123".into(), "hello world".to_string(), "1234".into()),
        CheckAutoModStatusBody::new("393".into(), "automoded word".to_string(), "1234".into()),
    ];

    dbg!(req.create_request(body, "token", "clientid").unwrap());

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

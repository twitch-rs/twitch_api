//! Retrieves the active shared chat session for a channel.
//! [`get-shared-chat-session`](https://dev.twitch.tv/docs/api/reference#get-shared-chat-session)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetSharedChatSessionRequest]
//!
//! To use this endpoint, construct a [`GetSharedChatSessionRequest`] with the [`GetSharedChatSessionRequest::broadcaster_id`] method.
//!
//! ```rust
//! use twitch_api::helix::chat::get_shared_chat_session;
//! let request =
//!     get_shared_chat_session::GetSharedChatSessionRequest::broadcaster_id(
//!         "12345",
//!     );
//! ```
//!
//! ## Response: [SharedChatSession]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, chat::get_shared_chat_session};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_shared_chat_session::GetSharedChatSessionRequest::broadcaster_id("12345");
//! let response: Option<helix::chat::SharedChatSession> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetSharedChatSessionRequest::parse_response(None, &request.get_uri(), response)`](GetSharedChatSessionRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Shared Chat Session](super::get_shared_chat_session)
///
/// [`get-shared-chat-session`](https://dev.twitch.tv/docs/api/reference#get-shared-chat-session)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetSharedChatSessionRequest<'a> {
    /// The User ID of the channel broadcaster.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetSharedChatSessionRequest<'a> {
    /// Retrieve the active shared chat session for a channel
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Shared Chat Session](super::get_shared_chat_session)
///
/// [`get-shared-chat-session`](https://dev.twitch.tv/docs/api/reference#get-shared-chat-session)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SharedChatSession {
    /// The unique identifier for the shared chat session.
    pub session_id: types::SharedChatSessionId,
    /// The User ID of the host channel.
    pub host_broadcaster_id: types::UserId,
    /// The list of participants in the session.
    pub participants: Vec<SharedChatParticipant>,
    /// The UTC date and time (in RFC3339 format) for when the session was created.
    pub created_at: types::Timestamp,
    /// The UTC date and time (in RFC3339 format) for when the session was last updated.
    pub updated_at: types::Timestamp,
}

/// A participant in a shared chat session
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SharedChatParticipant {
    /// The User ID of the participant channel.
    pub broadcaster_id: types::UserId,
}

impl Request for GetSharedChatSessionRequest<'_> {
    type Response = Option<SharedChatSession>;

    const PATH: &'static str = "shared_chat/session";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetSharedChatSessionRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let resp = match status {
            http::StatusCode::OK => {
                let resp: helix::InnerResponse<helix::request::ZeroOrOne<SharedChatSession>> =
                    helix::parse_json(response, true).map_err(|e| {
                        helix::HelixRequestGetError::DeserializeError(
                            response.to_string(),
                            e,
                            uri.clone(),
                            status,
                        )
                    })?;
                resp.data.0
            }
            _ => {
                return Err(helix::HelixRequestGetError::InvalidResponse {
                    reason: "unexpected status code",
                    response: response.to_string(),
                    status,
                    uri: uri.clone(),
                })
            }
        };
        Ok(helix::Response::with_data(resp, request))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetSharedChatSessionRequest::broadcaster_id("198704263");

    // From twitch docs (modified `updated_at`)
    let data = br#"
        {
          "data": [
            {
              "session_id": "359bce59-fa4e-41a5-bd6f-9bc0c8360485",
              "host_broadcaster_id": "198704263",
              "participants": [{
                  "broadcaster_id": "198704263"
              }, {
                  "broadcaster_id": "487263401"
              }],
              "created_at": "2024-09-29T19:45:37Z",
              "updated_at": "2024-09-29T19:50:01Z"
            }
          ]
        }
        "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263"
    );

    let res = GetSharedChatSessionRequest::parse_response(Some(req), &uri, http_response).unwrap();
    let res = res.data.unwrap();
    assert_eq!(
        res.session_id.as_str(),
        "359bce59-fa4e-41a5-bd6f-9bc0c8360485"
    );
    assert_eq!(res.host_broadcaster_id.as_str(), "198704263");
    assert_eq!(res.participants.len(), 2);
    assert_eq!(res.participants[0].broadcaster_id.as_str(), "198704263");
    assert_eq!(res.participants[1].broadcaster_id.as_str(), "487263401");
    assert_eq!(res.created_at.as_str(), "2024-09-29T19:45:37Z");
    assert_eq!(res.updated_at.as_str(), "2024-09-29T19:50:01Z");
}

#[cfg(test)]
#[test]
fn test_request_empty() {
    use helix::*;
    let req = GetSharedChatSessionRequest::broadcaster_id("198704263");

    let data = br#"{ "data": [] }"#.to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/shared_chat/session?broadcaster_id=198704263"
    );

    let res = GetSharedChatSessionRequest::parse_response(Some(req), &uri, http_response).unwrap();
    assert!(res.data.is_none());
}

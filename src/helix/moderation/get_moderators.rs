//! Returns all moderators in a channel.
//! [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetModeratorsRequest]
//!
//! To use this endpoint, construct a [`GetModeratorsRequest`] with the [`GetModeratorsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::moderation::get_moderators;
//! let request = get_moderators::GetModeratorsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [Moderator]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::get_moderators};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_moderators::GetModeratorsRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_moderators::Moderator> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
use super::*;

// FIXME: Twitch Docs is borked here, mentions query param user_id
// user_id	no	string	Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
// Maximum: 100
/// Query Parameters for [Get Moderators](super::get_moderators)
///
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetModeratorsRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
}

/// Return Values for [Get Moderators](super::get_moderators)
///
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Moderator {
    /// User ID of moderator
    ///
    /// Twitch says: `User ID of a user who has been banned.` but this seems wrong.
    pub user_id: types::UserId,
    /// Display name of moderator
    pub user_name: types::DisplayName,
}

impl helix::Request for GetModeratorsRequest {
    type Response = Vec<Moderator>;

    const PATH: &'static str = "moderation/moderators";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl helix::RequestGet for GetModeratorsRequest {}

impl helix::Paginated for GetModeratorsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetModeratorsRequest::builder()
        .broadcaster_id("198704263".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [
        {
            "user_id": "424596340",
            "user_name": "quotrok"
        },
        {
            "user_id": "424596340",
            "user_name": "quotrok"
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
    }
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=198704263"
    );

    dbg!(req.parse_response(&uri, http_response).unwrap());
}

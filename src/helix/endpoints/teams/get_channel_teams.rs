//! Gets information for a specific Twitch Team.
//! [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
//!
//! ## Request: [GetChannelTeamsRequest]
//!
//! To use this endpoint, construct a [`GetChannelTeamsRequest`] with the [`GetChannelTeamsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::teams::get_channel_teams;
//! let request =
//!     get_channel_teams::GetChannelTeamsRequest::broadcaster_id("1337");
//! ```
//!
//! ## Response: [BroadcasterTeam]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, teams::get_channel_teams};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_teams::GetChannelTeamsRequest::broadcaster_id("1337");
//! let response: Vec<get_channel_teams::BroadcasterTeam> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetChannelTeamsRequest::parse_response(None, &request.get_uri(), response)`](GetChannelTeamsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Channel Teams](super::get_channel_teams)
///
/// [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetChannelTeamsRequest<'a> {
    /// Team ID.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetChannelTeamsRequest<'a> {
    /// Get the team of this specific broadcaster
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Channel Teams](super::get_channel_teams)
///
/// [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct BroadcasterTeam {
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Team information
    #[serde(flatten)]
    pub team: TeamInformation,
}

impl Request for GetChannelTeamsRequest<'_> {
    type PaginationData = ();
    type Response = Vec<BroadcasterTeam>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "teams/channel";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetChannelTeamsRequest<'_> {
    fn parse_inner_response(
        _request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<Option<_>> =
            crate::parse_json(response, true).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            response.data.unwrap_or_default(),
            (),
            response.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelTeamsRequest::broadcaster_id("44322889");

    // From twitch docs
    let data = br#"
{
    "data": [
    {
        "broadcaster_id": "96909659",
        "broadcaster_name": "CSharpFritz",
        "broadcaster_login": "csharpfritz",
        "background_image_url": null,
        "banner": null,
        "created_at": "2019-02-11T12:09:22Z",
        "updated_at": "2020-11-18T15:56:41Z",
        "info": "<p>An outgoing and enthusiastic group of friendly channels that write code, teach about technology, and promote the technical community.</p>",
        "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/team-livecoders-team_logo_image-bf1d9a87ca81432687de60e24ad9593d-600x600.png",
        "team_name": "livecoders",
        "team_display_name": "Live Coders",
        "id": "6358"
    }
    ]
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/teams/channel?broadcaster_id=44322889"
    );

    dbg!(GetChannelTeamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_null() {
    use helix::*;
    let req = GetChannelTeamsRequest::broadcaster_id("1234");

    let data = br#"{"data": null}"#.to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/teams/channel?broadcaster_id=1234"
    );

    dbg!(GetChannelTeamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

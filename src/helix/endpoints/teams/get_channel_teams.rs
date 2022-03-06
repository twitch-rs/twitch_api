//! Gets information for a specific Twitch Team.
//! [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
//!
//! ## Request: [GetChannelTeamsRequest]
//!
//! To use this endpoint, construct a [`GetChannelTeamsRequest`] with the [`GetChannelTeamsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api2::helix::teams::get_channel_teams;
//! let request = get_channel_teams::GetChannelTeamsRequest::builder()
//!     .broadcaster_id("1337")
//!     .build();
//! ```
//!
//! ## Response: [BroadcasterTeam]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, teams::get_channel_teams};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_channel_teams::GetChannelTeamsRequest::builder()
//!     .broadcaster_id("1337")
//!     .build();
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
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetChannelTeamsRequest {
    /// Team ID.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Return Values for [Get Channel Teams](super::get_channel_teams)
///
/// [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
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

impl Request for GetChannelTeamsRequest {
    type Response = Vec<BroadcasterTeam>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "teams/channel";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetChannelTeamsRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetChannelTeamsRequest::builder()
        .broadcaster_id("44322889".to_string())
        .build();

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

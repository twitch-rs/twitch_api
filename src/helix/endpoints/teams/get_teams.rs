//! Gets information for a specific Twitch Team.
//! [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
//!
//! ## Request: [GetTeamsRequest]
//!
//! To use this endpoint, construct a [`GetTeamsRequest`] with the [`GetTeamsRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::teams::get_teams;
//! let request = get_teams::GetTeamsRequest::builder()
//!     .name("coolteam".to_string())
//!     .build();
//! ```
//!
//! ## Response: [Team]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, teams::get_teams};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_teams::GetTeamsRequest::builder()
//!     .name("coolteam".to_string())
//!     .build();
//! let response: Vec<get_teams::Team> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetTeamsRequest::parse_response(None, &request.get_uri(), response)`](GetTeamsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Teams](super::get_teams)
///
/// [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
#[derive(PartialEq, Eq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetTeamsRequest {
    /// Team ID.
    #[builder(default, setter(into))]
    pub id: Option<types::TeamId>,
    /// Team name.
    #[builder(default, setter(into))]
    pub name: Option<String>,
}

/// Return Values for [Get Teams](super::get_teams)
///
/// [`get-teams`](https://dev.twitch.tv/docs/api/reference#get-teams)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Team {
    /// Users in the team.
    pub users: Vec<types::User>,
    /// Information about the team.
    #[serde(flatten)]
    pub team: TeamInformation,
}

impl Request for GetTeamsRequest {
    type Response = Vec<Team>;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadEmail];
    const PATH: &'static str = "teams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetTeamsRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetTeamsRequest::builder().id(Some("6358".into())).build();

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "users": [
              {
                "user_id": "278217731",
                "user_name": "mastermndio",
                "user_login": "mastermndio"
              },
              {
                "user_id": "41284990",
                "user_name": "jenninexus",
                "user_login": "jenninexus"
              }
            ],
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
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/teams?id=6358");

    dbg!(GetTeamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

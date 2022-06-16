//! Gets games sorted by number of current viewers on Twitch, most popular first.
//! [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetTopGamesRequest]
//!
//! To use this endpoint, construct a [`GetTopGamesRequest`] with the [`GetTopGamesRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::games::get_top_games;
//! let request = get_top_games::GetTopGamesRequest::builder()
//!     .first(100)
//!     .build();
//! ```
//!
//! ## Response: [Game](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, games::get_top_games};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_top_games::GetTopGamesRequest::builder()
//!     .build();
//! let response: Vec<get_top_games::Game> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetTopGamesRequest::parse_response(None, &request.get_uri(), response)`](GetTopGamesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Top Games](super::get_games)
///
/// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetTopGamesRequest {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default, setter(into))]
    pub after: Option<helix::Cursor>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default, setter(into))]
    pub before: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get Top Games](super::get_games)
///
/// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
pub type Game = types::TwitchCategory;

impl Request for GetTopGamesRequest {
    type Response = Vec<Game>;

    const PATH: &'static str = "games/top";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetTopGamesRequest {}

impl helix::Paginated for GetTopGamesRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetTopGamesRequest::builder().build();

    // From twitch docs
    let data = br#"
{
    "data": [
      {
        "id": "493057",
        "name": "PLAYERUNKNOWN'S BATTLEGROUNDS",
        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/PLAYERUNKNOWN%27S%20BATTLEGROUNDS-{width}x{height}.jpg"
      },
      {
        "id": "493057",
        "name": "PLAYERUNKNOWN'S BATTLEGROUNDS",
        "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/PLAYERUNKNOWN%27S%20BATTLEGROUNDS-{width}x{height}.jpg"
      }
    ],
    "pagination":{"cursor":"eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ=="}
}
"#
        .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/games/top?");

    dbg!(GetTopGamesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

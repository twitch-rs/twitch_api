//! Gets game information by game ID or name.
//! [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetGamesRequest]
//!
//! To use this endpoint, construct a [`GetGamesRequest`] with the [`GetGamesRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::games::get_games;
//! let request = get_games::GetGamesRequest::builder()
//!     .id(vec!["4321".into()])
//!     .build();
//! ```
//!
//! ## Response: [Game](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, games::get_games};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_games::GetGamesRequest::builder()
//!     .id(vec!["4321".into()])
//!     .build();
//! let response: Vec<get_games::Game> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetGamesRequest::parse_response(None, &request.get_uri(), response)`](GetGamesRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Games](super::get_games)
///
/// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetGamesRequest {
    /// Game ID. At most 100 id values can be specified.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub id: Vec<types::CategoryId>,
    /// Game name. The name must be an exact match. For instance, “Pokemon” will not return a list of Pokemon games; instead, query the specific Pokemon game(s) in which you are interested. At most 100 name values can be specified.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub name: Vec<String>,
}

/// Return Values for [Get Games](super::get_games)
///
/// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
pub type Game = types::TwitchCategory;

impl Request for GetGamesRequest {
    type Response = Vec<Game>;

    const PATH: &'static str = "games";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for GetGamesRequest {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetGamesRequest::builder().id(vec!["493057".into()]).build();

    // From twitch docs
    let data = br#"
{
    "data": [
        {
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-52x72.jpg",
            "id": "33214",
            "name": "Fortnite"
        },
        {
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-52x72.jpg",
            "id": "33214",
            "name": "Fortnite"
        }
    ],
    "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN"
  }
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/games?id=493057"
    );

    dbg!(GetGamesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

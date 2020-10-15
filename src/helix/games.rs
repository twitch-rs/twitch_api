//! Endpoints regarding games

#[doc(inline)]
pub use get_games::GetGamesRequest;

#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// A game as defined by Twitch
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct TwitchGame {
    ///Template URL for the game’s box art.
    box_art_url: String,
    ///Game ID.
    id: types::GameId,
    ///Game name.
    name: String,
}

/// Gets game information by game ID or name.
/// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
pub mod get_games {
    use super::*;

    /// Query Parameters for [Get Games](super::get_games)
    ///
    /// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetGamesRequest {
        /// Game ID. At most 100 id values can be specified.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        id: Vec<types::GameId>,
        /// Game name. The name must be an exact match. For instance, “Pokemon” will not return a list of Pokemon games; instead, query the specific Pokemon game(s) in which you are interested. At most 100 name values can be specified.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        name: Vec<String>,
    }

    /// Return Values for [Get Games](super::get_games)
    ///
    /// [`get-games`](https://dev.twitch.tv/docs/api/reference#get-games)
    pub type Game = helix::games::TwitchGame;

    impl helix::Request for GetGamesRequest {
        type Response = Vec<Game>;

        const PATH: &'static str = "games";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetGamesRequest {}

    #[test]
    fn parse_response() {
        use helix::*;
        let req = GetGamesRequest::builder()
            .id(vec!["493057".to_string()])
            .build();

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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets games sorted by number of current viewers on Twitch, most popular first.
/// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
pub mod get_top_games {
    use super::*;

    /// Query Parameters for [Get Games](super::get_games)
    ///
    /// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetTopGamesRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub before: Option<String>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[builder(setter(strip_option))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
    }

    /// Return Values for [Get Games](super::get_games)
    ///
    /// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
    pub type Game = helix::games::TwitchGame;

    impl helix::Request for GetTopGamesRequest {
        type Response = Vec<Game>;

        const PATH: &'static str = "games/top";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetTopGamesRequest {}

    impl helix::Paginated for GetTopGamesRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn parse_response() {
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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

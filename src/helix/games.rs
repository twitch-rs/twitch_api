//! Endpoints regarding games

#[doc(inline)]
pub use get_games::GetGamesRequest;

#[doc(inline)]
pub use get_top_games::GetTopGamesRequest;

use crate::helix;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// A game as defined by Twitch
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[non_exhaustive]
pub struct TwitchGame {
    ///Template URL for the game’s box art.
    box_art_url: String,
    ///Game ID.
    id: String,
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
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetGamesRequest {
        /// Game ID. At most 100 id values can be specified.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        id: Vec<String>,
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
        type Response = Game;

        const PATH: &'static str = "games";
        #[cfg(feature = "client")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetGamesRequest {}
}

/// Gets games sorted by number of current viewers on Twitch, most popular first.
/// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
pub mod get_top_games {
    use super::*;

    /// Query Parameters for [Get Games](super::get_games)
    ///
    /// [`get-top-games`](https://dev.twitch.tv/docs/api/reference#get-top-games)
    #[derive(PartialEq, TypedBuilder, Deserialize, Serialize, Clone, Debug)]
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
        type Response = Game;

        const PATH: &'static str = "games/top";
        #[cfg(feature = "client")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetTopGamesRequest {}

    impl helix::Paginated for GetTopGamesRequest {
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }
}

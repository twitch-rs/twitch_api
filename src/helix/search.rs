//! Endpoints regarding search
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, search::SearchCategoriesRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = SearchCategoriesRequest::builder()
//!     .query("Pokémon")
//!     .build();
//!
//! println!("{:?}", &client.req_get(req, &token).await?.data);
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use search_categories::{Categories, SearchCategoriesRequest};
#[doc(inline)]
pub use search_channels::{Channels, SearchChannelsRequest};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Returns a list of games or categories that match the query via name either entirely or partially.
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
pub mod search_categories {
    use super::*;

    // FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
    /// Query Parameters for [Search Categories](super::search_categories)
    ///
    /// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct SearchCategoriesRequest {
        /// URL encoded search query
        #[builder(setter(into))]
        pub query: String,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub before: Option<helix::Cursor>,
    }

    /// Return Values for [Search Categories](super::search_categories)
    ///
    /// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
    pub type Categories = types::TwitchCategory;

    impl helix::Request for SearchCategoriesRequest {
        type Response = Vec<Categories>;

        const PATH: &'static str = "search/categories";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for SearchCategoriesRequest {}

    impl helix::Paginated for SearchCategoriesRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = SearchCategoriesRequest::builder().query("fort").build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
            "id": "33214",
            "name": "Fortnite",
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-{width}x{height}.jpg"
        },
        {
            "id": "33214",
            "name": "Fortnite",
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-{width}x{height}.jpg"
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
            "https://api.twitch.tv/helix/search/categories?query=fort"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Returns a list of channels (users who have streamed within the past 6 months) that match the query via channel name or description either entirely or partially.
/// [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
pub mod search_channels {
    use super::*;

    /// Query Parameters for [Search Channels](super::search_channels)
    ///
    /// [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct SearchChannelsRequest {
        /// URL encoded search query
        #[builder(setter(into))]
        pub query: String,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100 Default: 20
        #[builder(default)] // FIXME: No setter because int
        pub first: Option<usize>,
        /// Filter results for live streams only. Default: false
        #[builder(default, setter(into))]
        pub live_only: Option<bool>,
    }

    /// Return Values for [Search Channels](super::search_channels)
    ///
    /// [`search-channels`](https://dev.twitch.tv/docs/api/reference#search-channels)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct Channels {
        /// ID of the game being played on the stream
        pub game_id: types::CategoryId,
        /// Channel ID
        pub id: types::UserId,
        /// Display name corresponding to user_id
        pub display_name: types::DisplayName,
        /// Channel language (Broadcaster Language field from the [Channels service][crate::helix::channels])
        pub broadcaster_language: String,
        /// channel title
        pub title: String,
        /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image.
        pub thumbnail_url: String,
        /// Live status
        pub is_live: bool,
        /// UTC timestamp. (live only)
        pub started_at: types::Timestamp,
        // FIXME: Twitch doc say tag_ids
        /// Shows tag IDs that apply to the stream (live only).See https://www.twitch.tv/directory/all/tags for tag types
        pub tags_ids: Vec<types::TagId>,
    }

    impl helix::Request for SearchChannelsRequest {
        type Response = Vec<Channels>;

        const PATH: &'static str = "search/categories";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for SearchChannelsRequest {}

    impl helix::Paginated for SearchChannelsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = SearchChannelsRequest::builder().query("fort").build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
            "broadcaster_language": "en",
            "display_name": "a_seagull",
            "game_id": "506442",
            "id": "19070311",
            "is_live": true,
            "tags_ids": [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ],
            "thumbnail_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/a_seagull-profile_image-4d2d235688c7dc66-300x300.png",
            "title": "a_seagull",
            "started_at": "2020-03-18T17:56:00Z"
        }
    ],
    "pagination": {}
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/search/categories?query=fort"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

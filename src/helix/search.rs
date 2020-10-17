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
pub use search_categories::{Search, SearchCategoriesRequest};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Returns a list of games or categories that match the query via name either entirely or partially.
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
pub mod search_categories {
    use super::*;

    // FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
    /// Query Parameters for [Search Category](super::search_categories)
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

    /// Return Values for [Search Category](super::search_categories)
    ///
    /// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
    pub type Search = types::TwitchCategory;

    impl helix::Request for SearchCategoriesRequest {
        type Response = Vec<Search>;

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
        let req = SearchCategoriesRequest::builder()
            .query("fort")
            .build();

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

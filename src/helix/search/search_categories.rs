//! Returns a list of games or categories that match the query via name either entirely or partially.
//! [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SearchCategoriesRequest]
//!
//! To use this endpoint, construct a [`SearchCategoriesRequest`] with the [`SearchCategoriesRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::search::search_categories;
//! let request = search_categories::SearchCategoriesRequest::builder()
//!     .query("hello")
//!     .build();
//! ```
//!
//! ## Response: [Category](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, search::search_categories};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = search_categories::SearchCategoriesRequest::builder()
//!     .query("hello")
//!     .build();
//! let response: Vec<search_categories::Category> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`SearchCategoriesRequest::parse_response(None, &request.get_uri(), response)`](SearchCategoriesRequest::parse_response)

use super::*;
use helix::RequestGet;

// FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
/// Query Parameters for [Search Categories](super::search_categories)
///
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct SearchCategoriesRequest {
    /// URI encoded search query
    #[builder(setter(into))]
    pub query: String,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub after: Option<helix::Cursor>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[builder(default)]
    pub before: Option<helix::Cursor>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[builder(setter(into), default)]
    pub first: Option<String>,
}

/// Return Values for [Search Categories](super::search_categories)
///
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
pub type Category = types::TwitchCategory;

impl Request for SearchCategoriesRequest {
    type Response = Vec<Category>;

    const PATH: &'static str = "search/categories";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[];
}

impl RequestGet for SearchCategoriesRequest {}

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

    dbg!(SearchCategoriesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

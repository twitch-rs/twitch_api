//! Endpoints regarding streams
//!
//! # Examples
//!
//! ```rust,no_run
//! # use twitch_api2::helix::{HelixClient, streams::GetStreamsRequest};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
//! let client = HelixClient::new();
//! # let _: &HelixClient<twitch_api2::DummyHttpClient> = &client;
//! let req = GetStreamsRequest::builder()
//!     .user_login(vec!["justinfan1337".to_string()])
//!     .build();
//!
//! // If this doesn't return a result, that would mean the stream is not live.
//! println!("{:?}", &client.req_get(req, &token).await?.data.get(0));
//! # Ok(())
//! # }
//! ```
#[doc(inline)]
pub use get_streams::{GetStreamsRequest, Stream};

#[doc(inline)]
pub use get_stream_tags::{GetStreamTagsRequest, Tag};

use crate::{helix, types};
use serde::{Deserialize, Serialize};

/// Gotten from [`Stream.type_`](get_streams::Stream#structfield.type_)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum StreamType {
    /// Stream is live.
    #[serde(rename = "live")]
    Live,

    // Got error from endpoint
    //Error, TODO: Should this be here?

    //
    /// Stream not live
    ///
    /// # Notes
    /// This is never returned from twitch endpoints. To get this
    /// Just do a [`GetStreamsRequest`] and if there is no response for your user_id/user_login, you can be
    /// sure that the channel is not live
    #[serde(other)]
    NotLive,
}

impl StreamType {
    /// Check if the stream is live or not
    pub fn is_live(&self) -> bool { matches!(self, StreamType::Live) }
}
/// Gets information about active streams.
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
///
/// # Accessing the endpoint
///
/// ## Request: [GetStreamsRequest]
///
/// To use this endpoint, construct a [`GetStreamsRequest`] with the [`GetStreamsRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::streams::get_streams;
/// let request = get_streams::GetStreamsRequest::builder()
///     .user_login(vec!["justintvfan".to_string()])
///     .build();
/// ```
///
/// ## Response: [Stream]
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, streams::get_streams};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_streams::GetStreamsRequest::builder()
///     .user_login(vec!["justintvfan".to_string()])
///     .build();
/// let response: Vec<get_streams::Stream> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_streams {
    use super::*;

    /// Query Parameters for [Get Streams](super::get_streams)
    ///
    /// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetStreamsRequest {
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub after: Option<helix::Cursor>,
        /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        pub before: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default)]
        #[builder(setter(strip_option))]
        pub first: Option<usize>,
        /// Returns streams broadcasting a specified game ID. You can specify up to 10 IDs.
        #[builder(default)]
        pub game_id: Vec<types::CategoryId>,
        /// Stream language. You can specify up to 100 languages.
        #[builder(default)]
        pub language: Option<String>,
        /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
        #[builder(default, setter(into))]
        pub user_id: Vec<types::UserId>,
        /// Returns streams broadcast by one or more specified user login names. You can specify up to 100 names.
        #[builder(default)]
        pub user_login: Vec<types::UserName>,
    }

    /// Return Values for [Get Streams](super::get_streams)
    ///
    /// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
    #[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
    #[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
    #[non_exhaustive]
    pub struct Stream {
        /// ID of the game being played on the stream.
        pub game_id: types::CategoryId,
        /// Stream ID.
        pub id: String,
        /// Stream language.
        pub language: String,
        /// UTC timestamp.
        pub started_at: types::Timestamp,
        /// Shows tag IDs that apply to the stream.
        pub tag_ids: Vec<types::TagId>,
        /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image
        pub thumbnail_url: String,
        /// Stream title.
        pub title: String,
        /// Stream type: "live" or "" (in case of error).
        #[serde(rename = "type")]
        pub type_: StreamType,
        /// ID of the user who is streaming.
        pub user_id: types::UserId,
        /// Display name corresponding to user_id.
        pub user_name: types::UserName,
        /// Number of viewers watching the stream at the time of the query.
        pub viewer_count: usize,
    }

    impl helix::Request for GetStreamsRequest {
        type Response = Vec<Stream>;

        const PATH: &'static str = "streams";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetStreamsRequest {}

    impl helix::Paginated for GetStreamsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetStreamsRequest::builder().build();

        // From twitch docs. example 1 in https://dev.twitch.tv/docs/api/reference#get-streams is malformed
        let data = br#"
{
    "data": [
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "game_id": "417752",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ]
        },
        {
            "id": "26007494656",
            "user_id": "23161357",
            "user_name": "LIRIK",
            "game_id": "417752",
            "type": "live",
            "title": "Hey Guys, It's Monday - Twitter: @Lirik",
            "viewer_count": 32575,
            "started_at": "2017-08-14T16:08:32Z",
            "language": "en",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_lirik-{width}x{height}.jpg",
            "tag_ids":  [
                "6ea6bca4-4712-4ab9-a906-e3336a9d8039"
            ]
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6MjB9fQ=="
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/streams?");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets the list of tags for a specified stream (channel).
/// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
///
/// # Accessing the endpoint
///
/// ## Request: [GetStreamTagsRequest]
///
/// To use this endpoint, construct a [`GetStreamTagsRequest`] with the [`GetStreamTagsRequest::builder()`] method.
///
/// ```rust, no_run
/// use twitch_api2::helix::streams::get_stream_tags;
/// let request = get_stream_tags::GetStreamTagsRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// ```
///
/// ## Response: [Tag](helix::tags::TwitchTag)
///
/// Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
///
/// ```rust, no_run
/// use twitch_api2::helix::{self, streams::get_stream_tags};
/// # use twitch_api2::client;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
/// # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
/// # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None).await?;
/// let request = get_stream_tags::GetStreamTagsRequest::builder()
///     .broadcaster_id("1234")
///     .build();
/// let response: Vec<get_stream_tags::Tag> = client.req_get(request, &token).await?.data;
/// # Ok(())
/// # }
/// ```
///
/// You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
/// and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())
pub mod get_stream_tags {
    use super::*;

    /// Query Parameters for [Get Stream Tags](super::get_stream_tags)
    ///
    /// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetStreamTagsRequest {
        // FIXME: twitch docs sucks
        /// ID of the stream whose tags are going to be fetched
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
    }

    /// Return Values for [Get Stream Tags](super::get_stream_tags)
    ///
    /// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
    pub type Tag = helix::tags::TwitchTag;

    impl helix::Request for GetStreamTagsRequest {
        type Response = Vec<Tag>;

        const PATH: &'static str = "streams/tags";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[];
    }

    impl helix::RequestGet for GetStreamTagsRequest {}

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetStreamTagsRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = "\
{\n\
    \"data\": [\n\
        {\n\
            \"tag_id\": \"621fb5bf-5498-4d8f-b4ac-db4d40d401bf\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"Завършване без продължаване\",\n\
                \"cs-cz\": \"Na jeden z&aacute;tah\",\n\
                \"da-dk\": \"Continue klaret\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"bg-bg\": \"За потоци с акцент върху завършване на аркадна игра с монети, в която не се използва продължаване\",\n\
                \"cs-cz\": \"Pro vys&iacute;l&aacute;n&iacute; s důrazem na plněn&iacute; mincov&yacute;ch ark&aacute;dov&yacute;ch her bez použit&iacute; pokračov&aacute;n&iacute;.\",\n\
                \"da-dk\": \"Til streams med v&aelig;gt p&aring; at gennemf&oslash;re et arkadespil uden at bruge continues\"\n\
            }\n\
        },\n\
        {\n\
            \"tag_id\": \"79977fb9-f106-4a87-a386-f1b0f99783dd\",\n\
            \"is_auto\": false,\n\
            \"localization_names\": {\n\
                \"bg-bg\": \"PvE\",\n\
                \"cs-cz\": \"PvE\"\n\
            },\n\
            \"localization_descriptions\": {\n\
                \"bg-bg\": \"За потоци с акцент върху PVE геймплей\",\n\
                \"cs-cz\": \"Pro vys&iacute;l&aacute;n&iacute; s důrazem na hratelnost \\\"hr&aacute;č vs. prostřed&iacute;\\\".\",\n\
                \"da-dk\": \"Til streams med v&aelig;gt p&aring; spil, hvor det er spilleren mod omgivelserne.\"\n\
            }\n\
        }\n\
    ]\n\
}\n\
"
        .as_bytes().to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(
            uri.to_string(),
            "https://api.twitch.tv/helix/streams/tags?broadcaster_id=198704263"
        );

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

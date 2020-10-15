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

use crate::helix;
use serde::{Deserialize, Serialize};

/// Gotten from [Stream.type_](get_streams::Stream#structfield.type_)
#[derive(PartialEq, Deserialize, Debug, Clone)]
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
    /// Just do a [GetStreamsRequest] and if there is no response for your user_id/user_login, you can be
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
        /// Returns streams broadcasting a specified game ID. You can specify up to 10 IDs.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub game_id: Vec<String>,
        /// Stream language. You can specify up to 100 languages.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub language: Option<String>,
        /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<String>,
        /// Returns streams broadcast by one or more specified user login names. You can specify up to 100 names.
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_login: Vec<String>,
    }

    /// Return Values for [Get Streams](super::get_streams)
    ///
    /// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Stream {
        /// ID of the game being played on the stream.
        pub game_id: String,
        /// Stream ID.
        pub id: String,
        /// Stream language.
        pub language: String,
        /// UTC timestamp.
        pub started_at: String,
        /// Shows tag IDs that apply to the stream.
        pub tag_ids: Vec<String>,
        /// Thumbnail URL of the stream. All image URLs have variable width and height. You can replace {width} and {height} with any values to get that size image
        pub thumbnail_url: String,
        /// Stream title.
        pub title: String,
        /// Stream type: "live" or "" (in case of error).
        #[serde(rename = "type")]
        pub type_: StreamType,
        /// ID of the user who is streaming.
        pub user_id: String,
        /// Display name corresponding to user_id.
        pub user_name: String,
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
        fn set_pagination(&mut self, cursor: helix::Cursor) { self.after = Some(cursor) }
    }

    #[test]
    fn parse_response() {
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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Gets information about active streams.
/// [`get-stream-tags`](https://dev.twitch.tv/docs/api/reference#get-stream-tags)
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
        pub broadcaster_id: String,
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
    fn parse_response() {
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

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

//! Gets information about active streams.
//! [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
//!
//! # Accessing the endpoint
//!
//! ## Notes
//!
//! See also [`HelixClient::get_streams_from_logins`](crate::helix::HelixClient::get_streams_from_logins) and
//! [`HelixClient::get_streams_from_ids`](crate::helix::HelixClient::get_streams_from_ids)
//!
//!
//! ## Request: [GetStreamsRequest]
//!
//! To use this endpoint, construct a [`GetStreamsRequest`] with the [`GetStreamsRequest::user_ids()`], [`GetStreamsRequest::user_logins()`] or [`GetStreamsRequest::game_ids()`] method.
//!
//! See also
//!
//! ```rust
//! use twitch_api::helix::streams::get_streams;
//! let request = get_streams::GetStreamsRequest::user_logins(
//!     &["justintvfan".into()][..],
//! );
//! ```
//!
//! ## Response: [Stream]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, streams::get_streams};
//! # use twitch_api::{client, types};
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let client: helix::HelixClient<'static, client::DummyHttpClient> =
//!     helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let logins: &[&types::UserNameRef] = &["justintvfan".into()];
//! let request = get_streams::GetStreamsRequest::user_logins(logins);
//! let response: Vec<get_streams::Stream> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetStreamsRequest::parse_response(None, &request.get_uri(), response)`](GetStreamsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Streams](super::get_streams)
///
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetStreamsRequest<'a> {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub before: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// Returns streams broadcasting a specified game ID. You can specify up to 10 IDs.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub game_id: Cow<'a, [&'a types::CategoryIdRef]>,
    /// Stream language. You can specify up to 100 languages.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub language: Option<Cow<'a, str>>,
    /// Returns streams broadcast by one or more specified user IDs. You can specify up to 100 IDs.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_id: Cow<'a, [&'a types::UserIdRef]>,
    /// Returns streams broadcast by one or more specified user login names. You can specify up to 100 names.
    #[cfg_attr(
        feature = "typed-builder",
        builder(default_code = "Cow::Borrowed(&[])", setter(into))
    )]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub user_login: Cow<'a, [&'a types::UserNameRef]>,
}

impl<'a> GetStreamsRequest<'a> {
    /// Return streams for specified user ids
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::streams::GetStreamsRequest;
    /// let ids: &[&twitch_types::UserIdRef] = &["1234".into()];
    /// let request = GetStreamsRequest::user_ids(ids);
    /// ```
    pub fn user_ids(user_ids: impl Into<Cow<'a, [&'a types::UserIdRef]>>) -> Self {
        Self {
            user_id: user_ids.into(),
            ..Self::default()
        }
    }

    /// Return streams for specified users by [nickname](types::UserName)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_api::helix::streams::GetStreamsRequest;
    /// let ids: &[&twitch_types::UserNameRef] = &["justintvfan".into()];
    /// let request = GetStreamsRequest::user_logins(ids);
    /// ```
    pub fn user_logins(user_logins: impl Into<Cow<'a, [&'a types::UserNameRef]>>) -> Self {
        Self {
            user_login: user_logins.into(),
            ..Self::default()
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }

    /// Return streams with these [Id](types::CategoryId)s
    pub fn game_ids(game_ids: impl Into<Cow<'a, [&'a types::CategoryIdRef]>>) -> Self {
        Self {
            game_id: game_ids.into(),
            ..Self::default()
        }
    }
}

impl Default for GetStreamsRequest<'_> {
    fn default() -> Self {
        Self {
            after: None,
            before: None,
            first: None,
            game_id: Cow::Borrowed(&[]),
            language: None,
            user_id: Cow::Borrowed(&[]),
            user_login: Cow::Borrowed(&[]),
        }
    }
}

/// Return Values for [Get Streams](super::get_streams)
///
/// [`get-streams`](https://dev.twitch.tv/docs/api/reference#get-streams)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Stream {
    /// ID of the game being played on the stream.
    pub game_id: types::CategoryId,
    /// Name of the game being played.
    pub game_name: String,
    /// Stream ID.
    pub id: types::StreamId,
    /// Stream language.
    pub language: String,
    /// Indicates if the broadcaster has specified their channel contains mature content that may be inappropriate for younger audiences.
    pub is_mature: bool,
    /// UTC timestamp.
    pub started_at: types::Timestamp,
    /// Shows tag IDs that apply to the stream.
    #[serde(deserialize_with = "helix::deserialize_default_from_null", default)]
    #[deprecated(note = "use `tags` instead")]
    pub tag_ids: Vec<types::TagId>,
    /// The tags applied to the stream.
    #[serde(deserialize_with = "helix::deserialize_default_from_null")]
    pub tags: Vec<String>,
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
    pub user_name: types::DisplayName,
    /// Login of the user who is streaming.
    pub user_login: types::UserName,
    /// Number of viewers watching the stream at the time of the query.
    pub viewer_count: usize,
}

impl Request for GetStreamsRequest<'_> {
    type Response = Vec<Stream>;

    const PATH: &'static str = "streams";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetStreamsRequest<'_> {}

impl helix::Paginated for GetStreamsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetStreamsRequest::default();

    // From twitch docs, kinda. example 1 in https://dev.twitch.tv/docs/api/reference#get-streams is malformed
    let data = r#"
    {
        "data": [
          {
            "id": "123456789",
            "user_id": "98765",
            "user_login": "sandysanderman",
            "user_name": "SandySanderman",
            "game_id": "494131",
            "game_name": "Little Nightmares",
            "type": "live",
            "title": "hablamos y le damos a Little Nightmares 1",
            "tags": ["Español"],
            "viewer_count": 78365,
            "started_at": "2021-03-10T15:04:21Z",
            "language": "es",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_auronplay-{width}x{height}.jpg",
            "tag_ids": [],
            "is_mature": false
          },
          {
            "id": "123456789",
            "user_id": "98765",
            "user_login": "sandysanderman",
            "user_name": "SandySanderman",
            "game_id": "494131",
            "game_name": "Little Nightmares",
            "type": "live",
            "title": "hablamos y le damos a Little Nightmares 1",
            "tags": ["Español"],
            "viewer_count": 78365,
            "started_at": "2021-03-10T15:04:21Z",
            "language": "es",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_auronplay-{width}x{height}.jpg",
            "tag_ids": [],
            "is_mature": false
          }
        ],
        "pagination": {
          "cursor": "eyJiIjp7IkN1cnNvciI6ImV5SnpJam8zT0RNMk5TNDBORFF4TlRjMU1UY3hOU3dpWkNJNlptRnNjMlVzSW5RaU9uUnlkV1Y5In0sImEiOnsiQ3Vyc29yIjoiZXlKeklqb3hOVGs0TkM0MU56RXhNekExTVRZNU1ESXNJbVFpT21aaGJITmxMQ0owSWpwMGNuVmxmUT09In19"
        }
      }
"#
        .as_bytes().to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/streams?");

    dbg!(GetStreamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_null_tags_issue184() {
    use helix::*;
    let req = GetStreamsRequest::default();

    // From twitch docs, kinda. example 1 in https://dev.twitch.tv/docs/api/reference#get-streams is malformed
    let data = br#"
{
    "data": [
        {
            "id": "123456789",
            "user_id": "98765",
            "user_login": "sandysanderman",
            "user_name": "SandySanderman",
            "game_id": "494131",
            "game_name": "Little Nightmares",
            "type": "live",
            "title": "hablamos y le damos a Little Nightmares 1",
            "tags": null,
            "viewer_count": 78365,
            "started_at": "2021-03-10T15:04:21Z",
            "language": "es",
            "thumbnail_url": "https://static-cdn.jtvnw.net/previews-ttv/live_user_auronplay-{width}x{height}.jpg",
            "tag_ids": null,
            "is_mature": false
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

    dbg!(GetStreamsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

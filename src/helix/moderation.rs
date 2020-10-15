//! Endpoints regarding moderation

use crate::{helix, types};
#[doc(inline)]
pub use check_automod_status::{
    CheckAutoModStatus, CheckAutoModStatusBody, CheckAutoModStatusRequest,
};
#[doc(inline)]
pub use get_banned_events::{BannedEvents, GetBannedEventsRequest};
#[doc(inline)]
pub use get_banned_users::{BannedUsers, GetBannedUsersRequest};
#[doc(inline)]
pub use get_moderator_events::{GetModeratorEventsRequest, ModeratorEvents};
#[doc(inline)]
pub use get_moderators::{GetModeratorsRequest, Moderators};
use serde::{Deserialize, Serialize};

/// Returns all moderators in a channel.
/// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
pub mod get_moderators {
    use super::*;

    // FIXME: Twitch Docs is borked here, mentions query param user_id
    // user_id	no	string	Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
    // Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
    // Maximum: 100
    /// Query Parameters for [Get Moderators](super::get_moderators)
    ///
    /// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetModeratorsRequest {
        /// Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Moderators](super::get_moderators)
    ///
    /// [`get-moderators`](https://dev.twitch.tv/docs/api/reference#get-moderators)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct Moderators {
        /// User ID of moderator
        ///
        /// Twitch says: `User ID of a user who has been banned.` but this seems wrong.
        user_id: types::UserId,
        /// Display name of moderator
        ///
        /// Twitch says: `Display name of a user who has been banned.` but this seems wrong.
        user_name: types::DisplayName,
    }

    impl helix::Request for GetModeratorsRequest {
        type Response = Vec<Moderators>;

        const PATH: &'static str = "moderation/moderators";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestGet for GetModeratorsRequest {}

    impl helix::Paginated for GetModeratorsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetModeratorsRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
            "user_id": "424596340",
            "user_name": "quotrok"
        },
        {
            "user_id": "424596340",
            "user_name": "quotrok"
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/moderation/moderators?broadcaster_id=198704263");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Returns a list of moderators or users added and removed as moderators from a channel.
/// [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
pub mod get_moderator_events {
    use super::*;
    use std::collections::HashMap;

    /// Query Parameters for [Get Moderators Events](super::get_moderator_events)
    ///
    /// [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetModeratorEventsRequest {
        /// Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
        // FIXME: Twitch docs sucks...
        /// Filters the results and only returns a status object for users who are moderators in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/moderators?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<types::UserId>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Moderators Events](super::get_moderator_events)
    ///
    /// [`get-moderator-events`](https://dev.twitch.tv/docs/api/reference#get-moderator-events)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct ModeratorEvents {
        /// Event ID
        pub id: String,
        // FIXME: Twitch docs sucks...
        /// Displays `moderation.moderator.add` or `moderation.moderator.remove`
        pub event_type: String,
        /// RFC3339 formatted timestamp for events.
        pub event_timestamp: types::Timestamp,
        /// Returns the version of the endpoint.
        pub version: String,
        /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
        pub event_data: HashMap<String, String>,
    }

    impl helix::Request for GetModeratorEventsRequest {
        type Response = Vec<ModeratorEvents>;

        const PATH: &'static str = "moderation/moderators/events";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestGet for GetModeratorEventsRequest {}

    impl helix::Paginated for GetModeratorEventsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetModeratorEventsRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
        "id": "1IVBTnDSUDApiBQW4UBcVTK4hPr",
        "event_type": "moderation.moderator.remove",
        "event_timestamp": "2019-03-15T18:18:14Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_name": "aan22209",
            "user_id": "423374343",
            "user_name": "glowillig"
        }
        },
        {
        "id": "1IVIPQdYIEnD8nJ376qkASDzsj7",
        "event_type": "moderation.moderator.add",
        "event_timestamp": "2019-03-15T19:15:13Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_name": "aan22209",
            "user_id": "423374343",
            "user_name": "glowillig"
        }
        },
        {
        "id": "1IVBTP7gG61oXLMu7fvnRhrpsro",
        "event_type": "moderation.moderator.remove",
        "event_timestamp": "2019-03-15T18:18:11Z",
        "version": "1.0",
        "event_data": {
            "broadcaster_id": "198704263",
            "broadcaster_name": "aan22209",
            "user_id": "424596340",
            "user_name": "quotrok"
        }
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/moderation/moderators/events?broadcaster_id=198704263");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Returns all banned and timed-out users in a channel.
/// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
pub mod get_banned_users {
    use super::*;

    /// Query Parameters for [Get Banned Users](super::get_banned_users)
    ///
    /// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetBannedUsersRequest {
        /// Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
        /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<types::UserId>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
    }

    /// Return Values for [Get Banned Users](super::get_banned_users)
    ///
    /// [`get-banned-users`](https://dev.twitch.tv/docs/api/reference#get-banned-users)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct BannedUsers {
        /// User ID of a user who has been banned.
        pub user_id: types::UserId,
        /// Display name of a user who has been banned.
        pub user_name: types::DisplayName,
        /// RFC3339 formatted timestamp for timeouts; empty string for bans.
        pub expires_at: Option<types::Timestamp>,
    }

    impl helix::Request for GetBannedUsersRequest {
        type Response = Vec<BannedUsers>;

        const PATH: &'static str = "moderation/banned";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestGet for GetBannedUsersRequest {}

    impl helix::Paginated for GetBannedUsersRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetBannedUsersRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [
        {
        "user_id": "423374343",
        "user_name": "glowillig",
        "expires_at": "2019-03-15T02:00:28Z"
        },
        {
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": "2018-08-07T02:07:55Z"
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjEwMDQ3MzA2NDo4NjQwNjU3MToxSVZCVDFKMnY5M1BTOXh3d1E0dUdXMkJOMFcifX0"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/moderation/banned?broadcaster_id=198704263");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Returns all banned and timed-out users in a channel.
/// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
pub mod get_banned_events {
    use super::*;
    use std::collections::HashMap;

    /// Query Parameters for [Get Banned Events](super::get_banned_events)
    ///
    /// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct GetBannedEventsRequest {
        /// Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
        /// Filters the results and only returns a status object for users who are banned in this channel and have a matching user_id.
        /// Format: Repeated Query Parameter, eg. /moderation/banned?broadcaster_id=1&user_id=2&user_id=3
        /// Maximum: 100
        #[builder(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub user_id: Vec<types::UserId>,
        /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub after: Option<helix::Cursor>,
        /// Maximum number of objects to return. Maximum: 100. Default: 20.
        #[builder(default, setter(into))]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first: Option<usize>,
    }

    /// Return Values for [Get Banned Events](super::get_banned_events)
    ///
    /// [`get-banned-events`](https://dev.twitch.tv/docs/api/reference#get-banned-events)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct BannedEvents {
        /// Event ID
        pub id: String,
        /// Displays `moderation.user.ban` or `moderation.user.unban`
        pub event_type: String,
        /// RFC3339 formatted timestamp for events.
        pub event_timestamp: String,
        /// Returns the version of the endpoint.
        pub version: String,
        // FIXME: Should be a struct, maybe
        /// Returns `broadcaster_id`, `broadcaster_name`, `user_id`, `user_name`, and `expires_at`.
        pub event_data: HashMap<String, String>,
    }

    impl helix::Request for GetBannedEventsRequest {
        type Response = Vec<BannedEvents>;

        const PATH: &'static str = "moderation/banned/events";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestGet for GetBannedEventsRequest {}

    impl helix::Paginated for GetBannedEventsRequest {
        fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = GetBannedEventsRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
    "data": [
    {
        "id": "1IPFqAb0p0JncbPSTEPhx8JF1Sa",
        "event_type": "moderation.user.ban",
        "event_timestamp": "2019-03-13T15:55:14Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    },
    {
        "id": "1IPFsDv5cs4mxfJ1s2O9Q5flf4Y",
        "event_type": "moderation.user.unban",
        "event_timestamp": "2019-03-13T15:55:30Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    },
    {
        "id": "1IPFqmlu9W2q4mXXjULyM8zX0rb",
        "event_type": "moderation.user.ban",
        "event_timestamp": "2019-03-13T15:55:19Z",
        "version": "1.0",
        "event_data": {
        "broadcaster_id": "198704263",
        "broadcaster_name": "aan22209",
        "user_id": "424596340",
        "user_name": "quotrok",
        "expires_at": ""
        }
    }
    ],
    "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjp7IkN1cnNvciI6IjE5OTYwNDI2MzoyMDIxMjA1MzE6MUlQRnFtbHU5VzJxNG1YWGpVTHlNOHpYMHJiIn19"
    }
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/moderation/banned/events?broadcaster_id=198704263");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

/// Determines whether a string message meets the channel’s AutoMod requirements.
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
pub mod check_automod_status {
    use super::*;
    /// Query Parameters for [Check AutoMod Status](super::check_automod_status)
    ///
    /// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
    #[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
    #[non_exhaustive]
    pub struct CheckAutoModStatusRequest {
        /// Must match the User ID in the Bearer token.
        #[builder(setter(into))]
        pub broadcaster_id: types::UserId,
    }

    /// Body Parameters for [Check AutoMod Status](super::check_automod_status)
    ///
    /// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
    #[derive(
        PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default,
    )]
    #[non_exhaustive]
    pub struct CheckAutoModStatusBody {
        /// Developer-generated identifier for mapping messages to results.
        #[builder(setter(into))]
        pub msg_id: String,
        /// Message text.
        #[builder(setter(into))]
        pub msg_text: String,
        /// User ID of the sender.
        #[builder(setter(into))]
        pub user_id: String,
    }

    /// Return Values for [Check AutoMod Status](super::check_automod_status)
    ///
    /// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
    #[derive(PartialEq, Deserialize, Debug, Clone)]
    #[non_exhaustive]
    pub struct CheckAutoModStatus {
        /// The msg_id passed in the body of the POST message. Maps each message to its status.
        pub msg_id: String,
        /// Indicates if this message meets AutoMod requirements.
        pub is_permitted: bool,
    }

    impl helix::Request for CheckAutoModStatusRequest {
        type Response = Vec<CheckAutoModStatus>;

        const PATH: &'static str = "moderation/enforcements/status";
        #[cfg(feature = "twitch_oauth2")]
        const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
    }

    impl helix::RequestPost for CheckAutoModStatusRequest {
        type Body = Vec<CheckAutoModStatusBody>;

        fn body(&self, body: &Self::Body) -> Result<String, serde_json::Error> {
            #[derive(Serialize)]
            struct InnerBody<'a> {
                data: &'a Vec<CheckAutoModStatusBody>,
            }

            serde_json::to_string(&InnerBody { data: &body })
        }
    }

    #[test]
    fn test_request() {
        use helix::*;
        let req = CheckAutoModStatusRequest::builder()
            .broadcaster_id("198704263".to_string())
            .build();

        // From twitch docs
        let data = br#"
{
   "data": [
     {
       "msg_id": "123",
       "is_permitted": true
     },
     {
       "msg_id": "393",
       "is_permitted": false
     }
   ]
}
"#
        .to_vec();

        let http_response = http::Response::builder().body(data).unwrap();

        let uri = req.get_uri().unwrap();
        assert_eq!(uri.to_string(), "https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=198704263");

        dbg!(req.parse_response(&uri, http_response).unwrap());
    }
}

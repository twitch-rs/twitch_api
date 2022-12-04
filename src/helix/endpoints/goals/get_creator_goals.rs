//! Gets the broadcaster’s list of active goals. Use this to get the current progress of each goal.
//! [`get-creator-goals`](https://dev.twitch.tv/docs/api/reference#get-creator-goals)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCreatorGoalsRequest]
//!
//! To use this endpoint, construct a [`GetCreatorGoalsRequest`] with the [`GetCreatorGoalsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::goals::get_creator_goals;
//! let request =
//!     get_creator_goals::GetCreatorGoalsRequest::broadcaster_id("4321");
//! ```
//!
//! ## Response: [CreatorGoal](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, goals::get_creator_goals};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_creator_goals::GetCreatorGoalsRequest::broadcaster_id("4321");
//! let response: Vec<get_creator_goals::CreatorGoal> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCreatorGoalsRequest::parse_response(None, &request.get_uri(), response)`](GetCreatorGoalsRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Creator Goals](super::get_creator_goals)
///
/// [`get-creator-goals`](https://dev.twitch.tv/docs/api/reference#get-creator-goals)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetCreatorGoalsRequest<'a> {
    /// Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    pub cursor: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// Retreive a single event by event ID
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub id: Option<Cow<'a, str>>,
}

impl<'a> GetCreatorGoalsRequest<'a> {
    /// Gets the broadcaster’s list of active goals.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.to_cow(),
            cursor: Default::default(),
            first: Default::default(),
            id: Default::default(),
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Get Creator Goals](super::get_creator_goals)
///
/// [`get-creator-goals`](https://dev.twitch.tv/docs/api/reference#get-creator-goals)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CreatorGoal {
    /// An ID that uniquely identifies this goal.
    pub id: types::CreatorGoalId,
    /// An ID that uniquely identifies the broadcaster.
    pub broadcaster_id: types::UserId,
    /// The broadcaster’s display name.
    pub broadcaster_name: types::DisplayName,
    /// The broadcaster’s user handle.
    pub broadcaster_login: types::UserName,
    /// The type of goal.
    #[serde(rename = "type")]
    pub type_: types::CreatorGoalType,
    /// A description of the goal, if specified. The description may contain a maximum of 40 characters.
    pub description: String,
    /// The current value.
    pub current_amount: i64,
    /// The goal’s target value. For example, if the broadcaster has 200 followers before creating the goal, and their goal is to double that number, this field is set to 400.
    pub target_amount: i64,
    /// The UTC timestamp in RFC 3339 format, which indicates when the broadcaster created the goal.
    pub created_at: types::Timestamp,
}

impl Request for GetCreatorGoalsRequest<'_> {
    type Response = Vec<CreatorGoal>;

    const PATH: &'static str = "goals";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadGoals];
}

impl RequestGet for GetCreatorGoalsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetCreatorGoalsRequest::broadcaster_id("141981764");

    // From twitch docs
    let data = br#"
    {
        "data": [
          {
            "id": "1woowvbkiNv8BRxEWSqmQz6Zk92",
            "broadcaster_id": "141981764",
            "broadcaster_name": "TwitchDev",
            "broadcaster_login": "twitchdev",
            "type": "follower",
            "description": "Follow goal for Helix testing",
            "current_amount": 27062,
            "target_amount": 30000,
            "created_at": "2021-08-16T17:22:23Z"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/goals?broadcaster_id=141981764"
    );

    dbg!(GetCreatorGoalsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

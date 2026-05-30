//! Gets the status of a Hype Train for the specified broadcaster.
//! [`get-hype-train-status`](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetHypeTrainStatusRequest]
//!
//! To use this endpoint, construct a [`GetHypeTrainStatusRequest`] with the [`GetHypeTrainStatusRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::hypetrain::get_hype_train_status;
//! let request = get_hype_train_status::GetHypeTrainStatusRequest::new("4321");
//! ```
//!
//! ## Response: [HypeTrainStatus]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, hypetrain::get_hype_train_status};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_hype_train_status::GetHypeTrainStatusRequest::new("4321");
//! let response: get_hype_train_status::HypeTrainStatus = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetHypeTrainStatusRequest::parse_response(None, &request.get_uri(), response)`](GetHypeTrainStatusRequest::parse_response)

use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Hype Train Status](super::get_hype_train_status)
///
/// [`get-hype-train-status`](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetHypeTrainStatusRequest<'a> {
    /// Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetHypeTrainStatusRequest<'a> {
    /// Get hypetrain evens
    pub fn new(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Hype Train Status](super::get_hype_train_status)
///
/// [`get-hype-train-status`](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainStatus {
    /// An object describing the current Hype Train. [None] if a Hype Train is not active.
    pub current: Option<HypeTrain>,
    /// An object with information about the channel’s Hype Train records. [None] if a Hype Train has not occurred.
    pub all_time_high: Option<HypeTrainRecord>,
    /// An object with information about the channel’s shared Hype Train records. [None] if a Hype Train has not occurred.
    pub shared_all_time_high: Option<HypeTrainRecord>,
}

/// Return Values for [Get Hype Train Status](super::get_hype_train_status)
///
/// [`get-hype-train-status`](https://dev.twitch.tv/docs/api/reference#get-hype-train-status)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrain {
    /// The Hype Train ID.
    pub id: types::HypeTrainId,
    /// The broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The current level of the Hype Train.
    pub level: u64,
    /// Total points contributed to the Hype Train.
    pub total: u64,
    /// The number of points contributed to the Hype Train at the current level.
    pub progress: u64,
    /// The number of points required to reach the next level.
    pub goal: u64,
    /// The contributors with the most points contributed.
    pub top_contributions: Vec<TopContribution>,
    /// A list containing the broadcasters participating in the shared Hype Train. Empty if the Hype Train is not shared.
    #[serde(default)]
    pub shared_train_participants: Vec<SharedTrainParticipant>,
    /// The time when the Hype Train started.
    pub started_at: types::Timestamp,
    /// The time when the Hype Train expires. The expiration is extended when the Hype Train reaches a new level.
    pub expires_at: types::Timestamp,
    /// The type of the Hype Train.
    #[serde(rename = "type")]
    pub type_: HypeTrainType,
    /// Indicates if the Hype Train is shared. When true, `shared_train_participants`` will contain the list of broadcasters the train is shared with.
    #[serde(default)]
    pub is_shared_train: bool,
}

/// A contribution with the most points contributed.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TopContribution {
    /// The ID of the user that made the contribution.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::UserName,
    /// The contribution method used.
    #[serde(rename = "type")]
    pub type_: HypeTrainContributionType,
    /// The total number of points contributed for the type.
    pub total: u64,
}

/// A broadcaster participating in a hype train.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct SharedTrainParticipant {
    /// The broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The broadcaster login.
    pub broadcaster_user_login: types::UserName,
    ///The broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
}

/// Information about a channel’s Hype Train records.
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct HypeTrainRecord {
    /// The level of the record Hype Train.
    pub level: u64,
    /// Total points contributed to the record Hype Train.
    pub total: u64,
    /// The time when the record was achieved.
    pub achieved_at: types::Timestamp,
}

/// Type of Hype Train event
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum HypeTrainType {
    /// A treasure train.
    Treasure,
    /// A golden Kappa train.
    GoldenKappa,
    /// A regular train.
    Regular,
    /// An unknown hype train type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

/// Type of Hype Train event
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum HypeTrainContributionType {
    /// Cheering with bits
    Bits,
    /// Subscription activity like subscribing or gifting subscriptions.
    Subscription,
    /// Covers other contribution methods not listed.
    Other,
    /// An unknown contribution type, contains the raw string provided by Twitch.
    #[serde(untagged)]
    Unknown(String),
}

impl Request for GetHypeTrainStatusRequest<'_> {
    type PaginationData = ();
    type Response = HypeTrainStatus;

    const PATH: &'static str = "hypetrain/status";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelReadHypeTrain];
}

impl RequestGet for GetHypeTrainStatusRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        helix::parse_single_return(request, uri, response, status)
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetHypeTrainStatusRequest::new("123");

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "current": {
                    "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
                    "broadcaster_user_id": "1337",
                    "broadcaster_user_login": "cool_user",
                    "broadcaster_user_name": "Cool_User",
                    "level": 2,
                    "total": 700,
                    "progress": 200,
                    "goal": 1000,
                    "top_contributions": [
                        {
                            "user_id": "123",
                            "user_login": "pogchamp",
                            "user_name": "PogChamp",
                            "type": "bits",
                            "total": 50
                        },
                        {
                            "user_id": "456",
                            "user_login": "kappa",
                            "user_name": "Kappa",
                            "type": "subscription",
                            "total": 45
                        }
                    ],
                    "shared_train_participants": [
                        {
                            "broadcaster_user_id": "456",
                            "broadcaster_user_login": "pogchamp",
                            "broadcaster_user_name": "PogChamp"
                        },
                        {
                            "broadcaster_user_id": "321",
                            "broadcaster_user_login": "pogchamp",
                            "broadcaster_user_name": "PogChamp"
                        }
                    ],
                    "started_at": "2020-07-15T17:16:03.17106713Z",
                    "expires_at": "2020-07-15T17:16:11.17106713Z",
                    "type": "golden_kappa"
                },
                "all_time_high": {
                    "level": 6,
                    "total": 2850,
                    "achieved_at": "2020-04-24T20:12:21.003802269Z"
                },
                "shared_all_time_high": {
                    "level": 16,
                    "total": 23850,
                    "achieved_at": "2020-04-27T20:12:21.003802269Z"
                }
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/hypetrain/status?broadcaster_id=123"
    );

    dbg!(GetHypeTrainStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_no_shared() {
    use helix::*;
    let req = GetHypeTrainStatusRequest::new("123");

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "current": {
                    "id": "1b0AsbInCHZW2SQFQkCzqN07Ib2",
                    "broadcaster_user_id": "1337",
                    "broadcaster_user_login": "cool_user",
                    "broadcaster_user_name": "Cool_User",
                    "level": 2,
                    "total": 700,
                    "progress": 200,
                    "goal": 1000,
                    "top_contributions": [
                        {
                            "user_id": "123",
                            "user_login": "pogchamp",
                            "user_name": "PogChamp",
                            "type": "bits",
                            "total": 50
                        },
                        {
                            "user_id": "456",
                            "user_login": "kappa",
                            "user_name": "Kappa",
                            "type": "subscription",
                            "total": 45
                        }
                    ],
                    "started_at": "2020-07-15T17:16:03.17106713Z",
                    "expires_at": "2020-07-15T17:16:11.17106713Z",
                    "type": "golden_kappa"
                },
                "all_time_high": {
                    "level": 6,
                    "total": 2850,
                    "achieved_at": "2020-04-24T20:12:21.003802269Z"
                },
                "shared_all_time_high": {
                    "level": 16,
                    "total": 23850,
                    "achieved_at": "2020-04-27T20:12:21.003802269Z"
                }
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/hypetrain/status?broadcaster_id=123"
    );

    dbg!(GetHypeTrainStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_empty() {
    use helix::*;
    let req = GetHypeTrainStatusRequest::new("123");

    // From twitch docs
    let data = br#"
    {
        "data": [
            {
                "current": null,
                "all_time_high": null,
                "shared_all_time_high": null
            }
        ]
    }
    "#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/hypetrain/status?broadcaster_id=123"
    );

    dbg!(GetHypeTrainStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

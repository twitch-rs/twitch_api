//! Checks if a specific user is subscribed to a specific channel.
//! [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CheckUserSubscription]
//!
//! To use this endpoint, construct a [`CheckUserSubscription`] with the [`CheckUserSubscription::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::subscriptions::check_user_subscription;
//! let request = check_user_subscription::CheckUserSubscription::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [UserSubscription]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, subscriptions::check_user_subscription};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = check_user_subscription::CheckUserSubscription::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let response: check_user_subscription::UserSubscription = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestGet::parse_response())

use std::convert::TryInto;

use super::*;
/// Query Parameters for [Check User Subscription](super::check_user_subscription)
///
/// [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CheckUserSubscription {
    /// User ID of the broadcaster. Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// Unique identifier of account to get subscription status of. Accepts up to 100 values.
    #[builder(default)]
    pub user_id: Vec<types::UserId>,
}

/// Return Values for [Check User Subscription](super::check_user_subscription)
///
/// [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserSubscription {
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Indicates if the subscription is a gift.
    pub is_gift: bool,
    /// Login of the gifter (if is_gift is true).
    pub gifter_login: Option<types::UserName>,
    /// Display name of the gifter (if is_gift is true).
    pub gifter_name: Option<types::DisplayName>,
    /// Subscription tier. 1000 is tier 1, 2000 is tier 2, and 3000 is tier 3.
    pub tier: types::SubscriptionTier,
}

impl helix::Request for CheckUserSubscription {
    type Response = UserSubscription;

    const PATH: &'static str = "subscriptions/user";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::UserReadSubscriptions];
}

impl helix::RequestGet for CheckUserSubscription {
    fn parse_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: http::Response<Vec<u8>>,
    ) -> Result<
        helix::Response<Self, <Self as helix::Request>::Response>,
        helix::HelixRequestGetError,
    >
    where
        Self: Sized,
    {
        let text = std::str::from_utf8(&response.body()).map_err(|e| {
            helix::HelixRequestGetError::Utf8Error(response.body().clone(), e, uri.clone())
        })?;
        //eprintln!("\n\nmessage is ------------ {} ------------", text);
        if let Ok(helix::HelixRequestError {
            error,
            status,
            message,
        }) = serde_json::from_str::<helix::HelixRequestError>(&text)
        {
            return Err(helix::HelixRequestGetError::Error {
                error,
                status: status.try_into().unwrap_or(http::StatusCode::BAD_REQUEST),
                message,
                uri: uri.clone(),
            });
        }
        let inner_response: helix::InnerResponse<Vec<_>> =
            serde_json::from_str(&text).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(text.to_string(), e, uri.clone())
            })?;
        Ok(helix::Response {
            data: inner_response.data.into_iter().next().ok_or(
                helix::HelixRequestGetError::InvalidResponse {
                    reason: "expected an entry in `data`",
                    response: text.to_string(),
                    status: response.status(),
                    uri: uri.clone(),
                },
            )?,
            pagination: inner_response.pagination.cursor,
            request,
        })
    }
}

#[test]
fn test_request1() {
    use helix::*;
    let req = CheckUserSubscription::builder()
        .broadcaster_id("123".to_string())
        .build();

    // From twitch docs.
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "149747285",
            "broadcaster_name": "TwitchPresents",
            "broadcaster_login": "twitchpresents",
            "is_gift": false,
            "tier": "1000"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id=123"
    );

    dbg!(CheckUserSubscription::parse_response(Some(req), &uri, http_response).unwrap());
}

#[test]
fn test_request2() {
    use helix::*;
    let req = CheckUserSubscription::builder()
        .broadcaster_id("123".to_string())
        .build();

    // From twitch docs.
    let data = br#"
    {
        "error": "Not Found",
        "message": "twitchdev has no subscription to twitchpresents",
        "status": 404
      }
"#
    .to_vec();

    let http_response = http::Response::builder().status(404).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id=123"
    );

    dbg!(CheckUserSubscription::parse_response(Some(req), &uri, http_response).unwrap_err());
}

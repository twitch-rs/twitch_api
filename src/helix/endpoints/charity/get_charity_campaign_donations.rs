//! Gets the list of donations that users have made to the broadcaster’s active charity campaign.
//! [`get-charity-campaign-donations`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign-donations)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCharityCampaignDonationsRequest]
//!
//! To use this endpoint, construct a [`GetCharityCampaignDonationsRequest`] with the [`GetCharityCampaignDonationsRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::charity::get_charity_campaign_donations;
//! let request =
//!     get_charity_campaign_donations::GetCharityCampaignDonationsRequest::broadcaster_id("123456");
//! ```
//!
//! ## Response: [CharityCampaignDonation]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, charity::get_charity_campaign_donations};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_charity_campaign_donations::GetCharityCampaignDonationsRequest::broadcaster_id("123456");
//! let response: Vec<get_charity_campaign_donations::CharityCampaignDonation> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCharityCampaignDonationsRequest::parse_response(None, &request.get_uri(), response)`](GetCharityCampaignDonationsRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Charity Campaign Donations](super::get_charity_campaign_donations)
///
/// [`get-charity-campaign-donations`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign-donations)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct GetCharityCampaignDonationsRequest<'a> {
    /// The ID of the broadcaster that’s currently running a charity campaign. This ID must match the user ID in the access token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The maximum number of items to return per page in the response. The minimum page size is 1 item per page and the maximum is 100. The default is 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// The cursor used to get the next page of results. The Pagination object in the response contains the cursor’s value.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
}

impl<'a> GetCharityCampaignDonationsRequest<'a> {
    /// Gets information about the charity campaign that a broadcaster is running.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            after: None,
            first: None,
        }
    }

    /// Set amount of results returned per page.
    pub fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

impl helix::Paginated for GetCharityCampaignDonationsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

/// Return Values for [Get Charity Campaign Donations](super::get_charity_campaign_donations)
///
/// [`get-charity-campaign-donations`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign-donations)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CharityCampaignDonation {
    /// An ID that identifies the donation. The ID is unique across campaigns.
    pub id: types::CharityDonationId,
    /// An ID that identifies the charity campaign that the donation applies to.
    pub campaign_id: types::CharityCampaignId,
    /// An ID that identifies a user that donated money to the campaign.
    pub user_id: types::UserId,
    /// The user’s login name.
    pub user_login: types::UserName,
    /// The user’s display name.
    pub user_name: types::DisplayName,
    /// An object that contains the amount of money that the user donated.
    pub amount: crate::extra::DonationAmount,
}

impl Request for GetCharityCampaignDonationsRequest<'_> {
    type Response = Vec<CharityCampaignDonation>;

    const PATH: &'static str = "charity/donations";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::scopes::Scope::ChannelReadCharity];
}

impl RequestGet for GetCharityCampaignDonationsRequest<'_> {}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetCharityCampaignDonationsRequest::broadcaster_id("123456");

    // twitch docs has ...
    let data = br##"
    {
        "data": [
          {
            "id": "a1b2c3-aabb-4455-d1e2f3",
            "campaign_id": "123-abc-456-def",
            "user_id": "5678",
            "user_login": "cool_user",
            "user_name": "Cool_User",
            "amount": {
              "value": 500,
              "decimal_places": 2,
              "currency": "USD"
            }
          },
          {
            "id": "z1y2x3-ccdd-6677-d1e2f3",
            "campaign_id": "123-abc-456-def",
            "user_id": "8765",
            "user_login": "cool_user2",
            "user_name": "Cool_User2",
            "amount": {
              "value": 10000,
              "decimal_places": 2,
              "currency": "USD"
            }
          }
        ],
        "pagination" : {
            "cursor" : "eyJiIjpudWxsLCJhIjp7Ik9mZnNldCI6NX19"
        }
      }
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/charity/donations?broadcaster_id=123456"
    );

    dbg!(
        GetCharityCampaignDonationsRequest::parse_response(Some(req), &uri, http_response).unwrap()
    );
}

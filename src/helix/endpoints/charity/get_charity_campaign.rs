//! Gets information about the charity campaign that a broadcaster is running, such as their fundraising goal and the amount that’s been donated so far.
//! [`get-charity-campaign`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign)
//!
//! # Accessing the endpoint
//!
//! ## Request: [GetCharityCampaignRequest]
//!
//! To use this endpoint, construct a [`GetCharityCampaignRequest`] with the [`GetCharityCampaignRequest::builder()`] method.
//!
//! ```rust
//! use twitch_api::helix::charity::get_charity_campaign;
//! let request =
//!     get_charity_campaign::GetCharityCampaignRequest::broadcaster_id(
//!         "123456",
//!     );
//! ```
//!
//! ## Response: [CharityCampaign]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, charity::get_charity_campaign};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_charity_campaign::GetCharityCampaignRequest::broadcaster_id("123456");
//! let response: Option<get_charity_campaign::CharityCampaign> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetCharityCampaignRequest::parse_response(None, &request.get_uri(), response)`](GetCharityCampaignRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Get Charity Campaign](super::get_charity_campaign)
///
/// [`get-charity-campaign`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
pub struct GetCharityCampaignRequest<'a> {
    /// The ID of the broadcaster that’s actively running a charity campaign.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> GetCharityCampaignRequest<'a> {
    /// Gets information about the charity campaign that a broadcaster is running.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Get Charity Campaign](super::get_charity_campaign)
///
/// [`get-charity-campaign`](https://dev.twitch.tv/docs/api/reference#get-charity-campaign)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CharityCampaign {
    /// An ID that uniquely identifies the charity campaign.
    pub id: types::CharityCampaignId,
    /// An ID that uniquely identifies the broadcaster that’s running the campaign.
    pub broadcaster_id: types::UserId,
    /// The broadcaster’s login name.
    pub broadcaster_login: types::UserName,
    /// The broadcaster’s display name.
    pub broadcaster_name: types::DisplayName,
    /// The charity’s name.
    pub charity_name: String,
    /// A description of the charity.
    pub charity_description: String,
    /// A URL to an image of the charity’s logo. The image’s type is PNG and its size is 100px X 100px.
    pub charity_logo: String,
    /// A URL to the charity
    pub charity_website: String,
    /// The current amount of donations that the campaign has received.
    pub current_amount: crate::extra::DonationAmount,
    /// An object that contains the amount of money that the campaign is trying to raise.
    ///
    /// This field may be [`None`] if the broadcaster has not defined a target goal.
    pub target_amount: Option<crate::extra::DonationAmount>,
}

impl Request for GetCharityCampaignRequest<'_> {
    type Response = Option<CharityCampaign>;

    const PATH: &'static str = "charity/campaigns";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] =
        &[twitch_oauth2::scopes::Scope::ChannelReadCharity];
}

impl RequestGet for GetCharityCampaignRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, <Self as Request>::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<Vec<_>> =
            crate::parse_json(response, true).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response {
            data: response.data.into_iter().next(),
            pagination: response.pagination.cursor,
            request,
            total: response.total,
            other: response.other,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetCharityCampaignRequest::broadcaster_id("123456");

    // From api call
    let data = br##"
    {
        "data": [
          {
            "id": "123-abc-456-def",
            "broadcaster_id": "123456",
            "broadcaster_name": "SunnySideUp",
            "broadcaster_login": "sunnysideup",
            "charity_name": "Example name",
            "charity_description": "Example description",
            "charity_logo": "https://example.url/logo.png",
            "charity_website": "https://www.example.com",
            "current_amount": {
              "value": 86000,
              "decimal_places": 2,
              "currency": "USD"
            },
            "target_amount": {
              "value": 1500000,
              "decimal_places": 2,
              "currency": "USD"
            }
          }
        ]
      }
"##
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/charity/campaigns?broadcaster_id=123456"
    );

    dbg!(GetCharityCampaignRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

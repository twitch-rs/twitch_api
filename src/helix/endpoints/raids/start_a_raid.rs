//! Raid another channel by sending the broadcaster’s viewers to the targeted channel.
//! [`start-a-raid`](https://dev.twitch.tv/docs/api/reference#start-a-raid)
//!
//! # Accessing the endpoint
//!
//! ## Request: [StartARaidRequest]
//!
//! To use this endpoint, construct a [`StartARaidRequest`] with the [`StartARaidRequest::new()`] method.
//!
//! ```rust
//! use twitch_api::helix::raids::start_a_raid;
//! let request = start_a_raid::StartARaidRequest::new("12345678", "87654321");
//! ```

//! ## Response: [StartARaidResponse]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, raids::start_a_raid};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = start_a_raid::StartARaidRequest::new("12345678", "87654321");
//! let response: start_a_raid::StartARaidResponse = client.req_post(request, Default::default(), &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`StartARaidRequest::parse_response(None, &request.get_uri(), response)`](StartARaidRequest::parse_response)

use super::*;
use helix::RequestPost;
/// Query Parameters for [Start A Raid](super::start_a_raid)
///
/// [`start-a-raid`](https://dev.twitch.tv/docs/api/reference#start-a-raid)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct StartARaidRequest<'a> {
    /// The ID of the broadcaster that’s sending the raiding party.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    from_broadcaster_id: Cow<'a, types::UserIdRef>,
    /// The ID of the broadcaster to raid.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    to_broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> StartARaidRequest<'a> {
    /// Create a new [`StartARaidRequest`]
    pub fn new(
        from_broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        to_broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            from_broadcaster_id: from_broadcaster_id.into_cow(),
            to_broadcaster_id: to_broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Start A Raid](super::start_a_raid)
///
/// [`start-a-raid`](https://dev.twitch.tv/docs/api/reference#start-a-raid)
#[derive(PartialEq, Eq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct StartARaidResponse {
    /// The UTC date and time, in RFC3339 format, when the raid request was created.
    created_at: types::Timestamp,
    /// A Boolean value that indicates whether the channel being raided contains mature content.
    is_mature: bool,
}
impl Request for StartARaidRequest<'_> {
    type Response = StartARaidResponse;

    const PATH: &'static str = "raids";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::ChannelManageRaids];
}

impl RequestPost for StartARaidRequest<'_> {
    type Body = helix::EmptyBody;

    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response_str: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestPostError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<Vec<Self::Response>> =
            helix::parse_json(response_str, true).map_err(|e| {
                helix::HelixRequestPostError::DeserializeError(
                    response_str.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        let data = response.data.into_iter().next().ok_or_else(|| {
            helix::HelixRequestPostError::InvalidResponse {
                reason: "response included no data",
                response: response_str.to_string(),
                status,
                uri: uri.clone(),
            }
        })?;
        Ok(helix::Response {
            data,
            pagination: response.pagination.cursor,
            request,
            total: None,
            other: None,
        })
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = StartARaidRequest::new("12345678", "87654321");

    dbg!(req
        .create_request(Default::default(), "token", "clientid")
        .unwrap());

    // From twitch docs
    let data = br##"
{
    "data": [
        {
            "created_at": "2022-02-18T07:20:50.52Z",
            "is_mature": false
        }
    ]
}
    "##
    .to_vec();

    let http_response = http::Response::builder().status(200).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/raids?from_broadcaster_id=12345678&to_broadcaster_id=87654321"
    );

    dbg!(StartARaidRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

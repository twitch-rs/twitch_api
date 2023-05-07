//! Cancel a pending raid.
//! [`cancel-a-raid`](https://dev.twitch.tv/docs/api/reference#cancel-a-raid)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CancelARaidRequest]
//!
//! To use this endpoint, construct a [`CancelARaidRequest`] with the [`CancelARaidRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::raids::cancel_a_raid;
//! let request = cancel_a_raid::CancelARaidRequest::broadcaster_id("1234");
//! ```
//!
//! ## Response: [CancelARaidResponse]
//!
//! Send the request to receive the response with [`HelixClient::req_delete()`](helix::HelixClient::req_delete).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, raids::cancel_a_raid};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = cancel_a_raid::CancelARaidRequest::broadcaster_id("1234");
//! let response: cancel_a_raid::CancelARaidResponse = client.req_delete(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestDelete::create_request)
//! and parse the [`http::Response`] with [`CancelARaidRequest::parse_response(None, &request.get_uri(), response)`](CancelARaidRequest::parse_response)

use super::*;
use helix::RequestDelete;
/// Query Parameters for [Cancel A Raid](super::cancel_a_raid)
///
/// [`cancel-a-raid`](https://dev.twitch.tv/docs/api/reference#cancel-a-raid)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct CancelARaidRequest<'a> {
    /// The ID of the broadcaster that sent the raiding party.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
}

impl<'a> CancelARaidRequest<'a> {
    /// Cancel a pending raid on this broadcasters channel
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
        }
    }
}

/// Return Values for [Cancel A Raid](super::cancel_a_raid)
///
/// [`cancel-a-raid`](https://dev.twitch.tv/docs/api/reference#cancel-a-raid)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum CancelARaidResponse {
    /// 204 - The pending raid was successfully canceled.
    Success,
}

impl Request for CancelARaidRequest<'_> {
    type Response = CancelARaidResponse;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "raids";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelManageRaids];
}

impl RequestDelete for CancelARaidRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestDeleteError>
    where
        Self: Sized,
    {
        match status {
            http::StatusCode::NO_CONTENT => Ok(helix::Response::with_data(
                CancelARaidResponse::Success,
                request,
            )),
            _ => Err(helix::HelixRequestDeleteError::InvalidResponse {
                reason: "unexpected status",
                response: response.to_string(),
                status,
                uri: uri.clone(),
            }),
        }
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = CancelARaidRequest::broadcaster_id("12345678");

    // From twitch docs
    let data = br#""#.to_vec();

    let http_response = http::Response::builder().status(204).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/raids?broadcaster_id=12345678"
    );

    dbg!(CancelARaidRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

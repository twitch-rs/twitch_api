#![allow(unknown_lints)] // remove once broken_intra_doc_links is on stable
#![deny(missing_docs, broken_intra_doc_links)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/twitch_oauth2/0.4.1")]
//! [![github]](https://github.com/emilgardis/twitch_oauth2)&ensp;[![crates-io]](https://crates.io/crates/twitch_oauth2)&ensp;[![docs-rs]](https://docs.rs/twitch_oauth2/0.4.1/twitch_oauth2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__oauth2-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_oauth2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs]: https://img.shields.io/badge/docs.rs-twitch__oauth2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>OAuth2 for Twitch endpoints</h5>
//!
//! ```rust,no_run
//! # use twitch_oauth2::{TwitchToken, UserToken, AccessToken, tokens::errors::ValidationError};
//! # #[tokio::main]
//! # async fn run() {
//! # let reqwest_http_client = twitch_oauth2::dummy_http_client; // This is only here to fool doc tests
//!     let token = AccessToken::new("sometokenherewhichisvalidornot".to_string());
//!
//!     match UserToken::from_existing(reqwest_http_client, token, None).await {
//!         Ok(t) => println!("user_token: {}", t.token().secret()),
//!         Err(e) => panic!("got error: {}", e),
//!     }
//! # }
//! # fn main() {run()}
//! ```
pub mod client;
pub mod id;
pub mod scopes;
pub mod tokens;

#[doc(no_inline)]
pub use oauth2::{AccessToken, ClientId, ClientSecret, RefreshToken};

use id::{TwitchClient, TwitchTokenErrorResponse};
use oauth2::{url::Url, AuthUrl, HttpRequest, HttpResponse, TokenResponse};
use std::future::Future;
use tokens::errors::{RefreshTokenError, RevokeTokenError, ValidationError};

#[doc(inline)]
pub use scopes::Scope;
#[doc(inline)]
pub use tokens::{AppAccessToken, TwitchToken, UserToken, ValidatedToken};

#[doc(hidden)]
pub async fn dummy_http_client(_: HttpRequest) -> Result<HttpResponse, DummyError> {
    Err(DummyError)
}
#[doc(hidden)]
#[derive(Debug, thiserror::Error)]
#[error("this client does not do anything, only used for documentation test that only checks code integrity")]
pub struct DummyError;

/// Validate this token.
///
/// Should be checked on regularly, according to <https://dev.twitch.tv/docs/authentication#validating-requests>
pub async fn validate_token<RE, C, F>(
    http_client: C,
    token: &AccessToken,
) -> Result<ValidatedToken, ValidationError<RE>>
where
    RE: std::error::Error + Send + Sync + 'static,
    C: FnOnce(HttpRequest) -> F,
    F: Future<Output = Result<HttpResponse, RE>>,
{
    use http::StatusCode;
    use oauth2::http::{header::AUTHORIZATION, HeaderMap, Method};

    let auth_header = format!("OAuth {}", token.secret());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        auth_header
            .parse()
            .expect("Failed to parse header for validation"),
    );
    let req = HttpRequest {
        url: Url::parse("https://id.twitch.tv/oauth2/validate")
            .expect("unexpectedly failed to parse validate url"),
        method: Method::GET,
        headers,
        body: vec![],
    };

    let resp = http_client(req).await.map_err(ValidationError::Request)?;
    match StatusCode::from_u16(resp.status_code.as_u16()) {
        Ok(status) if status.is_success() => Ok(serde_json::from_slice(&resp.body)?),
        Ok(status) if status == StatusCode::UNAUTHORIZED => Err(ValidationError::NotAuthorized),
        Ok(status) => {
            // TODO: Document this with a log call
            Err(ValidationError::TwitchError(TwitchTokenErrorResponse {
                status,
                message: String::from_utf8_lossy(&resp.body).into_owned(),
            }))
        }
        Err(_) => {
            unreachable!("converting from different http versions for the statuscode failed...")
        }
    }
}

/// Revoke the token.
///
/// See <https://dev.twitch.tv/docs/authentication#revoking-access-tokens>
pub async fn revoke_token<RE, C, F>(
    http_client: C,
    token: &AccessToken,
    client_id: &ClientId,
) -> Result<(), RevokeTokenError<RE>>
where
    RE: std::error::Error + Send + Sync + 'static,
    C: FnOnce(HttpRequest) -> F,
    F: Future<Output = Result<HttpResponse, RE>>,
{
    use oauth2::http::{HeaderMap, Method, StatusCode};
    use std::collections::HashMap;
    let mut params = HashMap::new();
    params.insert("client_id", client_id.as_str());
    params.insert("token", token.secret());
    let req = HttpRequest {
        url: Url::parse_with_params("https://id.twitch.tv/oauth2/revoke", &params)
            .expect("unexpectedly failed to parse revoke url"),
        method: Method::POST,
        headers: HeaderMap::new(),
        body: vec![],
    };

    let resp = http_client(req)
        .await
        .map_err(RevokeTokenError::RequestError)?;
    match resp.status_code {
        StatusCode::BAD_REQUEST => Err(RevokeTokenError::BadRequest(
            String::from_utf8(resp.body)
                .expect("couldn't parse 400 result for revoke as utf8... wow twitch"),
        )),
        StatusCode::OK => Ok(()),
        _ => Err(RevokeTokenError::Other(resp)),
    }
}

/// Refresh the token, call if it has expired.
///
/// See <https://dev.twitch.tv/docs/authentication#refreshing-access-tokens>
pub async fn refresh_token<RE, C, F>(
    http_client: C,
    refresh_token: RefreshToken,
    client_id: &ClientId,
    client_secret: &ClientSecret,
) -> Result<
    (
        AccessToken,
        Option<std::time::Instant>,
        Option<RefreshToken>,
    ),
    RefreshTokenError<RE>,
>
where
    RE: std::error::Error + Send + Sync + 'static,
    C: FnOnce(HttpRequest) -> F,
    F: Future<Output = Result<HttpResponse, RE>>,
{
    let now = std::time::Instant::now();

    let client = TwitchClient::new(
        client_id.clone(),
        Some(client_secret.clone()),
        AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_owned())
            .expect("unexpected failure to parse auth url for app_access_token"),
        Some(oauth2::TokenUrl::new(
            "https://id.twitch.tv/oauth2/token".to_string(),
        )?),
    );
    let client = client.set_auth_type(oauth2::AuthType::RequestBody);
    let client = client.exchange_refresh_token(&refresh_token);
    let res = client
        .request_async(http_client)
        .await
        .map_err(RefreshTokenError::RequestError)?;
    let refresh_token = res.refresh_token().cloned();
    let expires = res.expires_in().map(|dur| now + dur);
    let access_token = res.access_token;
    Ok((access_token, expires, refresh_token))
}

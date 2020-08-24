//! Errors

use crate::id::TwitchTokenErrorResponse;
use oauth2::HttpResponse as OAuth2HttpResponse;
use oauth2::RequestTokenError;
/// General errors for talking with twitch, currently only used in [AppAccessToken::get_app_access_token][crate::tokens::AppAccessToken::get_app_access_token]
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum TokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// request for token failed. {0}
    Request(RequestTokenError<RE, TwitchTokenErrorResponse>),
    /// could not parse url
    ParseError(#[from] oauth2::url::ParseError),
    /// could not get validation for token
    ValidationError(#[from] ValidationError<RE>),
}

/// Errors for [validate_token][crate::validate_token]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum ValidationError<RE: std::error::Error> {
    /// deserializations failed
    DeserializeError(#[from] serde_json::Error),
    /// token is not authorized for use
    NotAuthorized,
    /// twitch returned an unexpected status: {0}
    TwitchError(TwitchTokenErrorResponse),
    /// failed to request validation: {0}
    Request(RE),
}

/// Errors for [revoke_token][crate::revoke_token]
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum RevokeTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// 400 Bad Request: {0}
    BadRequest(String),
    /// failed to do revokation: {0}
    RequestError(RE),
    /// got unexpected return: {0:?}
    Other(OAuth2HttpResponse),
}

/// Errors for [TwitchToken::refresh_token][crate::TwitchToken::refresh_token]
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum RefreshTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// request for token failed. {0}
    RequestError(RequestTokenError<RE, TwitchTokenErrorResponse>),
    /// could not parse url
    ParseError(#[from] oauth2::url::ParseError),
    /// no refresh token found
    NoRefreshToken,
}

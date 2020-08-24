use crate::tokens::{
    errors::{RefreshTokenError, ValidationError},
    Scope, TwitchToken,
};
use oauth2::{AccessToken, ClientId, RefreshToken};
use oauth2::{HttpRequest, HttpResponse};
use std::future::Future;

/// An User Token from the [OAuth implicit code flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-implicit-code-flow) or [OAuth authorization code flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-authorization-code-flow)
#[derive(Debug, Clone)]
pub struct UserToken {
    access_token: AccessToken,
    client_id: ClientId,
    login: Option<String>,
    refresh_token: Option<RefreshToken>,
    expires: Option<std::time::Instant>,
    scopes: Vec<Scope>,
}

impl UserToken {
    /// Assemble token without checks.
    pub fn from_existing_unchecked(
        access_token: impl Into<AccessToken>,
        refresh_token: impl Into<Option<RefreshToken>>,
        client_id: impl Into<ClientId>,
        login: Option<String>,
        scopes: Option<Vec<Scope>>,
    ) -> UserToken
    {
        UserToken {
            access_token: access_token.into(),
            client_id: client_id.into(),
            login,
            refresh_token: refresh_token.into(),
            expires: None,
            scopes: scopes.unwrap_or_else(Vec::new),
        }
    }

    /// Assemble token and validate it. Retrieves [`login`](TwitchToken::login), [`client_id`](TwitchToken::client_id) and [`scopes`](TwitchToken::scopes)
    pub async fn from_existing<RE, C, F>(
        http_client: C,
        access_token: AccessToken,
        refresh_token: impl Into<Option<RefreshToken>>,
    ) -> Result<UserToken, ValidationError<RE>>
    where
        RE: std::error::Error + 'static,
        C: FnOnce(HttpRequest) -> F,
        F: Future<Output = Result<HttpResponse, RE>>,
    {
        let validated = crate::validate_token(http_client, &access_token).await?;
        Ok(Self::from_existing_unchecked(
            access_token,
            refresh_token.into(),
            validated.client_id,
            validated.login,
            validated.scopes,
        ))
    }
}

#[async_trait::async_trait(?Send)]
impl TwitchToken for UserToken {
    fn client_id(&self) -> &ClientId { &self.client_id }

    fn token(&self) -> &AccessToken { &self.access_token }

    fn login(&self) -> Option<&str> { self.login.as_deref() }

    async fn refresh_token<RE, C, F>(&mut self, _: C) -> Result<(), RefreshTokenError<RE>>
    where
        RE: std::error::Error + 'static,
        C: FnOnce(HttpRequest) -> F,
        F: Future<Output = Result<HttpResponse, RE>>, {
        Err(RefreshTokenError::NoRefreshToken)
    }

    fn expires(&self) -> Option<std::time::Instant> { None }

    fn scopes(&self) -> Option<&[Scope]> { Some(self.scopes.as_slice()) }
}

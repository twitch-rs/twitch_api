use super::errors::{TokenError, ValidationError};
use crate::{
    id::TwitchClient,
    tokens::{errors::RefreshTokenError, Scope, TwitchToken},
};
use oauth2::{AccessToken, AuthUrl, ClientId, ClientSecret, RefreshToken, TokenResponse};
use oauth2::{HttpRequest, HttpResponse};
use std::future::Future;

/// An App Access Token from the [OAuth client credentials flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-client-credentials-flow)
#[derive(Debug, Clone)]
pub struct AppAccessToken {
    access_token: AccessToken,
    refresh_token: Option<RefreshToken>,
    expires: Option<std::time::Instant>,
    client_id: ClientId,
    client_secret: ClientSecret,
    login: Option<String>,
    scopes: Option<Vec<Scope>>,
}

#[async_trait::async_trait(?Send)]
impl TwitchToken for AppAccessToken {
    fn client_id(&self) -> &ClientId { &self.client_id }

    fn token(&self) -> &AccessToken { &self.access_token }

    fn login(&self) -> Option<&str> { self.login.as_deref() }

    async fn refresh_token<RE, C, F>(
        &mut self,
        http_client: C,
    ) -> Result<(), RefreshTokenError<RE>>
    where
        RE: std::error::Error + Send + Sync + 'static,
        C: FnOnce(HttpRequest) -> F,
        F: Future<Output = Result<HttpResponse, RE>>,
    {
        let (access_token, expires, refresh_token) = if let Some(token) = self.refresh_token.take()
        {
            crate::refresh_token(http_client, token, &self.client_id, &self.client_secret).await?
        } else {
            return Err(RefreshTokenError::NoRefreshToken);
        };
        self.access_token = access_token;
        self.expires = expires;
        self.refresh_token = refresh_token;
        Ok(())
    }

    fn expires(&self) -> Option<std::time::Instant> { self.expires }

    fn scopes(&self) -> Option<&[Scope]> { self.scopes.as_deref() }
}

impl AppAccessToken {
    /// Assemble token without checks.
    pub fn from_existing_unchecked(
        access_token: AccessToken,
        client_id: impl Into<ClientId>,
        client_secret: impl Into<ClientSecret>,
        login: Option<String>,
        scopes: Option<Vec<Scope>>,
    ) -> AppAccessToken
    {
        AppAccessToken {
            access_token,
            refresh_token: None,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            login,
            expires: None,
            scopes,
        }
    }

    /// Assemble token and validate it. Retrieves [`client_id`](TwitchToken::client_id) and [`scopes`](TwitchToken::scopes).
    pub async fn from_existing<RE, C, F>(
        http_client: C,
        access_token: AccessToken,
        client_secret: ClientSecret,
    ) -> Result<AppAccessToken, ValidationError<RE>>
    where
        RE: std::error::Error + Send + Sync + 'static,
        C: FnOnce(HttpRequest) -> F,
        F: Future<Output = Result<HttpResponse, RE>>,
    {
        let token = access_token;
        let validated = crate::validate_token(http_client, &token).await?;
        Ok(Self::from_existing_unchecked(
            token,
            validated.client_id,
            client_secret,
            None,
            validated.scopes,
        ))
    }

    /// Generate app access token via [OAuth client credentials flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-client-credentials-flow)
    pub async fn get_app_access_token<RE, C, F>(
        http_client: C,
        client_id: ClientId,
        client_secret: ClientSecret,
        scopes: Vec<Scope>,
    ) -> Result<AppAccessToken, TokenError<RE>>
    where
        RE: std::error::Error + Send + Sync + 'static,
        C: Fn(HttpRequest) -> F,
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
        let mut client = client.exchange_client_credentials();
        for scope in scopes {
            client = client.add_scope(scope.as_oauth_scope());
        }
        let response = client
            .request_async(&http_client)
            .await
            .map_err(TokenError::Request)?;

        let app_access = AppAccessToken {
            access_token: response.access_token().clone(),
            refresh_token: response.refresh_token().cloned(),
            expires: response.expires_in().map(|dur| now + dur),
            client_id,
            client_secret,
            login: None,
            scopes: response
                .scopes()
                .cloned()
                .map(|s| s.into_iter().map(|s| s.into()).collect()),
        };

        let _ = app_access.validate_token(http_client).await?; // Sanity check
        Ok(app_access)
    }
}

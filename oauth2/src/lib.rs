#![deny(intra_doc_link_resolution_failure)]
#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/twitch_oauth2/0.1.0")]
//! [![github]](https://github.com/emilgardis/twitch_oauth2)&ensp;[![crates-io]](https://crates.io/crates/twitch_oauth2)&ensp;[![docs-rs]](https://docs.rs/twitch_oauth2)
//!
//! [github]: https://img.shields.io/badge/github-emilgardis/twitch__utils-8da0cb?style=for-the-badge&labelColor=555555&logo=github"
//! [crates-io]: https://img.shields.io/crates/v/twitch_oauth2.svg?style=for-the-badge&color=fc8d62&logo=rust"
//! [docs-rs]: https://img.shields.io/badge/docs.rs-twitch_oauth2-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K"
//!
//! <br>
//!
//! <h5>OAuth2 for Twitch endpoints</h5>
//!
//! ```rust,ignore
//!     use twitch_oauth2::TwitchToken;
//!     let user_token = UserToken::from_existing("sometokenherewhichisvalidornot").await.unwrap();
//!     println!("user_token: {}", user_token.token().secret())
//! ```

use oauth2::helpers;
use oauth2::{
    basic::{BasicErrorResponse, BasicErrorResponseType, BasicTokenType},
    reqwest::{self, async_http_client},
    url::Url,
    AccessToken, AsyncClientCredentialsTokenRequest, AsyncRefreshTokenRequest, AuthUrl, Client,
    ClientId, ClientSecret, ExtraTokenFields, RefreshToken, RequestTokenError,
    StandardErrorResponse, TokenResponse, TokenType,
};

use serde::{Deserialize, Serialize};

use std::time::Duration;
use thiserror::Error;

/// Scopes for twitch.
///
/// https://dev.twitch.tv/docs/authentication/#scopes
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Scope {
    /// View analytics data for your extensions.
    #[serde(rename = "analytics:read:extensions")]
    AnalyticsReadExtensions,
    /// Manage a user object.
    #[serde(rename = "user:edit")]
    UserEdit,
    /// Read authorized user's email address.
    #[serde(rename = "user:read:email")]
    UserReadEmail,
    /// Read authorized user’s stream key.
    ///
    /// # Note:
    /// This scope seems to not work, even though it is documented.
    #[serde(rename = "user:read:stream_key")]
    UserReadStreamKey,
    /// Create and edit clips as a specific user.
    #[serde(rename = "clips:edit")]
    ClipsEdit,
    /// View bits information for your channel.
    #[serde(rename = "bits:read")]
    BitsRead,
    /// View analytics data for your games.
    #[serde(rename = "analytics:read:games")]
    AnalyticsReadGames,
    /// Edit your channel's broadcast configuration, including extension configuration. (This scope implies user:read:broadcast capability.)
    #[serde(rename = "user:edit:broadcast")]
    UserEditBroadcast,
    /// View your broadcasting configuration, including extension configurations.
    #[serde(rename = "user:read:broadcast")]
    UserReadBroadcast,
    /// View live Stream Chat and Rooms messages
    #[serde(rename = "chat:read")]
    ChatRead,
    /// Send live Stream Chat and Rooms messages
    #[serde(rename = "chat:edit")]
    ChatEdit,
    /// Perform moderation actions in a channel
    #[serde(rename = "channel:moderate")]
    ChannelModerate,
    /// Get a list of all subscribers to your channel and check if a user is subscribed to your channel
    #[serde(rename = "channel:read:subscriptions")]
    ChannelReadSubscriptions,
    // FIXME: Documentation.
    ///
    #[serde(rename = "channel:read:hype_train")]
    ChannelReadHypeTrain,
    /// View your whisper messages.
    #[serde(rename = "whispers:read")]
    WhispersRead,
    /// Send whisper messages.
    #[serde(rename = "whispers:edit")]
    WhispersEdit,
    /// View your channel's moderation data including Moderators, Bans, Timeouts and Automod settings
    #[serde(rename = "moderation:read")]
    ModerationRead,
    /// View your channel points custom reward redemptions
    #[serde(rename = "channel:read:redemptions")]
    ChannelReadRedemptions,
    /// Other scope that is not implemented.
    Other(String),
}

impl Scope {
    /// Get [Scope] as [oauth2::Scope]
    pub fn as_oauth_scope(&self) -> oauth2::Scope {
        use self::Scope::*;
        let s = match self {
            AnalyticsReadExtensions => "analytics:read:extensions".to_string(),
            UserEdit => "user:edit".to_string(),
            UserReadEmail => "user:read:email".to_string(),
            UserReadStreamKey => "user:read:stream_key".to_string(),
            ClipsEdit => "clips:edit".to_string(),
            BitsRead => "bits:read".to_string(),
            AnalyticsReadGames => "analytics:read:games".to_string(),
            UserEditBroadcast => "user:edit:broadcast".to_string(),
            UserReadBroadcast => "user:read:broadcast".to_string(),
            ChatRead => "chat:read".to_string(),
            ChatEdit => "chat:edit".to_string(),
            ChannelModerate => "channel:moderate".to_string(),
            ChannelReadSubscriptions => "channel:read:subscriptions".to_string(),
            ChannelReadHypeTrain => "channel:read:hype_train".to_string(),
            WhispersRead => "whispers:read".to_string(),
            WhispersEdit => "whispers:edit".to_string(),
            ModerationRead => "moderation:read".to_string(),
            ChannelReadRedemptions => "channel:read:redemptions".to_string(),
            Other(s) => s.clone(),
        };
        oauth2::Scope::new(s)
    }

    /// Get a vec of all defined twitch [Scopes][Scope]
    pub fn all() -> Vec<Scope> {
        vec![
            Scope::AnalyticsReadExtensions,
            Scope::UserEdit,
            Scope::UserReadEmail,
            //Scope::UserReadStreamKey, // Broken?
            Scope::ClipsEdit,
            Scope::BitsRead,
            Scope::AnalyticsReadGames,
            Scope::UserEditBroadcast,
            Scope::UserReadBroadcast,
            Scope::ChatRead,
            Scope::ChatEdit,
            Scope::ChannelModerate,
            Scope::ChannelReadSubscriptions,
            Scope::ChannelReadHypeTrain,
            Scope::WhispersRead,
            Scope::WhispersEdit,
            Scope::ModerationRead,
            Scope::ChannelReadRedemptions,
        ]
    }
}

impl From<oauth2::Scope> for Scope {
    fn from(scope: oauth2::Scope) -> Self {
        use self::Scope::*;
        match scope.as_str() {
            "analytics:read:extensions" => AnalyticsReadExtensions,
            "user:edit" => UserEdit,
            "user:read:email" => UserReadEmail,
            "user:read:stream_key" => Scope::UserReadStreamKey,
            "clips:edit" => ClipsEdit,
            "bits:read" => BitsRead,
            "analytics:read:games" => AnalyticsReadGames,
            "user:edit:broadcast" => UserEditBroadcast,
            "user:read:broadcast" => UserReadBroadcast,
            "chat:read" => ChatRead,
            "chat:edit" => ChatEdit,
            "channel:moderate" => ChannelModerate,
            "channel:read:subscriptions" => ChannelReadSubscriptions,
            "channel:read:hype_train" => ChannelReadHypeTrain,
            "whispers:read" => WhispersRead,
            "whispers:edit" => WhispersEdit,
            "moderation:read" => ModerationRead,
            "channel:read:redemptions" => ChannelReadRedemptions,
            s => Other(s.to_string()),
        }
    }
}

/// Trait for twitch tokens to get fields and generalize over [AppAccessToken] and [UserToken]
#[async_trait::async_trait]
pub trait TwitchToken {
    /// Get the client-id. Twitch requires this in all helix api calls
    fn client_id(&self) -> &str;
    /// Get the [AccessToken] for authenticating
    fn token(&self) -> &AccessToken;
    /// Get the username associated to this token
    fn login(&self) -> Option<&str>;
    /// Refresh this token, changing the token to a newer one
    async fn refresh_token(&mut self) -> Result<(), RefreshTokenError>;
    /// Get instant when token will expire.
    fn expires(&self) -> Option<std::time::Instant>;
    /// Validate this token. Should be checked repeatedly.
    async fn validate_token(&self) -> Result<ValidatedToken, ValidationError>
    where Self: Sized {
        validate_token(&self.token()).await
    }
}

/// Token validation returned from [`https://id.twitch.tv/oauth2/validate`](https://dev.twitch.tv/docs/authentication#validating-requests)
#[derive(Debug, Clone, Deserialize)]
pub struct ValidatedToken {
    /// Client ID associated to the token. Twitch requires this in all helix api calls
    pub client_id: String,
    /// Username associated to the token
    pub login: Option<String>,
    /// User ID associated to the token
    pub user_id: Option<String>,
    /// Scopes attached to the token.
    pub scopes: Option<Vec<Scope>>,
}

/// An App Access Token from the [OAuth client credentials flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-client-credentials-flow)
#[derive(Debug, Clone)]
pub struct AppAccessToken {
    access_token: AccessToken,
    refresh_token: Option<RefreshToken>,
    expires: Option<std::time::Instant>,
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    login: Option<String>,
    scopes: Option<Vec<Scope>>,
}

#[async_trait::async_trait]
impl TwitchToken for AppAccessToken {
    fn client_id(&self) -> &str { &self.client_id }

    fn token(&self) -> &AccessToken { &self.access_token }

    fn login(&self) -> Option<&str> { self.login.as_deref() }

    async fn refresh_token(&mut self) -> Result<(), RefreshTokenError> {
        self.refresh_token().await
    }

    fn expires(&self) -> Option<std::time::Instant> { self.expires }
}
/// Errors for [AppAccessToken::get_app_access_token]
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum TokenError {
    #[error("request for token failed. {0}")]
    RequestError(
        RequestTokenError<
            reqwest::AsyncHttpClientError,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    ParseError(#[from] oauth2::url::ParseError),
    #[error("could not get validation for token")]
    ValidationError(#[from] ValidationError),
    #[error("?")]
    Other,
}

/// Errors for [validate_token]
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("failed to do validation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
}

impl AppAccessToken {
    /// Assemble token without checks.
    pub fn from_existing_unchecked(
        access_token: String,
        client_id: String,
        client_secret: Option<String>,
        login: Option<String>,
        scopes: Option<Vec<Scope>>,
    ) -> AppAccessToken
    {
        AppAccessToken {
            access_token: AccessToken::new(access_token),
            refresh_token: None,
            client_id: ClientId::new(client_id),
            client_secret: client_secret.map(ClientSecret::new),
            login,
            expires: None,
            scopes,
        }
    }

    /// Assemble token and validate it. Retrieves [`login`](AppAccessToken.login), [`client_id`](AppAccessToken.client_id) and [`scopes`](AppAccessToken.scopes), will not get client_secret.
    pub async fn from_existing(access_token: String) -> Result<AppAccessToken, ValidationError> {
        let token = AccessToken::new(access_token);
        let validated = validate_token(&token).await?;
        Ok(Self::from_existing_unchecked(
            token.secret().to_owned(),
            validated.client_id,
            None,
            validated.login,
            validated.scopes,
        ))
    }

    /// Set client secret.
    pub fn set_client_secret(&mut self, secret: ClientSecret) {
        self.client_secret.replace(secret);
    }

    /// Generate app access token via [OAuth client credentials flow](https://dev.twitch.tv/docs/authentication/getting-tokens-oauth#oauth-client-credentials-flow)
    pub async fn get_app_access_token(
        client_id: String,
        client_secret: String,
        scopes: Vec<Scope>,
    ) -> Result<AppAccessToken, TokenError>
    {
        let now = std::time::Instant::now();
        let client = TwitchClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
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
            .request_async(async_http_client)
            .await
            .map_err(TokenError::RequestError)?;

        let mut app_access = AppAccessToken {
            access_token: response.access_token().clone(),
            refresh_token: response.refresh_token().cloned(),
            expires: response.expires_in().map(|dur| now + dur),
            client_id: ClientId::new(client_id),
            client_secret: Some(ClientSecret::new(client_secret)),
            login: None,
            scopes: response
                .scopes()
                .cloned()
                .map(|s| s.into_iter().map(|s| s.into()).collect()),
        };

        let validated = app_access.validate_token().await?;
        app_access.login = validated.login;
        Ok(app_access)
    }

    /// Refresh the token, call if it has expired.
    ///
    /// Uses [https://dev.twitch.tv/docs/authentication#refreshing-access-tokens]
    pub async fn refresh_token(&mut self) -> Result<(), RefreshTokenError> {
        let now = std::time::Instant::now();
        let refresh_token = if let Some(ref token) = self.refresh_token {
            token
        } else {
            return Err(RefreshTokenError::NoRefreshToken);
        };
        let client = TwitchClient::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_owned())
                .expect("unexpected failure to parse auth url for app_access_token"),
            Some(oauth2::TokenUrl::new(
                "https://id.twitch.tv/oauth2/token".to_string(),
            )?),
        );
        let client = client.set_auth_type(oauth2::AuthType::RequestBody);
        let client = client.exchange_refresh_token(refresh_token);
        let res = client
            .request_async(async_http_client)
            .await
            .map_err(RefreshTokenError::RequestError)?;
        self.refresh_token = res.refresh_token().cloned();
        self.expires = res.expires_in().map(|dur| now + dur);
        self.access_token = res.access_token;
        Ok(())
    }

    /// Revoke the token, uses [https://dev.twitch.tv/docs/authentication#revoking-access-tokens]
    pub async fn revoke_token(self) -> Result<(), RevokeTokenError> {
        revoke_token(self.access_token, self.client_id).await
    }
}
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
        access_token: String,
        refresh_token: Option<String>,
        client_id: String,
        login: Option<String>,
        scopes: Option<Vec<Scope>>,
    ) -> UserToken
    {
        UserToken {
            access_token: AccessToken::new(access_token),
            client_id: ClientId::new(client_id),
            login,
            refresh_token: refresh_token.map(RefreshToken::new),
            expires: None,
            scopes: scopes.unwrap_or_else(Vec::new),
        }
    }

    /// Assemble token and validate it. Retrieves [`login`](UserToken.login), [`client_id`](UserToken.client_id) and [`scopes`](UserToken.scopes), will not get client_secret.
    pub async fn from_existing(
        access_token: impl Into<String>,
        refresh_token: impl Into<Option<String>>,
    ) -> Result<UserToken, ValidationError>
    {
        let token = AccessToken::new(access_token.into());
        let validated = validate_token(&token).await?;
        Ok(Self::from_existing_unchecked(
            token.secret().to_owned(),
            refresh_token.into(),
            validated.client_id,
            validated.login,
            validated.scopes,
        ))
    }
}

#[async_trait::async_trait]
impl TwitchToken for UserToken {
    fn client_id(&self) -> &str { &self.client_id }

    fn token(&self) -> &AccessToken { &self.access_token }

    fn login(&self) -> Option<&str> { self.login.as_deref() }

    async fn refresh_token(&mut self) -> Result<(), RefreshTokenError> {
        Err(RefreshTokenError::NoRefreshToken)
    }

    fn expires(&self) -> Option<std::time::Instant> { None }
}

/// Validate this token. Should be checked repeatedly.
pub async fn validate_token(token: &AccessToken) -> Result<ValidatedToken, ValidationError> {
    use oauth2::http::{header::AUTHORIZATION, HeaderMap, Method};
    let auth_header = format!("OAuth {}", token.secret());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        auth_header
            .parse()
            .expect("Failed to parse header for validation"),
    );
    let req = oauth2::HttpRequest {
        url: Url::parse("https://id.twitch.tv/oauth2/validate")
            .expect("unexpectedly failed to parse validate url"),
        method: Method::GET,
        headers,
        body: vec![],
    };

    let resp = oauth2::reqwest::async_http_client(req)
        .await
        .map_err(ValidationError::Reqwest)?
        .body;
    Ok(::serde_json::from_slice(&resp).unwrap())
}

/// Revoke the token, uses [https://dev.twitch.tv/docs/authentication#revoking-access-tokens]
pub async fn revoke_token(token: AccessToken, client_id: ClientId) -> Result<(), RevokeTokenError> {
    use oauth2::http::{HeaderMap, Method, StatusCode};
    use std::collections::HashMap;
    let mut params = HashMap::new();
    params.insert("client_id", client_id.as_str());
    params.insert("token", token.secret());
    let req = oauth2::HttpRequest {
        url: Url::parse_with_params("https://id.twitch.tv/oauth2/revoke", &params)
            .expect("unexpectedly failed to parse revoke url"),
        method: Method::GET,
        headers: HeaderMap::new(),
        body: vec![],
    };

    let resp = oauth2::reqwest::async_http_client(req)
        .await
        .map_err(RevokeTokenError::Reqwest)?;
    match resp.status_code {
        StatusCode::BAD_REQUEST => Err(RevokeTokenError::BadRequest(
            String::from_utf8(resp.body)
                .expect("couldn't parse 400 result for revoke as utf8... wow twitch"),
        )),
        StatusCode::OK => Ok(()),
        _ => Err(RevokeTokenError::Other(resp)),
    }
}

/// Errors for [revoke_token]
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum RevokeTokenError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    #[error("failed to do revokation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
    #[error("got unexpected return: {0:?}")]
    Other(oauth2::HttpResponse),
}

/// Errors for [TwitchToken::refresh_token]
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum RefreshTokenError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    #[error("failed to do refresh: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
    #[error("got unexpected return: {0:?}")]
    Other(oauth2::HttpResponse),
    #[error("request for token failed. {0}")]
    RequestError(
        RequestTokenError<
            reqwest::AsyncHttpClientError,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    ParseError(#[from] oauth2::url::ParseError),
    #[error("no refresh token found")]
    NoRefreshToken,
}

/// Twitch's representation of the oauth flow.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TwitchTokenResponse<EF, TT>
where
    TT: TokenType,
    EF: ExtraTokenFields, {
    access_token: AccessToken,
    #[serde(bound = "TT: TokenType")]
    #[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
    token_type: TT,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<oauth2::RefreshToken>,
    #[serde(rename = "scope")]
    #[serde(default)]
    scopes: Option<Vec<oauth2::Scope>>,
    #[serde(bound = "EF: ExtraTokenFields")]
    #[serde(flatten)]
    extra_fields: EF,
}

impl<EF, TT> oauth2::TokenResponse<TT> for TwitchTokenResponse<EF, TT>
where
    TT: TokenType,
    EF: ExtraTokenFields,
{
    fn access_token(&self) -> &AccessToken { &self.access_token }

    fn token_type(&self) -> &TT { &self.token_type }

    fn expires_in(&self) -> Option<Duration> { self.expires_in.map(Duration::from_secs) }

    fn refresh_token(&self) -> Option<&oauth2::RefreshToken> { self.refresh_token.as_ref() }

    fn scopes(&self) -> Option<&Vec<oauth2::Scope>> { self.scopes.as_ref() }
}

/// Client for Twitch OAuth2
pub type TwitchClient = Client<
    BasicErrorResponse,
    TwitchTokenResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
>;

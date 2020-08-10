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
    Other(String),
}

impl Scope {
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

#[async_trait::async_trait]
pub trait TwitchToken {
    fn client_id(&self) -> &str;
    fn token(&self) -> &AccessToken;
    fn login(&self) -> Option<&str>;
    async fn refresh_token(&mut self) -> Result<(), RefreshTokenError>;
    fn expires(&self) -> Option<std::time::Instant>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidatedToken {
    pub client_id: String,
    pub login: Option<String>,
    pub user_id: Option<String>,
    pub scopes: Option<Vec<Scope>>,
}

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

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("failed to do validation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
}

impl AppAccessToken {
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

    pub async fn validate_token(&self) -> Result<ValidatedToken, ValidationError> {
        validate_token(&self.access_token).await
    }

    pub async fn revoke_token(self) -> Result<(), RevokeTokenError> {
        revoke_token(self.access_token, self.client_id).await
    }
}

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

    pub async fn from_existing(
        access_token: String,
        refresh_token: Option<String>,
    ) -> Result<UserToken, ValidationError>
    {
        let token = AccessToken::new(access_token);
        let validated = validate_token(&token).await?;
        Ok(Self::from_existing_unchecked(
            token.secret().to_owned(),
            refresh_token,
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
async fn revoke_token(token: AccessToken, client_id: ClientId) -> Result<(), RevokeTokenError> {
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

#[derive(Error, Debug)]
pub enum RevokeTokenError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    #[error("failed to do revokation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
    #[error("got unexpected return: {0:?}")]
    Other(oauth2::HttpResponse),
}

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

pub type TwitchClient = Client<
    BasicErrorResponse,
    TwitchTokenResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
>;

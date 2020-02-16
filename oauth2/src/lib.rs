use oauth2::{
    basic::{BasicClient, BasicErrorResponseType},
    reqwest::{self, async_http_client},
    url::Url,
    AccessToken, AsyncClientCredentialsTokenRequest, AuthUrl, ClientId, ClientSecret, RedirectUrl,
    RequestTokenError, Scope, StandardErrorResponse, StandardTokenResponse, TokenResponse,
};

use serde::Deserialize;

use thiserror::Error;

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
    expires_in: Option<std::time::Duration>,
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    scopes: Option<Vec<Scope>>,
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Request for token failed. {0}")]
    RequestError(RequestTokenError<reqwest::AsyncHttpClientError,
                                    StandardErrorResponse<BasicErrorResponseType>>),
    #[error("?")]
    Other,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Failed to do do validation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
}

impl AppAccessToken {
    pub fn from_existing_unchecked(access_token: String, client_id: String,
                                   client_secret: Option<String>, scopes: Option<Vec<Scope>>)
                                   -> AppAccessToken {
        AppAccessToken { access_token: AccessToken::new(access_token),
                         client_id: ClientId::new(client_id),
                         client_secret: client_secret.map(|s| ClientSecret::new(s)),
                         expires_in: None,
                         scopes }
    }
    pub async fn from_existing(access_token: String) -> Result<AppAccessToken, ValidationError> {
        let token = AccessToken::new(access_token);
        let validated = _validate_token(&token).await?;
        Ok(Self::from_existing_unchecked(token.secret().to_owned(),
                                         validated.client_id,
                                         None,
                                         validated.scopes))
    }
    pub async fn get_app_access_token(client_id: String, client_secret: String,
                                      scopes: Vec<String>)
                                      -> Result<AppAccessToken, TokenError> {
        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new("https://id.twitch.tv/oauth2/token".to_owned())
                .expect("unexpected failure to parse auth url for app_access_token"),
            None,
        );
        let mut client = client.exchange_client_credentials();
        for scope in scopes {
            client = client.add_scope(Scope::new(scope));
        }
        let response = client.request_async(async_http_client)
                             .await
                             .map_err(|e| TokenError::RequestError(e))?;

        Ok(AppAccessToken { access_token: response.access_token().clone(),
                            expires_in: response.expires_in(),
                            scopes: response.scopes().cloned(),
                            client_secret: Some(ClientSecret::new(client_secret)),
                            client_id: ClientId::new(client_id) })
    }
    pub async fn validate_token(&self) -> Result<ValidatedToken, ValidationError> {
        _validate_token(&self.access_token).await
    }

    pub async fn revoke_token(self) -> Result<(), RevokeError> {
        _revoke_token(self.access_token, self.client_id).await
    }
}

#[derive(Debug, Clone)]
pub struct UserToken {
    access_token: String,
    refresh_token: String,
    expires_in: usize,
    scope: Vec<String>,
}

impl UserToken {
    pub async fn get_authorize(client_id: String, redirect: Url, scopes: Vec<String>,
                               force_verify: bool, state: Option<String>)
                               -> Result<UserToken, ()> {
        //let url = Url::parse_with_params("https://id.twitch.tv/oauth2/authorize").unwrap();

        todo!()
    }
}

async fn _validate_token(token: &AccessToken) -> Result<ValidatedToken, ValidationError> {
    use oauth2::http::{header::AUTHORIZATION, HeaderMap, Method};
    let auth_header = format!("OAuth {}", token.secret());
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION,
                   auth_header.parse()
                              .expect("Failed to parse header for validation"));
    let req = oauth2::HttpRequest {
        url: Url::parse("https://id.twitch.tv/oauth2/validate")
            .expect("unexpectedly failed to parse validate url"),
        method: Method::GET,
        headers,
        body: vec![],
    };

    let resp = oauth2::reqwest::async_http_client(req).await
                                                      .map_err(|e| ValidationError::Reqwest(e))?
                                                      .body;
    Ok(::serde_json::from_slice(&resp).unwrap())
}
async fn _revoke_token(token: AccessToken, client_id: ClientId) -> Result<(), RevokeError> {
    use oauth2::http::{HeaderMap, Method, StatusCode};
    use std::collections::HashMap;
    let mut params = HashMap::new();
    params.insert("client_id", client_id.as_str());
    params.insert("token", token.secret());
    let req = oauth2::HttpRequest {
        url: Url::parse_with_params("https://id.twitch.tv/oauth2/revoke", &params)
            .expect("unexpectedly failed to parse validate url"),
        method: Method::GET,
        headers: HeaderMap::new(),
        body: vec![],
    };

    let resp = oauth2::reqwest::async_http_client(req).await
                                                      .map_err(|e| RevokeError::Reqwest(e))?;
    match resp.status_code {
        StatusCode::BAD_REQUEST => Err(RevokeError::BadRequest(
            String::from_utf8(resp.body)
                .expect("couldn't parse 400 result for revoke as utf8... wow twitch"),
        )),
        StatusCode::OK => Ok(()),
        _ => Err(RevokeError::Other(resp.clone())),
    }
}

#[derive(Error, Debug)]
pub enum RevokeError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),
    #[error("Failed to do do revokation: {0}")]
    Reqwest(reqwest::AsyncHttpClientError),
    #[error("Got unexpected return: {0:?}")]
    Other(oauth2::HttpResponse),
}

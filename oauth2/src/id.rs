//! Representation of oauth2 flow in `id.twitch.tv`

use serde::{Deserialize, Serialize};

use oauth2::helpers;
use oauth2::{
    basic::BasicTokenType, Client, ExtraTokenFields, TokenType,
};

use crate::AccessToken;
use std::time::Duration;
/// Twitch's representation of the oauth flow.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TwitchTokenResponse<EF, TT>
where
    TT: TokenType,
    EF: ExtraTokenFields, {
    /// Access token
    pub access_token: AccessToken,
    /// Token type
    #[serde(bound = "TT: TokenType")]
    #[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
    pub token_type: TT,
    /// Time (in seconds) until token expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u64>,
    /// Token that can be used to refresh
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<oauth2::RefreshToken>,
    /// Scopes attached to token
    #[serde(rename = "scope")]
    #[serde(default)]
    pub scopes: Option<Vec<oauth2::Scope>>,
    /// Extras
    #[serde(bound = "EF: ExtraTokenFields")]
    #[serde(flatten)]
    pub extra_fields: EF,
}

/// Twitch's representation of the oauth flow for errors
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TwitchTokenErrorResponse {
    /// Status code of error
    #[serde(with = "status_code")]
    pub status: http::StatusCode,
    /// Message attached to error
    pub message: String,
}

#[doc(hidden)]
pub mod status_code {
    use http::StatusCode;
    use serde::{
        de::{Deserialize, Error, Unexpected},
        Deserializer, Serializer,
    };

    pub fn deserialize<'de, D>(de: D) -> Result<StatusCode, D::Error>
    where D: Deserializer<'de> {
        let code: u16 = Deserialize::deserialize(de)?;
        match StatusCode::from_u16(code) {
            Ok(code) => Ok(code),
            Err(_) => Err(Error::invalid_value(
                Unexpected::Unsigned(code as u64),
                &"a value between 100 and 600",
            )),
        }
    }

    pub fn serialize<S>(status: &StatusCode, ser: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        ser.serialize_u16(status.as_u16())
    }
}

impl std::fmt::Display for TwitchTokenErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status.as_u16(), self.message)
    }
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

impl oauth2::ErrorResponse for TwitchTokenErrorResponse {}

/// Client for Twitch OAuth2
pub type TwitchClient = Client<
    TwitchTokenErrorResponse,
    TwitchTokenResponse<oauth2::EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
>;

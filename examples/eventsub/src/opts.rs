use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(about, version)]
pub struct Opts {
    /// Client ID of twitch application
    #[clap(long, env, hide_env = true)]
    pub client_id: twitch_oauth2::ClientId,
    /// Client Secret of twitch application
    #[clap(long, env, hide_env = true)]
    pub client_secret: twitch_oauth2::ClientSecret,
    #[clap(long, env, hide_env = true)]
    pub sign_secret: SignSecret,
    #[clap(long, env, hide_env = true)]
    pub broadcaster_login: twitch_api::types::UserName,
    #[clap(long, env, hide_env = true)]
    pub website_callback: String,
    #[clap(long, env, hide_env = true)]
    pub website: String,
}

#[derive(Clone)]
pub struct SignSecret {
    secret: String,
}

impl SignSecret {
    /// Get a reference to the sign secret.
    pub fn secret(&self) -> &[u8] { self.secret.as_bytes() }

    pub fn secret_str(&self) -> &str { &self.secret }
}

impl std::fmt::Debug for SignSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignSecret")
            .field("secret", &"[redacted]")
            .finish()
    }
}

impl std::str::FromStr for SignSecret {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SignSecret {
            secret: s.to_string(),
        })
    }
}

pub mod websocket;

use std::sync::Arc;

use clap::Parser;
use eyre::WrapErr as _;
use tokio::sync::Mutex;

use twitch_api::{
    client::ClientDefault,
    eventsub::{self, Event, Message, Payload},
    HelixClient,
};
use twitch_oauth2::{Scope, TwitchToken as _, UserToken};

/// The scopes we need for the bot.
const SCOPES: &[Scope] = &[Scope::UserReadChat, Scope::UserWriteChat];

/// How often we should check if the token is still valid.
const TOKEN_VALIDATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
/// The threshold at which we should refresh the token before expiration.
///
/// Only checked every [TOKEN_VALIDATION_INTERVAL] seconds
const TOKEN_EXPIRATION_THRESHOLD: std::time::Duration = std::time::Duration::from_secs(60);

#[derive(Parser, Debug, Clone)]
#[clap(about, version)]
pub struct Cli {
    /// Client ID of twitch application
    #[clap(long, env, hide_env = true)]
    pub client_id: twitch_oauth2::ClientId,
    /// Chat to connect to, can take multiple values separated by commas
    #[clap(long, env, value_delimiter = ',', hide_env = true)]
    pub broadcaster_login: Vec<twitch_api::types::UserName>,
    /// Path to config file
    #[clap(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"))]
    pub config: std::path::PathBuf,
    /// Path to token file
    #[clap(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/auth.toml"))]
    pub auth: std::path::PathBuf,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    command: Vec<Command>,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize, Debug)]
pub struct Command {
    pub trigger: String,
    pub response: String,
}

impl Config {
    pub fn load(path: &std::path::Path) -> Result<Self, eyre::Report> {
        let config = std::fs::read_to_string(path)?;
        toml::from_str(&config).wrap_err("Failed to parse config")
    }
}

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::io::stderr)
        .init();
    _ = dotenvy::dotenv();
    let opts = Cli::parse();
    let config = Config::load(&opts.config)?;

    let client: HelixClient<reqwest::Client> = twitch_api::HelixClient::with_client(
        ClientDefault::default_client_with_name(Some("my_chatbot".parse()?))?,
    );

    // Get an user access token.
    // For this example we store the token in a file, but you should probably store it in a database or similar.
    // If there is no token saved, we use Device Code Flow to get a token.
    // This flow works best with public client type applications.
    // If you have a confidential client type application you should use `UserTokenBuilder` for OAuth authorization code flow.
    let token = if let Some(token) = load_token(&opts.auth, &client).await? {
        token
    } else {
        let mut builder = twitch_oauth2::tokens::DeviceUserTokenBuilder::new(
            opts.client_id.clone(),
            SCOPES.to_vec(),
        );
        let code = builder.start(&client).await?;
        println!("Please go to: {}", code.verification_uri);
        builder.wait_for_code(&client, tokio::time::sleep).await?
    };
    save_token(&token, &opts.auth)?;
    let token = Arc::new(Mutex::new(token));

    // Get the broadcaster ids from the logins.
    let mut broadcasters = vec![];
    for login in opts.broadcaster_login.iter() {
        if let Some(twitch_api::helix::users::User { id, .. }) =
            client.get_user_from_login(login, &token).await?
        {
            broadcasters.push(id);
        } else {
            eyre::bail!("No broadcaster found with login: {}", login);
        }
    }

    // Create the bot and start it.
    let bot = Bot {
        opts,
        client,
        token,
        config,
        broadcasters,
    };
    bot.start().await?;
    Ok(())
}

pub struct Bot {
    pub opts: Cli,
    pub client: HelixClient<'static, reqwest::Client>,
    pub token: Arc<Mutex<twitch_oauth2::UserToken>>,
    pub config: Config,
    pub broadcasters: Vec<twitch_api::types::UserId>,
}

impl Bot {
    /// Start the bot. This will connect to the chat and start handling for events with [Bot::handle_event].
    /// This will also start a task that will refresh the token if it's about to expire and check if it's still valid.
    pub async fn start(&self) -> Result<(), eyre::Report> {
        // To make a connection to the chat we need to use a websocket connection.
        // This is a wrapper for the websocket connection that handles the reconnects and handles all messages from eventsub.
        let websocket = websocket::ChatWebsocketClient {
            session_id: None,
            token: self.token.clone(),
            client: self.client.clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            chats: self.broadcasters.clone(),
        };
        let refresh_token = async move {
            let token = self.token.clone();
            let client = self.client.clone();
            // We check constantly if the token is valid.
            // We also need to refresh the token if it's about to be expired.
            let mut interval = tokio::time::interval(TOKEN_VALIDATION_INTERVAL);
            loop {
                interval.tick().await;
                let mut token = token.lock().await;
                refresh_and_validate_token(&mut token, &client, &self.opts).await?;
            }
            #[allow(unreachable_code)]
            Ok(())
        };
        let ws = websocket.run(|e, ts| async { self.handle_event(e, ts).await });
        futures::future::try_join(ws, refresh_token).await?;
        Ok(())
    }

    /// Handle chat messages, if they start with `!` send it to [Bot::command].
    async fn handle_chat_message(
        &self,
        token: tokio::sync::MutexGuard<'_, UserToken>,
        payload: eventsub::channel::ChannelChatMessageV1Payload,
        subscription: eventsub::EventSubscriptionInformation<
            eventsub::channel::ChannelChatMessageV1,
        >,
    ) -> Result<(), eyre::Error> {
        if let Some(command) = payload.message.text.strip_prefix("!") {
            let mut split_whitespace = command.split_whitespace();
            let command = split_whitespace.next().unwrap();
            let rest = split_whitespace.next();

            self.command(&payload, &subscription, command, rest, &token)
                .await?;
        }
        Ok(())
    }

    /// Handle all eventsub events.
    /// We print the message to the console and if it's a chat message we send it to [Bot::handle_chat_message].
    /// If there's an event you want to listen to you should first add it to [websocket::ChatWebsocketClient::process_welcome_message] and then handle it here.
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), eyre::Report> {
        let token = self.token.lock().await;
        let time_format = time::format_description::parse("[hour]:[minute]:[second]")?;
        match event {
            Event::ChannelChatMessageV1(Payload {
                message: Message::Notification(payload),
                subscription,
                ..
            }) => {
                println!(
                    "[{}] #{} {}: {}",
                    timestamp.to_utc().format(&time_format).unwrap(),
                    payload.broadcaster_user_login,
                    payload.chatter_user_name,
                    payload.message.text
                );

                self.handle_chat_message(token, payload, subscription)
                    .await?;
            }
            Event::ChannelChatNotificationV1(Payload {
                message: Message::Notification(payload),
                ..
            }) => {
                println!(
                    "[{}] [Event] {}: {}",
                    timestamp.to_utc().format(&time_format).unwrap(),
                    match &payload.chatter {
                        eventsub::channel::chat::notification::Chatter::Chatter {
                            chatter_user_name: user,
                            ..
                        } => user.as_str(),
                        _ => "anonymous",
                    },
                    payload.message.text
                );
            }
            _ => {}
        }
        Ok(())
    }

    async fn command(
        &self,
        payload: &eventsub::channel::ChannelChatMessageV1Payload,
        subscription: &eventsub::EventSubscriptionInformation<
            eventsub::channel::ChannelChatMessageV1,
        >,
        command: &str,
        _rest: Option<&str>,
        token: &UserToken,
    ) -> Result<(), eyre::Report> {
        tracing::info!("Command: {}", command);
        if let Some(response) = self.config.command.iter().find(|c| c.trigger == command) {
            self.client
                .send_chat_message_reply(
                    &subscription.condition.broadcaster_user_id,
                    &subscription.condition.user_id,
                    &payload.message_id,
                    response
                        .response
                        .replace("{user}", payload.chatter_user_name.as_str())
                        .as_str(),
                    token,
                )
                .await?;
        }
        Ok(())
    }
}

async fn refresh_and_validate_token(
    token: &mut UserToken,
    client: &HelixClient<'_, reqwest::Client>,
    opts: &Cli,
) -> Result<(), eyre::Report> {
    if token.expires_in() < TOKEN_EXPIRATION_THRESHOLD {
        tracing::info!("refreshed token");
        token
            .refresh_token(client)
            .await
            .wrap_err("couldn't refresh token")?;
        save_token(token, &opts.auth)?;
    }
    token
        .validate_token(client)
        .await
        .wrap_err("couldn't validate token")?;
    Ok(())
}

/// Used to save the token to a file
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct SavedToken {
    access_token: twitch_oauth2::AccessToken,
    refresh_token: twitch_oauth2::RefreshToken,
}

// you should probably replace this with something more robust
#[cfg(debug_assertions)]
fn save_token(
    token: &twitch_oauth2::UserToken,
    save_path: &std::path::Path,
) -> Result<(), eyre::Report> {
    let token = SavedToken {
        access_token: token.access_token.clone(),
        refresh_token: token.refresh_token.clone().unwrap(),
    };
    let text = toml::to_string(&token)?;
    std::fs::write(save_path, text)?;
    Ok(())
}

#[cfg(debug_assertions)]
async fn load_token(
    path: &std::path::Path,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<Option<twitch_oauth2::UserToken>, eyre::Report> {
    let Some(text) = std::fs::read_to_string(path).ok() else {
        return Ok(None);
    };
    let token: SavedToken = toml::from_str(&text)?;
    Ok(Some(
        twitch_oauth2::UserToken::from_existing(
            client,
            token.access_token,
            token.refresh_token,
            None,
        )
        .await?,
    ))
}

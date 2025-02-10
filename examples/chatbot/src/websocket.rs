use eyre::WrapErr;
use futures::TryStreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;
use tracing::Instrument;

use twitch_api::{
    eventsub::{
        self,
        event::websocket::{EventsubWebsocketData, ReconnectPayload, SessionData, WelcomePayload},
        Event,
    },
    types::{self},
    HelixClient,
};
use twitch_oauth2::{TwitchToken, UserToken};

pub struct ChatWebsocketClient {
    /// The session id of the websocket connection
    pub session_id: Option<String>,
    /// The token used to authenticate with the Twitch API
    pub token: Arc<Mutex<UserToken>>,
    /// The client used to make requests to the Twitch API
    pub client: HelixClient<'static, reqwest::Client>,
    /// The url to use for websocket
    pub connect_url: url::Url,
    /// Chats to connect to.
    pub chats: Vec<twitch_api::types::UserId>,
}

impl ChatWebsocketClient {
    /// Connect to the websocket and return the stream
    async fn connect(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        eyre::Error,
    > {
        tracing::info!("connecting to twitch");
        let config = tungstenite::protocol::WebSocketConfig::default();
        let (socket, _) =
            tokio_tungstenite::connect_async_with_config(&self.connect_url, Some(config), false)
                .await
                .wrap_err("Can't connect")?;

        Ok(socket)
    }

    /// Run the websocket subscriber
    #[tracing::instrument(name = "subscriber", skip_all, fields())]
    pub async fn run<Fut>(
        mut self,
        mut event_fn: impl FnMut(Event, types::Timestamp) -> Fut,
    ) -> Result<(), eyre::Report>
    where
        Fut: std::future::Future<Output = Result<(), eyre::Report>>,
    {
        // Establish the stream
        let mut s = self
            .connect()
            .await
            .context("when establishing connection")?;
        // Loop over the stream, processing messages as they come in.
        while let Some(msg) = futures::StreamExt::next(&mut s).await {
            let span = tracing::debug_span!("message received", raw_message = ?msg);
            let msg = match msg {
                Err(tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    tracing::warn!(
                        "connection was sent an unexpected frame or was reset, reestablishing it"
                    );
                    s = self
                        .connect()
                        .instrument(span)
                        .await
                        .context("when reestablishing connection")?;
                    continue;
                }
                _ => msg.context("when getting message")?,
            };
            self.process_message(msg, &mut event_fn)
                .instrument(span)
                .await?
        }
        Ok(())
    }

    /// Process a message from the websocket
    async fn process_message<Fut>(
        &mut self,
        msg: tungstenite::Message,
        event_fn: &mut impl FnMut(Event, types::Timestamp) -> Fut,
    ) -> Result<(), eyre::Report>
    where
        Fut: std::future::Future<Output = Result<(), eyre::Report>>,
    {
        match msg {
            tungstenite::Message::Text(s) => {
                tracing::trace!("{s}");
                // Parse the message into a [twitch_api::eventsub::EventsubWebsocketData]
                match Event::parse_websocket(&s)? {
                    EventsubWebsocketData::Welcome {
                        payload: WelcomePayload { session },
                        ..
                    }
                    | EventsubWebsocketData::Reconnect {
                        payload: ReconnectPayload { session },
                        ..
                    } => {
                        self.process_welcome_message(session).await?;
                        Ok(())
                    }
                    EventsubWebsocketData::Notification { metadata, payload } => {
                        event_fn(payload, metadata.message_timestamp.into_owned()).await?;
                        Ok(())
                    }
                    re @ EventsubWebsocketData::Revocation { .. } => {
                        eyre::bail!("got revocation event: {re:?}")
                    }
                    EventsubWebsocketData::Keepalive {
                        metadata: _,
                        payload: _,
                    } => Ok(()),
                    _ => Ok(()),
                }
            }
            tungstenite::Message::Close(_) => todo!(),
            _ => Ok(()),
        }
    }

    async fn process_welcome_message(&mut self, data: SessionData<'_>) -> Result<(), eyre::Report> {
        tracing::info!("connected to twitch chat");
        self.session_id = Some(data.id.to_string());
        if let Some(url) = data.reconnect_url {
            self.connect_url = url.parse()?;
        }
        let token = self.token.lock().await;
        let transport = eventsub::Transport::websocket(data.id.clone());
        for id in &self.chats {
            let user_id = token.user_id().unwrap().to_owned();
            let subs: Vec<_> = self
                .client
                .get_eventsub_subscriptions(Some(eventsub::Status::Enabled), None, None, &*token)
                .map_ok(|r| {
                    futures::stream::iter(
                        r.subscriptions
                            .into_iter()
                            .filter(|s| {
                                s.transport
                                    .as_websocket()
                                    .is_some_and(|t| t.session_id == data.id)
                            })
                            .map(Ok::<_, eyre::Report>),
                    )
                })
                .try_flatten()
                .try_collect()
                .await?;
            if !subs.is_empty() {
                continue;
            }
            let message =
                eventsub::channel::chat::ChannelChatMessageV1::new(id.clone(), user_id.clone());
            self.client
                .create_eventsub_subscription(message, transport.clone(), &*token)
                .await?;
            self.client
                .create_eventsub_subscription(
                    eventsub::channel::chat::ChannelChatNotificationV1::new(id.clone(), user_id),
                    transport.clone(),
                    &*token,
                )
                .await?;
        }
        Ok(())
    }
}

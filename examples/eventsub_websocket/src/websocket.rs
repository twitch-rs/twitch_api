use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eyre::Context;
use futures::{stream::SplitStream, StreamExt};
use reqwest::Client;
use tokio::{
    sync::{
        mpsc::{self, UnboundedSender},
        Mutex,
    },
    task::{JoinError, JoinHandle},
    time::{Duration, Instant},
};
use tokio_tungstenite::{
    tungstenite::{client::IntoClientRequest, protocol::WebSocketConfig, Message as WsMessage},
    MaybeTlsStream, WebSocketStream,
};
use twitch_api::{
    eventsub::{
        self,
        channel::{ChannelBanV1, ChannelUnbanV1},
        event::websocket::{EventsubWebsocketData, ReconnectPayload, WelcomePayload},
        Event, EventSubscription, Message, SessionData, Transport,
    },
    helix::eventsub::CreateEventSubSubscription,
    twitch_oauth2::{TwitchToken, UserToken},
    types, HelixClient,
};

/// Connect to the websocket and return the stream
async fn connect(
    request: impl IntoClientRequest + Unpin,
) -> Result<SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>, eyre::Error> {
    tracing::info!("connecting to twitch");
    let config = Some(WebSocketConfig {
        max_message_size: Some(64 << 20), // 64 MiB
        max_frame_size: Some(16 << 20),   // 16 MiB
        accept_unmasked_frames: false,
        ..WebSocketConfig::default()
    });
    let socket = tokio_tungstenite::connect_async_with_config(request, config, false)
        .await
        .context("Can't connect")?
        .0
        .split()
        .1;

    Ok(socket)
}

async fn refresh_if_expired(
    token: Arc<Mutex<UserToken>>,
    helix_client: &HelixClient<'_, Client>,
    _opts: &crate::Opts,
) {
    let lock = token.lock().await;

    if lock.expires_in() >= Duration::from_secs(60) {
        return;
    }
    let _client = helix_client.get_client();

    // TODO: token refresh logic is left up to the user

    drop(lock);
}

async fn subscribe(
    helix_client: &HelixClient<'_, Client>,
    session_id: String,
    token: &UserToken,
    subscription: impl EventSubscription + Send,
) -> eyre::Result<()> {
    let transport: Transport = Transport::websocket(session_id);
    let _event_info: CreateEventSubSubscription<_> = helix_client
        .create_eventsub_subscription(subscription, transport, token)
        .await?;
    Ok(())
}

/// action to perform on received message
enum Action {
    /// do nothing with the message
    Nothing,
    /// reset the timeout and keep the connection alive
    ResetKeepalive,
    /// kill predecessor and swap the handle
    KillPredecessor,
    /// spawn successor and await death signal
    AssignSuccessor(ActorHandle),
}

async fn process_welcome(
    subscribed: &AtomicBool,
    token: &Mutex<UserToken>,
    helix_client: &HelixClient<'_, Client>,
    user_id: &types::UserId,
    session: SessionData<'_>,
) -> eyre::Result<()> {
    // if we're already subscribed, don't subscribe again
    if subscribed.load(Ordering::Relaxed) {
        return Ok(());
    }
    let user_token = token.lock().await;
    tokio::try_join!(
        subscribe(
            helix_client,
            session.id.to_string(),
            &user_token,
            ChannelBanV1::broadcaster_user_id(user_id.clone()),
        ),
        subscribe(
            helix_client,
            session.id.to_string(),
            &user_token,
            ChannelUnbanV1::broadcaster_user_id(user_id.clone()),
        ),
    )?;
    subscribed.store(true, Ordering::Relaxed);
    Ok(())
}

/// Here is where you would handle the events you want to listen to
fn process_payload(event: Event) -> eyre::Result<Action> {
    match event {
        Event::ChannelBanV1(eventsub::Payload { message, .. }) => {
            match message {
                // not needed for websocket
                Message::VerificationRequest(_) => unreachable!(),
                Message::Revocation() => Err(eyre::eyre!("unexpected subscription revocation")),
                Message::Notification(payload) => {
                    // do something useful with the payload
                    tracing::info!(?payload, "got ban event");

                    // new events reset keepalive timeout too
                    Ok(Action::ResetKeepalive)
                }
                _ => Ok(Action::Nothing),
            }
        }
        Event::ChannelUnbanV1(eventsub::Payload { message, .. }) => {
            match message {
                // not needed for websocket
                Message::VerificationRequest(_) => unreachable!(),
                Message::Revocation() => Err(eyre::eyre!("unexpected subscription revocation")),
                Message::Notification(payload) => {
                    // do something useful with the payload
                    tracing::info!(?payload, "got unban event");

                    // new events reset keepalive timeout too
                    Ok(Action::ResetKeepalive)
                }
                _ => Ok(Action::Nothing),
            }
        }
        _ => Ok(Action::Nothing),
    }
}

struct WebSocketConnection {
    socket: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    helix_client: &'static HelixClient<'static, Client>,
    token: Arc<Mutex<UserToken>>,
    opts: Arc<crate::Opts>,
    subscribed: Arc<AtomicBool>,
    user_id: Arc<types::UserId>,
    kill_self_tx: UnboundedSender<()>,
}

impl WebSocketConnection {
    async fn receive_message(&mut self) -> eyre::Result<Option<String>> {
        let Some(message) = self.socket.next().await else {
            return Err(eyre::eyre!("websocket stream closed unexpectedly"));
        };
        match message.context("tungstenite error")? {
            WsMessage::Close(frame) => {
                let reason = frame.map(|frame| frame.reason).unwrap_or_default();
                Err(eyre::eyre!(
                    "websocket stream closed unexpectedly with reason {reason}"
                ))
            }
            WsMessage::Frame(_) => unreachable!(),
            WsMessage::Ping(_) | WsMessage::Pong(_) => {
                // no need to do anything as tungstenite automatically handles pings for you
                // but refresh the token just in case
                refresh_if_expired(self.token.clone(), self.helix_client, &self.opts).await;
                Ok(None)
            }
            WsMessage::Binary(_) => unimplemented!(),
            WsMessage::Text(payload) => Ok(Some(payload)),
        }
    }

    async fn process_message(&self, frame: String) -> eyre::Result<Action> {
        let event_data = Event::parse_websocket(&frame).context("parsing error")?;
        match event_data {
            EventsubWebsocketData::Welcome {
                payload: WelcomePayload { session },
                ..
            } => {
                process_welcome(
                    &self.subscribed,
                    &self.token,
                    self.helix_client,
                    &self.user_id,
                    session,
                )
                .await?;
                Ok(Action::KillPredecessor)
            }
            EventsubWebsocketData::Reconnect {
                payload: ReconnectPayload { session },
                ..
            } => {
                let url: String = session.reconnect_url.unwrap().into_owned();
                let successor = ActorHandle::spawn(
                    url,
                    self.helix_client,
                    self.kill_self_tx.clone(),
                    self.token.clone(),
                    self.opts.clone(),
                    self.subscribed.clone(),
                    self.user_id.clone(),
                );
                Ok(Action::AssignSuccessor(successor))
            }
            EventsubWebsocketData::Keepalive { .. } => Ok(Action::ResetKeepalive),
            EventsubWebsocketData::Revocation { metadata, .. } => {
                eyre::bail!("got revocation: {metadata:?}")
            }
            EventsubWebsocketData::Notification { payload: event, .. } => process_payload(event),
            _ => Ok(Action::Nothing),
        }
    }
}

struct ActorHandle(JoinHandle<eyre::Result<ActorHandle>>);

impl ActorHandle {
    pub fn spawn(
        url: impl IntoClientRequest + Unpin + Send + 'static,
        helix_client: &'static HelixClient<'_, Client>,
        kill_predecessor_tx: UnboundedSender<()>,
        token: Arc<Mutex<UserToken>>,
        opts: Arc<crate::Opts>,
        subscribed: Arc<AtomicBool>,
        user_id: Arc<types::UserId>,
    ) -> Self {
        Self(tokio::spawn(async move {
            let socket = connect(url).await?;
            // If we receive a reconnect message we want to spawn a new connection to twitch.
            // The already existing session should wait on the new session to receive a welcome message before being closed.
            // https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#reconnect-message
            let (kill_self_tx, mut kill_self_rx) = mpsc::unbounded_channel::<()>();

            let mut connection = WebSocketConnection {
                socket,
                helix_client,
                token,
                opts,
                subscribed,
                user_id,
                kill_self_tx,
            };

            /// default keepalive duration is 10 seconds
            const WINDOW: u64 = 10;
            let mut timeout: Instant = Instant::now() + Duration::from_secs(WINDOW);
            let mut successor: Option<Self> = None;

            loop {
                tokio::select! {
                    biased;
                    result = kill_self_rx.recv() => {
                        result.unwrap();
                        let Some(successor) = successor else {
                            // can't receive death signal from successor if it isn't spawned yet
                            unreachable!();
                        };
                        return Ok(successor);
                    }
                    result = connection.receive_message() => if let Some(frame) = result? {
                        let side_effect = connection.process_message(frame).await?;
                        match side_effect {
                            Action::Nothing => {}
                            Action::ResetKeepalive => timeout = Instant::now() + Duration::from_secs(WINDOW),
                            Action::KillPredecessor => kill_predecessor_tx.send(())?,
                            Action::AssignSuccessor(actor_handle) => {
                                successor = Some(actor_handle);
                            },
                        }
                    },
                    _ = tokio::time::sleep_until(timeout) => eyre::bail!("connection timed out"),
                }
            }
        }))
    }

    pub async fn join(self) -> Result<eyre::Result<Self>, JoinError> { self.0.await }
}

pub async fn run(
    helix_client: &'static HelixClient<'_, Client>,
    token: UserToken,
    opts: Arc<crate::opts::Opts>,
    user_id: types::UserId,
) -> eyre::Result<()> {
    let url = twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone();
    let token = Arc::new(Mutex::new(token));
    let user_id = Arc::new(user_id);
    let subscribed = Arc::new(AtomicBool::new(false));

    // since this is a root actor without a predecessor it has no previous connection to kill
    // but we still need to give it a sender to satisfy the function signature.
    // `_` and `_unused` have different semantics where `_` is dropped immediately and sender gets a recv error
    let (dummy_tx, _unused_rx) = mpsc::unbounded_channel::<()>();
    let mut handle = ActorHandle::spawn(
        url.clone(),
        helix_client,
        dummy_tx.clone(),
        token.clone(),
        opts.clone(),
        subscribed.clone(),
        user_id.clone(),
    );

    loop {
        handle = match handle.join().await? {
            Ok(handle) => handle,
            Err(err) => {
                subscribed.store(false, Ordering::Relaxed);
                tracing::error!("{err}");
                ActorHandle::spawn(
                    url.clone(),
                    helix_client,
                    dummy_tx.clone(),
                    token.clone(),
                    opts.clone(),
                    subscribed.clone(),
                    user_id.clone(),
                )
            }
        }
    }
}

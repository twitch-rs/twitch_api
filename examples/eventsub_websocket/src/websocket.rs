use std::{collections::HashSet, sync::Arc, time::Duration};

use eyre::Context;
use futures::{
    stream::{self, SplitStream},
    StreamExt, TryStreamExt,
};
use reqwest::Client;
use tokio::{
    sync::{
        mpsc::{self, UnboundedSender},
        Mutex, MutexGuard,
    },
    task::{JoinError, JoinHandle},
};
use tokio_tungstenite::{
    tungstenite::{
        client::IntoClientRequest, protocol::WebSocketConfig, Message as WsMessage,
    },
    MaybeTlsStream, WebSocketStream,
};
use twitch_api::{
    eventsub::{
        self,
        channel::{ChannelBanV1, ChannelUnbanV1},
        event::websocket::{EventsubWebsocketData, ReconnectPayload, WelcomePayload},
        Event, EventSubSubscription, EventSubscription, EventType, Message, Transport,
    },
    helix::{eventsub::CreateEventSubSubscription, ClientRequestError, HelixRequestPostError},
    twitch_oauth2::{TwitchToken, UserToken},
    types, HelixClient,
};

/// Connect to the websocket and return the stream
///
/// eventsub websocket doesn't support outgoing messages except pongs (which are implicitly handled by tungstenite)
/// so we return only the receiving end of the socket
///
/// [Getting Events Using WebSockets](https://dev.twitch.tv/docs/eventsub/handling-websocket-events/)
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

async fn make_token(
    client: &impl twitch_oauth2::client::Client,
    token: impl Into<twitch_oauth2::AccessToken>,
) -> Result<UserToken, eyre::Report> {
    UserToken::from_existing(client, token.into(), None, None)
        .await
        .context("could not use access token")
}

async fn refresh_if_expired(
    token: Arc<Mutex<UserToken>>,
    helix_client: &HelixClient<'_, Client>,
    opts: &crate::Opts,
) -> eyre::Result<()> {
    let mut lock: MutexGuard<'_, UserToken> = token.lock().await;

    if lock.expires_in() >= Duration::from_secs(60) {
        return Ok(());
    }
    let client = helix_client.get_client();

    let new_token = if let Some(ref access_token) = opts.access_token {
        make_token(client, access_token.secret().to_string()).await?
    } else if let (Some(ref oauth_service_url), Some(ref pointer)) =
        (&opts.oauth2_service_url, &opts.oauth2_service_pointer)
    {
        tracing::info!(
            "using oauth service on `{}` to get oauth token",
            oauth_service_url
        );

        let mut request = client.get(oauth_service_url.clone());
        if let Some(ref key) = opts.oauth2_service_key {
            request = request.bearer_auth(key.secret());
        }
        let request = request.build()?;
        tracing::debug!("request: {:?}", request);

        match client.execute(request).await {
            Ok(response)
                if !(response.status().is_client_error()
                    || response.status().is_server_error()) =>
            {
                let service_response: serde_json::Value = response
                    .json()
                    .await
                    .context("could not transform oauth service response to json")?;
                make_token(
                    client,
                    service_response
                        .pointer(pointer)
                        .ok_or_else(|| eyre::eyre!("could not get a field on `{}`", pointer))?
                        .as_str()
                        .ok_or_else(|| eyre::eyre!("token is not a string"))?
                        .to_string(),
                )
                .await?
            }
            Ok(response_error) => {
                let status = response_error.status();
                let error = response_error.text().await?;
                eyre::bail!(
                    "oauth service returned error code: {} with body: {:?}",
                    status,
                    error
                );
            }
            Err(e) => {
                return Err(e)
                    .with_context(|| format!("calling oauth service on `{}`", &oauth_service_url))
            }
        }
    } else {
        panic!("got empty vals for token cli group: {:?}", opts)
    };

    *lock = new_token;
    drop(lock);
    Ok(())
}

async fn get_existing_subscriptions(
    helix_client: &HelixClient<'_, Client>,
    token: &UserToken,
    session_id: &str,
    event_type: EventType,
) -> eyre::Result<HashSet<types::UserId>> {
    Ok(helix_client
        .get_eventsub_subscriptions(
            Some(eventsub::Status::Enabled),
            Some(event_type),
            None,
            token,
        )
        .map_ok(|r: twitch_api::helix::eventsub::EventSubSubscriptions| {
            stream::iter(
                r.subscriptions
                    .into_iter()
                    .filter(|s: &EventSubSubscription| {
                        s.transport.as_websocket().is_some_and(
                            |t: &eventsub::WebsocketTransportResponse| t.session_id == session_id,
                        )
                    })
                    .filter_map(|sub: EventSubSubscription| {
                        Some(types::UserId::new(
                            sub.condition
                                .get("broadcaster_user_id")?
                                .as_str()?
                                .to_owned(),
                        ))
                    })
                    .map(Ok::<_, ClientRequestError<reqwest::Error>>),
            )
        })
        .try_flatten()
        .try_collect::<HashSet<types::UserId>>()
        .await?)
}

async fn subscribe(
    helix_client: &HelixClient<'_, Client>,
    session_id: String,
    token: &UserToken,
    subscription: impl EventSubscription + Send,
) -> eyre::Result<()> {
    let transport: Transport = Transport::websocket(session_id);
    let event_info: Result<CreateEventSubSubscription<_>, ClientRequestError<reqwest::Error>> =
        helix_client
            .create_eventsub_subscription(subscription, transport, token)
            .await;
    match event_info {
        Err(ClientRequestError::HelixRequestPostError(HelixRequestPostError::Error {
            status,
            ..
        })) if status.as_u16() == 409 => {
            tracing::warn!("409 subscription already exists");
        }
        _ => {
            event_info?;
        }
    }
    Ok(())
}

async fn process_message(
    socket: &mut SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    helix_client: &'static HelixClient<'_, Client>,
    token: Arc<Mutex<UserToken>>,
    opts: Arc<crate::Opts>,
    channel_ids: Arc<HashSet<types::UserId>>,
    predecessor_killer: UnboundedSender<()>,
    self_killer: UnboundedSender<()>,
) -> eyre::Result<Option<ActorHandle>> {
    let Some(message) = socket.next().await else {
        return Err(eyre::eyre!("websocket stream closed unexpectedly"));
    };
    let frame = match message {
        Ok(WsMessage::Close(frame)) => {
            let reason = frame.map(|frame| frame.reason).unwrap_or_default();
            return Err(eyre::eyre!(
                "websocket stream closed unexpectedly with reason {reason}"
            ));
        }
        Ok(WsMessage::Frame(_)) => unreachable!(),
        Ok(WsMessage::Ping(_) | WsMessage::Pong(_)) => {
            // no need to do anything as tungstenite automatically handles pings for you
            // but refresh the token just in case
            refresh_if_expired(token, helix_client, &opts).await?;
            return Ok(None);
        }
        Ok(WsMessage::Binary(_)) => unimplemented!(),
        Ok(WsMessage::Text(payload)) => payload,
        Err(err) => return Err(err).context("tungstenite error"),
    };

    let event_data = Event::parse_websocket(&frame).context("parsing error")?;

    match event_data {
        EventsubWebsocketData::Welcome {
            payload: WelcomePayload { session },
            ..
        } => {
            let user_token: MutexGuard<'_, UserToken> = token.lock().await;
            // preventing duplicating subscriptions and hitting 409
            let ban_subs = get_existing_subscriptions(
                helix_client,
                &user_token,
                &session.id,
                EventType::ChannelBan,
            )
            .await?;
            let unban_subs = get_existing_subscriptions(
                helix_client,
                &user_token,
                &session.id,
                EventType::ChannelUnban,
            )
            .await?;
            let bans = channel_ids
                .difference(&ban_subs)
                .cloned()
                .map(|user_id: types::UserId| {
                    // cloning the session id
                    subscribe(
                        helix_client,
                        session.id.to_string(),
                        &user_token,
                        ChannelBanV1::broadcaster_user_id(user_id),
                    )
                });
            let unbans =
                channel_ids
                    .difference(&unban_subs)
                    .cloned()
                    .map(|user_id: types::UserId| {
                        subscribe(
                            helix_client,
                            session.id.to_string(),
                            &user_token,
                            ChannelUnbanV1::broadcaster_user_id(user_id),
                        )
                    });
            stream::iter(bans)
                .buffer_unordered(4)
                // this is horrible but at least it's concurrent
                .collect::<Vec<eyre::Result<()>>>()
                .await
                .into_iter()
                .try_for_each(|r| r)?;
            stream::iter(unbans)
                .buffer_unordered(4)
                // this is horrible but at least it's concurrent
                .collect::<Vec<eyre::Result<()>>>()
                .await
                .into_iter()
                .try_for_each(|r| r)?;

            drop(user_token);
            predecessor_killer.send(())?;

            Ok(None)
        }
        EventsubWebsocketData::Reconnect {
            payload: ReconnectPayload { session },
            ..
        } => {
            let url: String = session.reconnect_url.unwrap().into_owned();
            let successor =
                ActorHandle::spawn(url, helix_client, self_killer, token, opts, channel_ids);
            Ok(Some(successor))
        }
        // TODO: keepalive counting https://dev.twitch.tv/docs/eventsub/handling-websocket-events/#keepalive-message
        EventsubWebsocketData::Keepalive { .. } => Ok(None),
        EventsubWebsocketData::Revocation { metadata, .. } => {
            eyre::bail!("got revocation event: {metadata:?}")
        }
        EventsubWebsocketData::Notification { payload, .. } => {
            match payload {
                Event::ChannelBanV1(eventsub::Payload { message, .. }) => {
                    match message {
                        // not needed for websocket
                        Message::VerificationRequest(_) => unreachable!(),
                        Message::Revocation() => {
                            Err(eyre::eyre!("unexpected subscription revocation"))
                        }
                        Message::Notification(payload) => {
                            /*
                            do something useful with the payload
                            */
                            tracing::info!("doing something useful with channel.ban {payload:?}");

                            Ok(None)
                        }
                        message => {
                            tracing::debug!("unexpected message {message:?}");
                            Ok(None)
                        }
                    }
                }
                Event::ChannelUnbanV1(eventsub::Payload { message, .. }) => {
                    match message {
                        // not needed for websocket
                        Message::VerificationRequest(_) => unreachable!(),
                        Message::Revocation() => {
                            Err(eyre::eyre!("unexpected subscription revocation"))
                        }
                        Message::Notification(payload) => {
                            /*
                            do something useful with the payload
                            */
                            tracing::info!("doing something useful with channel.unban {payload:?}");

                            Ok(None)
                        }
                        message => {
                            tracing::debug!("unexpected message {message:?}");
                            Ok(None)
                        }
                    }
                }
                event => {
                    tracing::debug!("unexpected event {event:?}");
                    Ok(None)
                }
            }
        }
        data => {
            tracing::debug!("unexpected data {data:?}");
            Ok(None)
        }
    }
}

struct ActorHandle(JoinHandle<eyre::Result<ActorHandle>>);

impl ActorHandle {
    pub fn spawn(
        url: impl IntoClientRequest + Unpin + Send + 'static,
        helix_client: &'static HelixClient<'_, Client>,
        predecessor_killer: UnboundedSender<()>,
        token: Arc<Mutex<UserToken>>,
        opts: Arc<crate::Opts>,
        channel_ids: Arc<HashSet<types::UserId>>,
    ) -> Self {
        Self(tokio::spawn(async move {
            let mut socket = connect(url).await?;
            let (self_killer, mut terminator) = mpsc::unbounded_channel::<()>();
            let mut successor: Option<Self> = None;

            loop {
                tokio::select! {
                    biased;
                    result = terminator.recv() => {
                        result.unwrap();
                        return Ok(successor.expect("can't receive death signal from successor if it isn't spawned yet"));
                    }
                    result = process_message(
                        &mut socket,
                        helix_client,
                        token.clone(),
                        opts.clone(),
                        channel_ids.clone(),
                        predecessor_killer.clone(),
                        self_killer.clone(),
                    ) => if let Some(handle) = result? { successor = Some(handle) }
                }
            }
        }))
    }

    pub async fn join(self) -> Result<eyre::Result<Self>, JoinError> { self.0.await }
}

pub async fn run(
    helix_client: &'static HelixClient<'_, Client>,
    token: Arc<Mutex<UserToken>>,
    opts: Arc<crate::opts::Opts>,
    channel_ids: Arc<HashSet<types::UserId>>,
) -> eyre::Result<()> {
    let url = twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone();

    // `_` and `_unused` turn out to have different semantics where `_` is dropped immediately,
    // so sender gets a recv error
    let (dummy_killer, _unused) = mpsc::unbounded_channel::<()>();
    let mut handle = ActorHandle::spawn(
        url.clone(),
        helix_client,
        dummy_killer.clone(),
        token.clone(),
        opts.clone(),
        channel_ids.clone(),
    );

    loop {
        handle = match handle.join().await? {
            Ok(handle) => handle,
            Err(err) => {
                tracing::error!("{err}");
                ActorHandle::spawn(
                    url.clone(),
                    helix_client,
                    dummy_killer.clone(),
                    token.clone(),
                    opts.clone(),
                    channel_ids.clone(),
                )
            }
        }
    }
}

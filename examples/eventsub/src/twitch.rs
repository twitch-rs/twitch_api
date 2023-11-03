use std::sync::Arc;

use axum::{
    body::HttpBody,
    extract::{ws, Extension},
    http,
    response::IntoResponse,
};
use eyre::Context;
use futures::TryStreamExt;
use hyper::StatusCode;
use tokio::sync::{watch, RwLock};
use twitch_api::{
    eventsub::{
        self as twitch_eventsub,
        stream::{StreamOfflineV1, StreamOfflineV1Payload, StreamOnlineV1, StreamOnlineV1Payload},
        Event, EventType, Status,
    },
    helix,
    twitch_oauth2::{AppAccessToken, ClientId, ClientSecret, TwitchToken},
    types::{self, UserNameRef},
    HelixClient,
};

use crate::{opts::Opts, Config};

pub async fn eventsub_register(
    token: Arc<RwLock<AppAccessToken>>,
    config: Arc<Config>,
    client: HelixClient<'static, reqwest::Client>,
    website: String,
    sign_secret: crate::SignSecret,
) -> eyre::Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // check every day
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(24 * 60 * 60));

    loop {
        // first check if we are already registered
        interval.tick().await;
        tracing::info!("checking subs");
        let subs = client
            .get_eventsub_subscriptions(Status::Enabled, None, None, &*token.read().await)
            .map_ok(|events| {
                futures::stream::iter(events.subscriptions.into_iter().map(Ok::<_, eyre::Report>))
            })
            .try_flatten()
            // filter out websockets
            .try_filter(|event| futures::future::ready(event.transport.is_webhook()))
            .try_collect::<Vec<_>>()
            .await?;
        let online_exists = subs.iter().any(|sub| {
            // we've filtered out websocket transports
            sub.transport.as_webhook().unwrap().callback == website
                && sub.type_ == EventType::StreamOnline
                && sub.version == "1"
                && sub
                    .condition
                    .as_object()
                    .expect("a stream.online did not contain broadcaster")
                    .get("broadcaster_user_id")
                    .unwrap()
                    .as_str()
                    == Some(config.broadcaster.id.as_str())
        });
        let offline_exists = subs.iter().any(|sub| {
            // we've filtered out websocket transports
            sub.transport.as_webhook().unwrap().callback == website
                && sub.type_ == EventType::StreamOffline
                && sub.version == "1"
                && sub
                    .condition
                    .as_object()
                    .expect("a stream.offline did not contain broadcaster")
                    .get("broadcaster_user_id")
                    .unwrap()
                    .as_str()
                    == Some(config.broadcaster.id.as_str())
        });

        tracing::info!(
            offline = offline_exists,
            online = online_exists,
            "got existing subs"
        );

        let transport = twitch_eventsub::Transport::webhook(
            website.clone(),
            sign_secret.secret_str().to_string(),
        );
        drop(subs);
        if !online_exists {
            if std::env::var("DEV").is_ok() {
                tracing::info!("In dev mode, not registering eventsubs");
            } else {
                client
                    .create_eventsub_subscription(
                        StreamOnlineV1::broadcaster_user_id(config.broadcaster.id.clone()),
                        transport.clone(),
                        &*token.read().await,
                    )
                    .await
                    .wrap_err_with(|| "when registering online event")?;
            }
        }

        if !offline_exists {
            if std::env::var("DEV").is_ok() {
                tracing::info!("In dev mode, not registering eventsubs");
                return Ok(());
            } else {
                client
                    .create_eventsub_subscription(
                        StreamOfflineV1::broadcaster_user_id(config.broadcaster.id.clone()),
                        twitch_eventsub::Transport::webhook(
                            website.clone(),
                            sign_secret.secret_str().to_string(),
                        ),
                        &*token.read().await,
                    )
                    .await
                    .wrap_err_with(|| "when registering offline event")?;
            }
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}

pub async fn twitch_eventsub(
    Extension(sender): Extension<Arc<watch::Sender<LiveStatus>>>,
    Extension(opts): Extension<Arc<Opts>>,
    Extension(config): Extension<Arc<Config>>,
    Extension(cache): Extension<Arc<retainer::Cache<http::HeaderValue, ()>>>,
    request: http::Request<axum::body::Body>,
) -> impl IntoResponse {
    const MAX_ALLOWED_RESPONSE_SIZE: u64 = 64 * 1024;

    let (parts, body) = request.into_parts();
    let response_content_length = match body.size_hint().upper() {
        Some(v) => v,
        None => MAX_ALLOWED_RESPONSE_SIZE + 1, /* Just to protect ourselves from a malicious response */
    };
    let body = if response_content_length < MAX_ALLOWED_RESPONSE_SIZE {
        hyper::body::to_bytes(body).await.unwrap()
    } else {
        panic!("too big data given")
    };

    let request = http::Request::from_parts(parts, &*body);

    tracing::debug!("got event {}", std::str::from_utf8(request.body()).unwrap());
    tracing::debug!("got event headers {:?}", request.headers());
    if !Event::verify_payload(&request, opts.sign_secret.secret()) {
        return (StatusCode::BAD_REQUEST, "Invalid signature".to_string());
    }

    if let Some(id) = request.headers().get("Twitch-Eventsub-Message-Id") {
        if cache.get(id).await.is_none() {
            cache.insert(id.clone(), (), 400).await;
        } else {
            tracing::debug!("got already seen event");
            return (StatusCode::OK, "".to_string());
        }
    }

    // Event is verified, now do stuff.
    let event = Event::parse_http(&request).unwrap();
    //let event = Event::parse(std::str::from_utf8(request.body()).unwrap()).unwrap();
    tracing::info_span!("valid_event", event=?event);
    tracing::info!("got event!");

    if let Some(ver) = event.get_verification_request() {
        tracing::info!("subscription was verified");
        return (StatusCode::OK, ver.challenge.clone());
    }

    if event.is_revocation() {
        tracing::info!("subscription was revoked");
        return (StatusCode::OK, "".to_string());
    }
    use twitch_eventsub::{Message as M, Payload as P};

    // TODO: Some people have reported wierd bouncing when subscribing to stream.online/stream.offline, track this somehow.

    match event {
        Event::ChannelUpdateV2(P {
            message: M::Notification(_notification),
            ..
        }) => {}
        Event::StreamOnlineV1(P {
            message:
                M::Notification(StreamOnlineV1Payload {
                    broadcaster_user_id,
                    started_at,
                    ..
                }),
            ..
        }) if broadcaster_user_id == config.broadcaster.id => {
            tracing::info!(broadcaster_id=?broadcaster_user_id, "sending live status to clients");
            let _ = sender.send(LiveStatus::Live {
                started_at,
                url: config.broadcaster_url.clone(),
            });
        }
        Event::StreamOfflineV1(P {
            message:
                M::Notification(StreamOfflineV1Payload {
                    broadcaster_user_id,
                    ..
                }),
            ..
        }) if broadcaster_user_id == config.broadcaster.id => {
            tracing::info!(broadcaster_id=?broadcaster_user_id, "sending offline status to clients");
            let _ = sender.send(LiveStatus::Offline {
                url: config.broadcaster_url.clone(),
            });
        }
        Event::StreamOnlineV1(P {
            message: M::Notification(_),
            ..
        })
        | Event::StreamOfflineV1(P {
            message: M::Notification(_),
            ..
        }) => {
            tracing::info!("got online/offline status for another broadcaster, ignoring");
        }
        _ => {}
    }
    (StatusCode::OK, String::default())
}

pub fn stream_url_from_user(user: &UserNameRef) -> String { format!("https://twitch.tv/{user}") }

pub async fn refresher(
    client: HelixClient<'static, reqwest::Client>,
    token: Arc<RwLock<AppAccessToken>>,
    client_id: ClientId,
    client_secret: ClientSecret,
) -> eyre::Result<()> {
    loop {
        tracing::info!("hello!");
        tokio::time::sleep(token.read().await.expires_in() - tokio::time::Duration::from_secs(20))
            .await;
        let t = &mut *token.write().await;
        *t = AppAccessToken::get_app_access_token(
            client.get_client(),
            client_id.clone(),
            client_secret.clone(),
            vec![],
        )
        .await?;
    }
}

#[tracing::instrument(skip(client, token))]
pub async fn is_live<'a>(
    config: &'a Config,
    client: &HelixClient<'_, reqwest::Client>,
    token: &AppAccessToken,
) -> eyre::Result<LiveStatus> {
    tracing::info!("checking if live");
    if let Some(stream) = client
        .req_get(
            helix::streams::get_streams::GetStreamsRequest::user_ids(
                &[config.broadcaster.id.as_ref()][..],
            ),
            token,
        )
        .await
        .wrap_err_with(|| "could not check live streams")?
        .data
        .get(0)
    {
        Ok(LiveStatus::Live {
            started_at: stream.started_at.clone(),
            url: config.broadcaster_url.clone(),
        })
    } else {
        Ok(LiveStatus::Offline {
            url: config.broadcaster_url.clone(),
        })
    }
}

pub async fn checker(
    sender: Arc<watch::Sender<LiveStatus>>,
    config: Arc<Config>,
    client: HelixClient<'static, reqwest::Client>,
    token: Arc<RwLock<AppAccessToken>>,
) -> eyre::Result<()> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600));
    loop {
        let last = sender.borrow().clone();
        interval.tick().await;
        match is_live(&config, &client, &*token.read().await).await {
            Ok(live) => {
                if live != last {
                    sender.send(live)?;
                }
            }
            Err(e) => {
                tracing::error!("{}", e);
                if let Some(helix::HelixRequestGetError::Error {
                    status: hyper::StatusCode::FORBIDDEN,
                    ..
                }) = e.root_cause().downcast_ref::<helix::HelixRequestGetError>()
                {
                    tracing::warn!("Token needs to be refreshed");
                }
            }
        }
    }
    #[allow(unreachable_code)]
    Ok::<(), eyre::Report>(())
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiveStatus {
    Live {
        started_at: types::Timestamp,
        url: String,
    },
    Offline {
        url: String,
    },
}

impl LiveStatus {
    /// Returns `true` if the live status is [`Live`].
    ///
    /// [`Live`]: LiveStatus::Live
    pub fn is_live(&self) -> bool { matches!(self, Self::Live { .. }) }

    /// Returns `true` if the live status is [`Offline`].
    ///
    /// [`Offline`]: LiveStatus::Offline
    pub fn is_offline(&self) -> bool { matches!(self, Self::Offline { .. }) }

    pub fn to_message(&self) -> eyre::Result<ws::Message> {
        #[derive(serde::Serialize)]
        struct Msg {
            html: String,
            live: bool,
        }
        let msg = match self {
            Self::Live { .. } => Msg {
                html: "Yes".to_string(),
                live: true,
            },
            Self::Offline { .. } => Msg {
                html: "No".to_string(),
                live: false,
            },
        };
        Ok(ws::Message::Text(
            serde_json::to_string(&msg).wrap_err_with(|| "could not make into a message")?,
        ))
    }
}

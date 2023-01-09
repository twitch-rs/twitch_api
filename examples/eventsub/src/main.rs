#![warn(clippy::unwrap_in_result)]
pub mod opts;
pub mod twitch;
pub mod util;

use clap::Parser;
pub use opts::SignSecret;
use twitch::LiveStatus;

use std::{error::Error, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::{get, get_service, post},
    Extension, Router,
};

use askama_axum::Template;
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use opts::Opts;

use eyre::Context;

use reqwest::StatusCode;
use tokio::{sync::watch, task::JoinHandle};
use tower_http::{catch_panic::CatchPanicLayer, services::ServeDir, trace::TraceLayer};
use twitch_api::{client::ClientDefault, HelixClient};

use crate::twitch::stream_url_from_user;

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    util::install_utils()?;
    let opts = Opts::parse();

    tracing::debug!(
        "App started!\n{}",
        Opts::try_parse_from(["app", "--version"])
            .unwrap_err()
            .to_string()
    );

    run(&opts)
        .await
        .with_context(|| "when running application")?;

    Ok(())
}

pub async fn run(opts: &Opts) -> eyre::Result<()> {
    let client: HelixClient<'static, _> = twitch_api::HelixClient::with_client(
        <reqwest::Client>::default_client_with_name(Some(
            "twitch-rs/eventsub"
                .parse()
                .wrap_err_with(|| "when creating header name")
                .unwrap(),
        ))
        .wrap_err_with(|| "when creating client")?,
    );

    let token = twitch_oauth2::AppAccessToken::get_app_access_token(
        &client,
        opts.client_id.clone(),
        opts.client_secret.clone(),
        vec![],
    )
    .await?;

    let broadcaster = client
        .get_user_from_login(&opts.broadcaster_login, &token)
        .await?
        .ok_or_else(|| eyre::eyre!("broadcaster not found"))?;

    let config = Arc::new(Config {
        broadcaster_url: stream_url_from_user(&broadcaster.login),
        broadcaster,
        website_url: opts.website.clone(),
    });

    let live = twitch::is_live(&config, &client, &token).await?;

    let token = Arc::new(tokio::sync::RwLock::new(token));
    let (sender, recv) = watch::channel(live);
    let sender = Arc::new(sender);
    let retainer = Arc::new(retainer::Cache::<axum::http::HeaderValue, ()>::new());
    let ret = retainer.clone();
    let retainer_cleanup = async move {
        ret.monitor(10, 0.50, tokio::time::Duration::from_secs(86400 / 2))
            .await;
        Ok::<(), eyre::Report>(())
    };

    let app = Router::new()
        .route(
            "/ws",
            get({
                let recv = recv.clone();
                move |ws| handler(ws, recv)
            }),
        )
        .route(
            "/",
            get(move |config| serve_index(recv.borrow().clone(), config)),
        )
        .route(
            "/twitch/eventsub",
            post({
                move |sender, opts, config, cache, request| {
                    twitch::twitch_eventsub(sender, opts, config, cache, request)
                }
            }),
        )
        .nest_service(
            "/static",
            get_service(ServeDir::new("./static/")).handle_error(|error| async move {
                tracing::error!("{}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unhandled internal error".to_string(),
                )
            }),
        )
        .layer(
            tower::ServiceBuilder::new()
                //.layer(axum::error_handling::HandleErrorLayer::new(handle_error))
                .layer(Extension(client.clone()))
                .layer(Extension(config.clone()))
                .layer(Extension(sender.clone()))
                .layer(Extension(retainer.clone()))
                .layer(Extension(Arc::new(opts.clone())))
                .layer(TraceLayer::new_for_http().on_failure(
                    |error, _latency, _span: &tracing::Span| {
                        tracing::error!(error=%error);
                    },
                ))
                .layer(CatchPanicLayer::new()),
        );

    let server = tokio::spawn(async move {
        axum::Server::bind(
            &"0.0.0.0:80"
                .parse()
                .wrap_err_with(|| "when parsing address")?,
        )
        .serve(app.into_make_service())
        .await
        .wrap_err_with(|| "when serving")?;
        Ok::<(), eyre::Report>(())
    });
    tracing::info!("spinning up server!");
    if std::env::var("IM_A_SERVER").is_err() {
        tracing::warn!("to run this example you need to be a server with a url with tls,
        this means you're either behind a reverse-proxy, or you've setup this example to handle that");
        std::env::set_var("DEV", "1");
    }
    let r = tokio::try_join!(
        flatten(server),
        flatten(tokio::spawn(twitch::checker(
            sender.clone(),
            config.clone(),
            client.clone(),
            token.clone()
        ))),
        flatten(tokio::spawn(twitch::refresher(
            client.clone(),
            token.clone(),
            opts.client_id.clone(),
            opts.client_secret.clone()
        ))),
        flatten(tokio::spawn(twitch::eventsub_register(
            token.clone(),
            config.clone(),
            client.clone(),
            opts.website_callback.clone(),
            opts.sign_secret.clone()
        ))),
        flatten(tokio::spawn(retainer_cleanup)),
    );
    r?;
    Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, eyre::Report>>) -> Result<T, eyre::Report> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(e) => Err(e).wrap_err_with(|| "handling failed"),
    }
}

#[derive(Debug)]
pub struct Config {
    broadcaster: twitch_api::helix::users::User,
    broadcaster_url: String,
    website_url: String,
}

#[derive(Template)]
#[template(path = "is_live.html")]
struct IsLiveTemplate<'a> {
    is_live: bool,
    broadcaster_url: &'a str,
    broadcaster_display: &'a twitch_api::types::DisplayNameRef,
    broadcaster_profile_picture: Option<&'a str>,
    website_url: &'a str,
}

impl<'a> IsLiveTemplate<'a> {
    fn new(live: LiveStatus, config: &'a Config) -> Self {
        Self {
            is_live: live.is_live(),
            broadcaster_url: &config.broadcaster_url,
            broadcaster_display: &config.broadcaster.display_name,
            broadcaster_profile_picture: config.broadcaster.profile_image_url.as_deref(),
            website_url: &config.website_url,
        }
    }
}

async fn serve_index(
    live: LiveStatus,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse + 'static {
    askama_axum::IntoResponse::into_response(IsLiveTemplate::new(live, &config))
}

async fn handler(ws: WebSocketUpgrade, watch: watch::Receiver<LiveStatus>) -> impl IntoResponse {
    ws.on_upgrade(|f| handle_socket(f, watch))
}

async fn handle_socket(socket: WebSocket, watch: watch::Receiver<LiveStatus>) {
    let (sender, receiver) = socket.split();

    if let Err(e) = tokio::try_join!(
        flatten(tokio::spawn(write(sender, watch))),
        flatten(tokio::spawn(read(receiver)))
    )
    .wrap_err_with(|| "in stream join")
    {
        tracing::error!(error=%e, "socket failed")
    }
}
// Reads, basically only responds to pongs. Should not be a need for refreshes, but maybe.
async fn read(mut receiver: SplitStream<WebSocket>) -> Result<(), eyre::Report> {
    while let Some(msg) = receiver.next().await {
        tracing::debug!(message = ?msg, "got message")
    }
    Ok(())
}

// Sends live status to clients.
async fn write(
    mut sender: SplitSink<WebSocket, Message>,
    mut watch: watch::Receiver<LiveStatus>,
) -> Result<(), eyre::Report> {
    while watch.changed().await.is_ok() {
        let val = watch.borrow().clone();
        if let Ok(msg) = val.to_message() {
            if let Err(error) = sender.send(msg).await {
                if let Some(e) = error.source() {
                    if let Some(tokio_tungstenite::tungstenite::error::Error::ConnectionClosed) =
                        e.downcast_ref()
                    {
                        // NOOP
                    } else {
                        return Err(error.into());
                    }
                }
            };
        }
    }
    Ok(())
}

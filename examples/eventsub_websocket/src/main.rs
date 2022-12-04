#![warn(clippy::unwrap_in_result)]
pub mod opts;
pub mod util;
pub mod websocket;

use clap::Parser;
pub use opts::Secret;
use twitch_oauth2::UserToken;

use std::sync::Arc;

use opts::Opts;

use eyre::Context;

use tokio::{sync::RwLock, task::JoinHandle};
use twitch_api::{client::ClientDefault, HelixClient};

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

    tracing::debug!(opts = ?opts);

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

    let token = util::get_access_token(client.get_client(), opts).await?;
    let token: Arc<RwLock<UserToken>> = Arc::new(RwLock::new(token));
    let retainer = Arc::new(retainer::Cache::<String, ()>::new());
    let ret = retainer.clone();
    let retainer_cleanup = async move {
        ret.monitor(10, 0.50, tokio::time::Duration::from_secs(86400 / 2))
            .await;
        Ok::<(), eyre::Report>(())
    };
    let user_id = if let Some(ref id) = opts.channel_id {
        id.clone().into()
    } else if let Some(ref login) = opts.channel_login {
        client
            .get_user_from_login(login, &*token.read().await)
            .await?
            .ok_or_else(|| eyre::eyre!("no user found with name {login}"))?
            .id
    } else {
        token.read().await.user_id.clone()
    };

    let websocket_client = websocket::WebsocketClient {
        session_id: None,
        token,
        client,
        user_id,
        connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
    };

    let websocket_client = {
        let opts = opts.clone();
        async move { websocket_client.run(&opts).await }
    };

    let r = tokio::try_join!(
        flatten(tokio::spawn(retainer_cleanup)),
        flatten(tokio::spawn(websocket_client))
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

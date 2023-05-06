#![warn(clippy::unwrap_in_result)]
pub mod opts;
pub mod util;
pub mod websocket;

use clap::Parser;
pub use opts::Secret;

use std::sync::Arc;

use opts::Opts;

use eyre::Context;

use twitch_api::{client::ClientDefault, HelixClient};

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    // Setup dotenv, tracing and error reporting with eyre
    util::install_utils()?;
    let opts = Opts::parse();

    tracing::debug!(
        "App started!\n{}",
        Opts::try_parse_from(["app", "--version"])
            .unwrap_err()
            .to_string()
    );

    tracing::debug!(opts = ?opts);

    run(Arc::new(opts))
        .await
        .with_context(|| "when running application")?;

    Ok(())
}

/// Run the application
pub async fn run(opts: Arc<Opts>) -> eyre::Result<()> {
    // Create the HelixClient, which is used to make requests to the Twitch API
    let client: HelixClient<_> = twitch_api::HelixClient::with_client(
        <reqwest::Client>::default_client_with_name(Some(
            "twitch-rs/eventsub"
                .parse()
                .wrap_err_with(|| "when creating header name")
                .unwrap(),
        ))
        .wrap_err_with(|| "when creating client")?,
    );

    // Get the access token from the cli, dotenv or an oauth service
    let token: twitch_oauth2::UserToken =
        util::get_access_token(client.get_client(), &opts).await?;

    // Get the user id of the channel we want to listen to
    let user_id = if let Some(ref id) = opts.channel_id {
        id.clone().into()
    } else if let Some(ref login) = opts.channel_login {
        client
            .get_user_from_login(login, &token)
            .await?
            .ok_or_else(|| eyre::eyre!("no user found with name {login}"))?
            .id
    } else {
        // Use the user id from the token if no channel is specified
        token.user_id.clone()
    };

    let websocket_client = websocket::WebsocketClient {
        session_id: None,
        token,
        client,
        user_id,
        connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
        opts,
    };

    let websocket_client = tokio::spawn(async move { websocket_client.run().await });

    tokio::try_join!(flatten(websocket_client))?;
    Ok(())
}

async fn flatten<T>(
    handle: tokio::task::JoinHandle<Result<T, eyre::Report>>,
) -> Result<T, eyre::Report> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(e) => Err(e).wrap_err_with(|| "handling failed"),
    }
}

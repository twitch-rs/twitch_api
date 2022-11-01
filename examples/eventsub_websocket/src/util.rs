use eyre::Context;
use twitch_oauth2::UserToken;

pub fn install_utils() -> eyre::Result<()> {
    let _ = dotenvy::dotenv(); //ignore error
    install_tracing();
    install_eyre()?;
    Ok(())
}

fn install_eyre() -> eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));
    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true);
    #[rustfmt::skip]
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .map(|f| {
            f.add_directive("hyper=error".parse().expect("could not make directive"))
                .add_directive("h2=error".parse().expect("could not make directive"))
                .add_directive("rustls=error".parse().expect("could not make directive"))
                .add_directive("tungstenite=error".parse().expect("could not make directive"))
                .add_directive("retainer=info".parse().expect("could not make directive"))
                .add_directive("want=info".parse().expect("could not make directive"))
                .add_directive("reqwest=info".parse().expect("could not make directive"))
                .add_directive("mio=info".parse().expect("could not make directive"))
            //.add_directive("tower_http=error".parse().unwrap())
        })
        .expect("could not make filter layer");

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[tracing::instrument(skip(client, token))]
pub async fn make_token<'a>(
    client: &'a impl twitch_oauth2::client::Client<'a>,
    token: impl Into<twitch_oauth2::AccessToken>,
) -> Result<UserToken, eyre::Report> {
    UserToken::from_existing(client, token.into(), None, None)
        .await
        .context("could not get/make access token")
        .map_err(Into::into)
}

#[tracing::instrument(skip(client, opts))]
pub async fn get_access_token(
    client: &reqwest::Client,
    opts: &crate::Opts,
) -> Result<UserToken, eyre::Report> {
    if let Some(ref access_token) = opts.access_token {
        make_token(client, access_token.secret().to_string()).await
    } else if let (Some(ref oauth_service_url), Some(ref pointer)) =
        (&opts.oauth2_service_url, &opts.oauth2_service_pointer)
    {
        tracing::info!(
            "using oauth service on `{}` to get oauth token",
            oauth_service_url
        );

        let mut request = client.get(oauth_service_url.as_str());
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
                    .context("when transforming oauth service response to json")?;
                make_token(
                    client,
                    service_response
                        .pointer(pointer)
                        .ok_or_else(|| eyre::eyre!("could not get a field on `{}`", pointer))?
                        .as_str()
                        .ok_or_else(|| eyre::eyre!("token is not a string"))?
                        .to_string(),
                )
                .await
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
            Err(e) => Err(e)
                .wrap_err_with(|| eyre::eyre!("calling oauth service on `{}`", &oauth_service_url)),
        }
    } else {
        panic!("got empty vals for token cli group")
    }
}

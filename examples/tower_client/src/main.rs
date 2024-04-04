use futures::TryStreamExt;
use http::header::USER_AGENT;
use hyper_tls::HttpsConnector;
use hyper_util::rt::TokioExecutor;
use tower::ServiceBuilder;
use tower_http::{
    classify::StatusInRangeAsFailures, decompression::DecompressionLayer,
    set_header::SetRequestHeaderLayer, trace::TraceLayer,
};
use twitch_api::{client::TowerService, HelixClient};
use twitch_oauth2::{AccessToken, UserToken};

fn main() {
    use std::error::Error;
    if let Err(err) = run() {
        println!("Error: {err}");
        let mut e: &'_ dyn Error = err.as_ref();
        while let Some(cause) = e.source() {
            println!("Caused by: {cause}");
            e = cause;
        }
    }
}

#[tokio::main]
/// Run the application
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();
    let mut args = std::env::args().skip(1);

    let tower_client = ServiceBuilder::new()
        // This buffer is trivial, we need the service to be clonable
        .buffer(1)
        // Setup tracing
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
        .layer(TraceLayer::new_for_http())
        // Set the user-agent
        .layer(SetRequestHeaderLayer::overriding(
            USER_AGENT,
            twitch_api::client::user_agent(Some("twitch_api/examples/tower_client".parse()?))?,
        ))
        .layer(DecompressionLayer::new())
        // Rate limit to 1 request per second
        .layer(tower::limit::RateLimitLayer::new(
            1,
            std::time::Duration::from_secs(1),
        ))
        // Use hyper
        .service(
            hyper_util::client::legacy::Builder::new(TokioExecutor::new())
                .build::<_, http_body_util::Full<hyper::body::Bytes>>(HttpsConnector::new()),
        );

    tracing::info!("Creating client");
    let client: HelixClient<Box<dyn twitch_api::HttpClient<Error = _>>> =
        HelixClient::with_client(Box::new(TowerService::new(tower_client)));

    tracing::info!("Getting token");
    let token = UserToken::from_existing(
        &client,
        std::env::var("TWITCH_TOKEN")
            .ok()
            .or_else(|| args.next())
            .map(AccessToken::new)
            .expect("Please set env: TWITCH_TOKEN or pass token as first argument"),
        None,
        None,
    )
    .await?;

    tracing::info!("getting users");

    let futs: futures::stream::FuturesUnordered<_> =
        std::iter::repeat_with(|| client.get_user_from_login("twitchdev", &token))
            .take(100)
            .collect();
    let res: Vec<_> = futs.try_collect().await?;
    tracing::info!("Got {} results", res.len());
    Ok(())
}

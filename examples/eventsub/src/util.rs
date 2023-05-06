/// Setup dotenv, tracing and error reporting with eyre
pub fn install_utils() -> eyre::Result<()> {
    let _ = dotenvy::dotenv(); //ignore error
    install_tracing();
    install_eyre()?;
    Ok(())
}

/// Install eyre and setup a panic hook
fn install_eyre() -> eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));
    Ok(())
}

/// Install tracing with a specialized filter
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
            //.add_directive("tower_http=error".parse().unwrap())
        })
        .expect("could not make filter layer");

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

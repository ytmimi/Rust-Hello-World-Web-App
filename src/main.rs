use hello_world_web_server::get_app;
use std::net::SocketAddr;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // setup environment variable filters used to setup tracing (logging)
    let env_logging_filter = tracing_subscriber::EnvFilter::try_from_env("LOGGING_DIRECTIVE")
        .unwrap_or_else(|_| "hello_world_web_server=info,axum::rejection=trace".into());

    // initialize tracing (logging). Now we can make calls to the `trace!`, `debug!`, `info!`,
    // `warn!`, and `error!` macros provided by the `tracing` crate
    tracing_subscriber::registry()
        .with(env_logging_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = get_app().layer(
        // Add logging middleware layer
        TraceLayer::new_for_http(),
    );

    // Run our app!
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("able to bind to host and port");
}

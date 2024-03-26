use std::{env, net::SocketAddr};

use anyhow::{Context, Result};
use tracing::info;

use pi_web::{app::App, config::Settings, telemetry::init_tracing};

fn main() -> Result<()> {
    init_tracing();

    let config_file = env::args().nth(1).context("Missing config file")?;
    let settings = Settings::parse(&config_file).context("Incomplete configuration")?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()?;

    runtime.block_on(serve(settings))?;

    Ok(())
}

async fn serve(settings: Settings) -> Result<()> {
    info!("Creating listener");

    let listener = tokio::net::TcpListener::bind((settings.deploy.addr, settings.deploy.port))
        .await
        .context("Could not create listener")?;

    info!("Building app");

    let app = App::new(settings.app)
        .build()
        .await
        .context("Could not create app")?
        .into_make_service_with_connect_info::<SocketAddr>();

    info!("Serving app");

    axum::serve(listener, app).await.context("App failed")?;

    Ok(())
}

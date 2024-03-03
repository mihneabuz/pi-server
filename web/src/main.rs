use std::env;

use anyhow::{Context, Result};

use pi_web::{app::App, config::Settings, telemetry::init_tracing};

fn main() -> Result<()> {
    init_tracing();

    let config_file = env::args().nth(1).context("Missing config file")?;
    let settings = Settings::parse(&config_file)?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()?;

    runtime.block_on(serve(settings))?;

    Ok(())
}

async fn serve(settings: Settings) -> Result<()> {
    let listener = tokio::net::TcpListener::bind((settings.deploy.addr, settings.deploy.port))
        .await
        .context("Could not create listener")?;

    let app = App::new(settings.app)
        .build()
        .await
        .context("Could not create app")?;

    axum::serve(listener, app).await.context("App failed")?;

    Ok(())
}

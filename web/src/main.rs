use std::net::Ipv4Addr;

use anyhow::Result;
use pi_web::{app::App, config::Settings, telemetry::init_tracing};

fn main() -> Result<()> {
    init_tracing();

    let settings = Settings::parse()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), settings.port))
            .await
            .unwrap();

        axum::serve(listener, App::new(settings).build())
            .await
            .unwrap();
    });

    Ok(())
}

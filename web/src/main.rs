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
        let listener = tokio::net::TcpListener::bind((settings.deploy.addr, settings.deploy.port))
            .await
            .unwrap();

        axum::serve(listener, App::new(settings.app).build().await)
            .await
            .unwrap();
    });

    Ok(())
}

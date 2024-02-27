mod middleware;
mod pages;
mod telemetry;

use std::net::Ipv4Addr;

use axum::{http::StatusCode, response::IntoResponse, Router};
use pages::home::HomePage;
use pi_web::Page;
use telemetry::{init_tracing, MakeSpanWithId};
use tower_http::{services::ServeDir, trace::TraceLayer};

fn main() {
    init_tracing();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(std::thread::available_parallelism().unwrap().get())
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(0, 0, 0, 0), 3000))
            .await
            .unwrap();

        axum::serve(listener, app()).await.unwrap();
    })
}

fn app() -> Router {
    let trace_layer = TraceLayer::new_for_http().make_span_with(MakeSpanWithId);

    Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/", HomePage::app())
        .layer(trace_layer)
        .fallback(not_found)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

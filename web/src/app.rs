use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{config::Settings, pages::home::HomePage, telemetry::MakeSpanWithId, Page};

pub struct App;

impl App {
    pub fn new(_settings: Settings) -> Self {
        Self {}
    }

    pub fn build(self) -> Router {
        let trace_layer = TraceLayer::new_for_http().make_span_with(MakeSpanWithId);

        Router::new()
            .nest_service("/public", ServeDir::new("public"))
            .nest_service("/", HomePage::app())
            .layer(trace_layer)
            .fallback(not_found)
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    config::AppSettings,
    pages::{BlogApp, HomeApp, Module, ProjectsApp},
    telemetry::MakeSpanWithId,
};

pub struct App {
    settings: AppSettings,
}

impl App {
    pub fn new(settings: AppSettings) -> Self {
        Self { settings }
    }

    pub async fn build(self) -> Result<Router> {
        let trace_layer = TraceLayer::new_for_http().make_span_with(MakeSpanWithId);

        Ok(Router::new()
            .nest_service("/public", ServeDir::new(self.settings.public_dir))
            .merge(HomeApp.app())
            .merge(BlogApp::build(self.settings.blogs_dir).await?.app())
            .merge(ProjectsApp.app())
            .layer(trace_layer)
            .fallback(not_found))
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

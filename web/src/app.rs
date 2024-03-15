use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    config::{AppSettings, StaticFileCompression},
    info_cached_memory,
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

    pub fn serve_static(&self) -> ServeDir {
        let dir = &self.settings.public_dir;

        match self.settings.compression {
            StaticFileCompression::None => ServeDir::new(dir),
            StaticFileCompression::Gzip => ServeDir::new(dir).precompressed_gzip(),
        }
    }

    pub async fn build(self) -> Result<Router> {
        let trace_layer = TraceLayer::new_for_http().make_span_with(MakeSpanWithId);

        let app = Router::new()
            .nest_service("/public", self.serve_static())
            .merge(HomeApp.app())
            .merge(BlogApp::build(self.settings.blogs_dir).await?.app())
            .merge(ProjectsApp.app())
            .layer(trace_layer)
            .fallback(not_found);

        info_cached_memory!();

        Ok(app)
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

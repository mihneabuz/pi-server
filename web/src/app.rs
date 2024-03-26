use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::services::ServeDir;

use crate::{
    config::{AppSettings, StaticFileCompression},
    info_cached_memory,
    middleware::{RateLimit, Tracing},
    pages::{BlogApp, HomeApp, ProjectsApp},
    router::RouterExt,
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
        let app = Router::new()
            .nest_service("/public", self.serve_static())
            .merge_module(HomeApp)
            .merge_module(BlogApp::build(self.settings.blogs_dir).await?)
            .merge_module(ProjectsApp)
            .middleware(Tracing)
            .middleware(RateLimit::new(self.settings.rate_limit))
            .fallback(not_found);

        info_cached_memory!();

        Ok(app)
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

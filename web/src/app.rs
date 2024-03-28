use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Router};
use tower_http::services::ServeDir;

use crate::{
    config::{AppSettings, StaticFileCompression},
    info_cached_memory,
    middleware::{ConnectionLimit, RateLimit, Stats, Tracing},
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
        let serve = ServeDir::new(&self.settings.public_dir);

        match self.settings.compression {
            StaticFileCompression::None => serve,
            StaticFileCompression::Gzip => serve.precompressed_gzip(),
            StaticFileCompression::Brotli => serve.precompressed_br(),
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
            .middleware(ConnectionLimit::new(self.settings.conn_limit))
            .middleware(Stats)
            .fallback(not_found);

        #[cfg(debug_assertions)]
        let app = app.layer(tower_livereload::LiveReloadLayer::new());

        info_cached_memory!();

        Ok(app)
    }
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Leave...")
}

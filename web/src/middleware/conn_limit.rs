use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Response, StatusCode},
    response::IntoResponse,
    BoxError,
};
use tower::{limit::GlobalConcurrencyLimitLayer, load_shed::LoadShedLayer, ServiceBuilder};

use crate::middleware::Middleware;

pub struct ConnectionLimit {
    limit: u32,
}

impl ConnectionLimit {
    pub fn new(limit: u32) -> Self {
        Self { limit }
    }
}

impl Middleware for ConnectionLimit {
    fn attach<S>(self, router: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        tracing::info!("MAX CONNECTIONS: {}", self.limit);
        router.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(connection_limit_error))
                .layer(LoadShedLayer::new())
                .layer(GlobalConcurrencyLimitLayer::new(self.limit as usize)),
        )
    }
}

async fn connection_limit_error(_: BoxError) -> Response<Body> {
    (StatusCode::SERVICE_UNAVAILABLE, "Server overloaded. Sorry").into_response()
}

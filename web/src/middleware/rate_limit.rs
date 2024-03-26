use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, BoxError};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

use super::Middleware;

pub struct RateLimit;

impl Middleware for RateLimit {
    fn attach<S>(self, router: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        router.layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(rate_limit_error))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(10, Duration::from_secs(1))),
        )
    }
}

async fn rate_limit_error(_: BoxError) -> impl IntoResponse {
    (StatusCode::TOO_MANY_REQUESTS, "Slow down please")
}

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_governor::{governor::GovernorConfigBuilder, GovernorError, GovernorLayer};

use crate::{config::RateLimitSettings, middleware::Middleware};

pub struct RateLimit {
    max_allowed: u32,
    per_seconds: u32,
}

impl RateLimit {
    pub fn new(settings: RateLimitSettings) -> Self {
        Self {
            max_allowed: settings.max_allowed,
            per_seconds: settings.per_seconds,
        }
    }
}

impl Middleware for RateLimit {
    fn attach<S>(self, router: axum::Router<S>) -> axum::Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        let config = Box::new(
            GovernorConfigBuilder::default()
                .per_second(self.per_seconds as u64)
                .burst_size(self.max_allowed)
                .error_handler(rate_limit_error)
                .finish()
                .unwrap(),
        );

        router.layer(GovernorLayer {
            config: Box::leak(config),
        })
    }
}

fn rate_limit_error(_: GovernorError) -> Response<Body> {
    (StatusCode::TOO_MANY_REQUESTS, "Slow down").into_response()
}

use axum::Router;
use tower_http::trace::{MakeSpan, TraceLayer};

use crate::middleware::Middleware;

#[derive(Clone)]
struct MakeSpanWithId;

impl<B> MakeSpan<B> for MakeSpanWithId {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        let request_id = ulid::Ulid::new();
        tracing::span!(
            tracing::Level::TRACE,
            "request",
            id = %request_id,
            uri = %request.uri(),
            method = %request.method(),
        )
    }
}

pub struct Tracing;

impl Middleware for Tracing {
    fn attach<S>(self, router: Router<S>) -> Router<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        router.layer(TraceLayer::new_for_http().make_span_with(MakeSpanWithId))
    }
}

use tower_http::trace::MakeSpan;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace")))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[derive(Clone)]
pub struct MakeSpanWithId;

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

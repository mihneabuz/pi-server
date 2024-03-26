mod rate_limit;
mod tracing;

pub use rate_limit::*;
pub use tracing::*;

use axum::Router;

pub trait Middleware {
    fn attach<S>(self, router: Router<S>) -> Router<S>
    where
        S: Clone + Send + Sync + 'static;
}

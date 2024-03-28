mod conn_limit;
mod rate_limit;
mod stats;
mod tracing;

pub use conn_limit::*;
pub use rate_limit::*;
pub use stats::*;
pub use tracing::*;

use axum::Router;

pub trait Middleware {
    fn attach<S>(self, router: Router<S>) -> Router<S>
    where
        S: Clone + Send + Sync + 'static;
}

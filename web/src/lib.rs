pub mod app;
pub mod components;
pub mod config;
pub mod middleware;
pub mod pages;
pub mod telemetry;

use axum::Router;
use maud::{html, Markup, DOCTYPE};

pub trait Page {
    fn app() -> Router;

    #[allow(async_fn_in_trait)]
    async fn head() -> Markup;

    #[allow(async_fn_in_trait)]
    async fn body() -> Markup;

    #[allow(async_fn_in_trait)]
    async fn index() -> Markup {
        html! {
            (DOCTYPE)
            html {
                head {
                    (Self::head().await)
                }
                body {
                    (Self::body().await)
                }
            }
        }
    }
}

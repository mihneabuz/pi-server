use axum::{routing::get, Router};
use maud::{html, Markup};

use pi_web::{components::HeadBuilder, Page};

pub struct HomePage;
impl Page for HomePage {
    fn app() -> Router {
        Router::new().route("/", get(Self::index))
    }

    async fn head() -> Markup {
        HeadBuilder::new("Home").build()
    }

    async fn body() -> Markup {
        html! {
            div class="py-2 px-4 bg-green-500 w-72 h-72" {
                h1 { "Hello, World!" }
            }
        }
    }
}

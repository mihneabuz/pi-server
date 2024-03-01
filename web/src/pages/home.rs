use axum::{routing::get, Router};
use maud::{html, Markup};

use crate::{components::HeadBuilder, pages::Page};

pub struct HomePage;

impl Page for HomePage {
    const TITLE: &'static str = "Home";
    const BASE_PATH: &'static str = "/";

    fn app(self) -> Router {
        Router::new().route(Self::BASE_PATH, get(Self::index))
    }
}

impl HomePage {
    async fn index() -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        Self::page(head, Self::body())
    }

    fn body() -> Markup {
        html! {
            div class="mx-auto" {
                h3 class="text-teal-500 text-2xl italic" { "Hi, my name is" }
                h1 class="text-slate-100 text-4xl font-bold" { "Mihnea Buzatu" }
            }
        }
    }
}

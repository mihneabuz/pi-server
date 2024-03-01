use axum::{routing::get, Router};
use maud::{html, Markup};

use crate::{components::HeadBuilder, pages::Page};

pub struct ProjectsPage;

impl Page for ProjectsPage {
    const TITLE: &'static str = "Projects";
    const BASE_PATH: &'static str = "/projects";

    fn app(self) -> Router {
        Router::new().route(Self::BASE_PATH, get(Self::index))
    }
}

impl ProjectsPage {
    async fn index() -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        Self::page(head, Self::body())
    }

    fn body() -> Markup {
        html! {
            h1 { "Hello, Projects!" }
        }
    }
}

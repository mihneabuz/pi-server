use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{Page, NAV_PAGES},
};

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
        let nav = NavBuilder::new(&NAV_PAGES).active(Self::BASE_PATH).build();

        html! {
            (DOCTYPE)
            html {
                head { (head) }
                body class="h-full bg-neutral-800 flex flex-col" {
                    (nav)
                    (Self::content())
                }
            }
        }
    }

    fn content() -> Markup {
        html! {
            h1 { "Hello, Projects!" }
        }
    }
}

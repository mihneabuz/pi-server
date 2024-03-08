use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{Module, NAV_PAGES},
};

pub struct ProjectsApp;

impl Module for ProjectsApp {
    const TITLE: &'static str = "Projects";
    const BASE_PATH: &'static str = "/projects";

    fn app(self) -> Router {
        Router::new().route(Self::BASE_PATH, get(Self::index))
    }
}

impl ProjectsApp {
    async fn index() -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        let nav = NavBuilder::new(&NAV_PAGES).active(Self::BASE_PATH).build();

        html! {
            (DOCTYPE)
            html class="h-full" {
                head { (head) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (nav)
                    (Self::content())
                }
            }
        }
    }

    fn content() -> Markup {
        html! {
            div class="flex justify-center items-center grow" {
                h1 class="text-6xl font-bold text-slate-200" {
                    "🚧 Under construction! 🚧"
                }
            }
        }
    }
}

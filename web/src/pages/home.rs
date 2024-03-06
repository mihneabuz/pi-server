use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{BlogPage, Page, ProjectsPage, NAV_PAGES},
};

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
                div class="w-[60%]" {
                    h3 class="my-1 text-2xl italic text-teal-500" { "Hi, my name is" }
                    h1 class="my-2 text-4xl font-bold text-slate-200" { "Mihnea Buzatu" }

                    h3 class="my-4 text-2xl italic text-teal-500" {
                        "I like to build "
                        a href=(ProjectsPage::BASE_PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "stuff" }
                        " and sometimes "
                        a href=(BlogPage::BASE_PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "write" }
                        " about it."
                    }
                }
            }
        }
    }
}

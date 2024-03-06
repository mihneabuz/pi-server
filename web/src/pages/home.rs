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
                body class="h-full bg-neutral-800 flex flex-col" {
                    (nav)
                    (Self::content())
                }
            }
        }
    }

    fn content() -> Markup {
        html! {
            div class="flex grow justify-center items-center" {
                div class="w-[60%]" {
                    h3 class="my-1 text-teal-500 text-2xl italic" { "Hi, my name is" }
                    h1 class="my-2 text-slate-200 text-4xl font-bold" { "Mihnea Buzatu" }

                    h3 class="my-4 text-teal-500 text-2xl italic" {
                        "I like to build "
                        a href=(ProjectsPage::BASE_PATH)
                            class="text-teal-400 hover:text-teal-200 transition-all" { "stuff" }
                        " and sometimes "
                        a href=(BlogPage::BASE_PATH)
                            class="text-teal-400 hover:text-teal-200 transition-all" { "write" }
                        " about it."
                    }
                }
            }
        }
    }
}

use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{BlogApp, Module, ProjectsApp, NAV_PAGES},
};

pub struct HomeApp;

impl Module for HomeApp {
    const TITLE: &'static str = "Home";
    const BASE_PATH: &'static str = "/";

    fn app(self) -> Router {
        Router::new().route(Self::BASE_PATH, get(Self::index))
    }
}

impl HomeApp {
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
                div class="w-[80%] lg:w-[60%]" {
                    h3 class="my-1 text-3xl italic text-teal-500" { "Hi, my name is" }
                    h1 class="my-2 text-5xl font-bold text-slate-200" { "Mihnea Buzatu" }

                    h3 class="my-4 text-3xl italic text-teal-500" {
                        "I like to build "
                        a href=(ProjectsApp::BASE_PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "stuff" }
                        " and sometimes "
                        a href=(BlogApp::BASE_PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "write" }
                        " about it."
                    }
                }
            }
        }
    }
}

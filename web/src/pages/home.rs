use axum::Router;
use maud::{html, Markup, DOCTYPE};

use crate::{
    pages::{BlogApp, Module, ProjectsApp},
    static_page,
};

pub struct HomeApp;

impl Module for HomeApp {
    const PATH: &'static str = "/";
    const TITLE: &'static str = "Home";

    fn app(self) -> Router {
        Router::new().route(Self::PATH, static_page!(self.index()))
    }
}

impl HomeApp {
    fn index(&self) -> Markup {
        html! {
            (DOCTYPE)
            html class="h-full" {
                head { (self.head()) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (self.nav())
                    (self.content())
                }
            }
        }
    }

    fn content(&self) -> Markup {
        html! {
            div class="flex justify-center items-center grow" {
                div class="w-[80%] lg:w-[60%]" {
                    h3 class="my-1 text-4xl italic text-teal-500" { "Hi, my name is" }
                    h1 class="my-2 text-6xl font-bold text-slate-200" { "Mihnea Buzatu" }

                    h3 class="my-4 text-4xl italic text-teal-500" {
                        "I like to build "
                        a href=(ProjectsApp::PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "stuff" }
                        " and sometimes "
                        a href=(BlogApp::PATH)
                            class="text-teal-400 transition-all hover:text-teal-200" { "write" }
                        " about it."
                    }
                }
            }
        }
    }
}

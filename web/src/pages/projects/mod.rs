use axum::Router;
use maud::{html, Markup, DOCTYPE};

use crate::{pages::Module, static_page};

pub struct ProjectsApp;

impl Module for ProjectsApp {
    const PATH: &'static str = "/projects";
    const TITLE: &'static str = "Projects";

    fn app(self) -> Router {
        Router::new().route(Self::PATH, static_page!(self.index()))
    }
}

impl ProjectsApp {
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
                h1 class="text-6xl font-bold text-slate-200" {
                    "🚧 Under construction! 🚧"
                }
            }
        }
    }
}

mod demo;

use axum::Router;
use maud::{html, Markup};

use crate::{pages::Module, static_page};

use self::demo::DemoGame;

pub struct ProjectsApp;

impl Module for ProjectsApp {
    const PATH: &'static str = "/projects";
    const TITLE: &'static str = "Projects";

    fn app(self) -> Router {
        Router::new()
            .route(Self::PATH, static_page!(self.index()))
            .merge(DemoGame.app())
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

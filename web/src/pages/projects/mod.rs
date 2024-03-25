mod demo;

use axum::Router;
use maud::{html, Markup};

use crate::{components::Card, pages::Module, router::RouterExt, static_page};

use self::demo::DemoGame;

pub struct ProjectsApp;

impl Module for ProjectsApp {
    const PATH: &'static str = "/projects";
    const TITLE: &'static str = "Projects";

    fn app(self) -> Router {
        Router::new()
            .route(Self::PATH, static_page!(self.index()))
            .merge_module(DemoGame)
    }

    fn content(&self) -> Markup {
        let cards = [Card::new("Game of Life")
            .description("Demo app powered by WASM")
            .link_to(DemoGame::PATH)];

        html! {
            div class="grid grid-cols-1 gap-16 m-20 lg:grid-cols-2 2xl:grid-cols-3" {
                @for card in cards.into_iter() {
                    (card.build())
                }
            }
        }
    }
}

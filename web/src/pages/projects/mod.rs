use maud::{html, Markup};

use crate::pages::Module;

pub struct ProjectsApp;

impl Module for ProjectsApp {
    const PATH: &'static str = "/projects";
    const TITLE: &'static str = "Projects";

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

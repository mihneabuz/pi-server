use maud::{html, Markup};

use crate::pages::{BlogApp, Module, ProjectsApp};

pub struct HomeApp;

impl Module for HomeApp {
    const PATH: &'static str = "/";
    const TITLE: &'static str = "Home";

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

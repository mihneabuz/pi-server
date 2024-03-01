mod blog;
mod home;
mod projects;

pub use blog::*;
pub use home::*;
pub use projects::*;

use axum::Router;
use maud::{html, Markup, DOCTYPE};

use crate::components::{Nav, NavEntry};

pub trait Page {
    const TITLE: &'static str;
    const BASE_PATH: &'static str;

    fn app(self) -> Router;

    fn page(head: Markup, body: Markup) -> Markup {
        page(head, body, Self::BASE_PATH)
    }
}

const NAV_PAGES: [NavEntry; 3] = [
    (HomePage::TITLE, HomePage::BASE_PATH),
    (BlogPage::TITLE, BlogPage::BASE_PATH),
    (ProjectsPage::TITLE, ProjectsPage::BASE_PATH),
];

pub fn page(head: Markup, body: Markup, path: &'static str) -> Markup {
    let nav = Nav::new(&NAV_PAGES).active(path).build();

    html! {
        (DOCTYPE)
        html {
            head { (head) }
            body class="bg-neutral-800" {
                (nav)
                (body)
            }
        }
    }
}

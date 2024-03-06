mod blog;
mod home;
mod projects;

pub use blog::*;
pub use home::*;
pub use projects::*;

use axum::Router;

use crate::components::NavEntry;

pub trait Page {
    const TITLE: &'static str;
    const BASE_PATH: &'static str;

    fn app(self) -> Router;
}

const NAV_PAGES: [NavEntry; 3] = [
    (HomePage::TITLE, HomePage::BASE_PATH),
    (BlogPage::TITLE, BlogPage::BASE_PATH),
    (ProjectsPage::TITLE, ProjectsPage::BASE_PATH),
];

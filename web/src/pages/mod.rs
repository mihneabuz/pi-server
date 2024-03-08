mod blog;
mod home;
mod projects;

pub use blog::*;
pub use home::*;
pub use projects::*;

use axum::Router;

use crate::components::NavEntry;

pub trait Module {
    const TITLE: &'static str;
    const BASE_PATH: &'static str;

    fn app(self) -> Router;
}

const NAV_PAGES: [NavEntry; 3] = [
    (HomeApp::TITLE, HomeApp::BASE_PATH),
    (BlogApp::TITLE, BlogApp::BASE_PATH),
    (ProjectsApp::TITLE, ProjectsApp::BASE_PATH),
];

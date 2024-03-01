use std::path::PathBuf;

use axum::{routing::get, Router};
use maud::{html, Markup};

use crate::{components::HeadBuilder, pages::Page};

pub struct BlogPage {
    blogs_dir: PathBuf,
}

impl Page for BlogPage {
    const TITLE: &'static str = "Blog";
    const BASE_PATH: &'static str = "/blog";

    fn app(self) -> Router {
        Router::new().route(Self::BASE_PATH, get(Self::index))
    }
}

impl BlogPage {
    pub async fn build(blogs_dir: PathBuf) -> Self {
        Self { blogs_dir }
    }

    async fn index() -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        Self::page(head, Self::body())
    }

    fn body() -> Markup {
        html! {
            h1 { "Hello, blog!" }
        }
    }
}

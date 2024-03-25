mod entry;
mod loader;
mod renderer;

use std::path::Path;

use anyhow::Result;
use axum::Router;
use maud::{html, Markup};

use crate::{
    pages::{
        blog::{entry::Blog, loader::BlogLoader},
        Module,
    },
    router::RouterExt,
    static_page,
};

pub struct BlogApp {
    blogs: Vec<Blog>,
}

impl BlogApp {
    pub async fn build(blogs_dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            blogs: BlogLoader::read_dir(blogs_dir).await?,
        })
    }
}

impl Module for BlogApp {
    const PATH: &'static str = "/blog";
    const TITLE: &'static str = "Blog";

    fn app(self) -> Router {
        Router::new()
            .route(Self::PATH, static_page!(Module::index(&self)))
            .merge_modules(self.blogs.into_iter())
    }

    fn content(&self) -> Markup {
        html! {
            div class="grid grid-cols-1 gap-16 m-20 lg:grid-cols-2 2xl:grid-cols-3" {
                @for entry in self.blogs.iter() {
                    (entry.card())
                }
            }
        }
    }
}

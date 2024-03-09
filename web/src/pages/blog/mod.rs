mod entry;
mod loader;
mod renderer;

use std::path::PathBuf;

use anyhow::Result;
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{blog::entry::Blog, blog::loader::BlogLoader, Module, NAV_PAGES},
    static_page,
};

pub struct BlogApp {
    blogs: Vec<Blog>,
}

impl Module for BlogApp {
    const TITLE: &'static str = "Blog";
    const BASE_PATH: &'static str = "/blog";

    fn app(self) -> Router {
        let mut inner = Router::new().route("/", get(static_page!(self.index())));

        for blog in self.blogs {
            let path = format!("/{}", blog.title);
            inner = inner.route(&path, get(static_page!(blog.render())));
        }

        Router::new().nest(Self::BASE_PATH, inner)
    }
}

impl BlogApp {
    pub async fn build(blogs_dir: PathBuf) -> Result<Self> {
        let blogs = BlogLoader::read_dir(blogs_dir).await?;
        Ok(Self { blogs })
    }

    fn index(&self) -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        let nav = NavBuilder::new(&NAV_PAGES).active(Self::BASE_PATH).build();

        html! {
            (DOCTYPE)
            html class="h-full" {
                head { (head) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (nav)
                    div class="flex justify-center items-center grow" {
                        h1 class="text-6xl font-bold text-slate-200" {
                            "🚧 Under construction! 🚧"
                        }
                    }
                }
            }
        }
    }
}

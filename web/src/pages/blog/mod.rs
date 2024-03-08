mod item;

use std::path::PathBuf;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use tracing::{info, warn};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{Module, NAV_PAGES},
};

use item::BlogPage;

pub struct BlogApp {
    blogs: Vec<BlogPage>,
}

impl Module for BlogApp {
    const TITLE: &'static str = "Blog";
    const BASE_PATH: &'static str = "/blog";

    fn app(self) -> Router {
        let index = self.index();
        let mut inner = Router::new().route("/", get(move || async { index }));

        for blog in self.blogs {
            let path = format!("/{}", blog.title);
            let rendered = blog.render();
            inner = inner.route(&path, get(|| async { rendered }));
        }

        Router::new().nest(Self::BASE_PATH, inner)
    }
}

impl BlogApp {
    pub async fn build(blogs_dir: PathBuf) -> Result<Self> {
        let mut blogs = Vec::new();

        let mut entries = tokio::fs::read_dir(&blogs_dir).await?;
        while let Some(file) = entries.next_entry().await? {
            if !file.file_type().await?.is_file() {
                warn!(?file, "Unrecognized file type");
                continue;
            }

            let blog = BlogPage::read(blogs_dir.join(file.file_name()))
                .await
                .context("Invalid blog file")?;

            info!(title = blog.title, date = %blog.date, "Found blog");

            blogs.push(blog);
        }

        info!(count = blogs.len(), "Done loading blogs");

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

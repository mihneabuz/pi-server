mod render;

use std::path::PathBuf;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use tracing::{info, warn};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{Page, NAV_PAGES},
};

use render::Blog;

pub struct BlogPage {
    blogs: Vec<Blog>,
}

impl Page for BlogPage {
    const TITLE: &'static str = "Blog";
    const BASE_PATH: &'static str = "/blog";

    fn app(self) -> Router {
        let mut inner = Router::new().route("/", get(Self::index));

        for blog in self.blogs {
            inner = inner.route(&format!("/{}", blog.title), get(|| Self::blog(blog)));
        }

        Router::new().nest(Self::BASE_PATH, inner)
    }
}

impl BlogPage {
    pub async fn build(blogs_dir: PathBuf) -> Result<Self> {
        let mut blogs = Vec::new();

        let mut entries = tokio::fs::read_dir(&blogs_dir).await?;
        while let Some(file) = entries.next_entry().await? {
            if !file.file_type().await?.is_file() {
                warn!(?file, "Unrecognized file type");
                continue;
            }

            let blog = Blog::read(blogs_dir.join(file.file_name()))
                .await
                .context("Invalid blog file")?;

            info!(title = blog.title, date = %blog.date, "Found blog");

            blogs.push(blog);
        }

        info!(count = blogs.len(), "Done loading blogs");

        Ok(Self { blogs })
    }

    async fn index() -> Markup {
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

    async fn blog(blog: Blog) -> Markup {
        let head = HeadBuilder::new(&blog.title.replace('_', " ")).build();
        let nav = NavBuilder::new(&NAV_PAGES).build();

        let content = blog.render();

        html! {
            (DOCTYPE)
            html class="h-full" {
                head { (head) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (nav)
                    div class="m-20 text-slate-200" {
                        (content)
                    }
                }
            }
        }
    }
}

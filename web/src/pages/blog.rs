use std::path::PathBuf;

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use chrono::NaiveDate;
use maud::{html, Markup, DOCTYPE};
use tracing::warn;

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{Page, NAV_PAGES},
};

#[derive(Debug)]
struct Blog {
    title: String,
    date: NaiveDate,
    file: PathBuf,
}

impl Blog {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let file_name = path.file_name()?.to_string_lossy();
        let (date, title) = file_name.trim_end_matches(".md").split_once(':')?;

        Some(Self {
            title: title.to_string(),
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?,
            file: path,
        })
    }
}

pub struct BlogPage {
    blogs: Vec<Blog>,
}

impl Page for BlogPage {
    const TITLE: &'static str = "Blog";
    const BASE_PATH: &'static str = "/blog";

    fn app(self) -> Router {
        let mut app = Router::new().route(Self::BASE_PATH, get(Self::index));

        // TODO: this is temporary
        for blog in self.blogs {
            app = app.route(
                &format!("{}/{}", Self::BASE_PATH, blog.title),
                get(|| async { blog.title }),
            );
        }

        app
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

            let blog_file = blogs_dir.join(file.file_name());
            let blog = Blog::from_path(blog_file).context("Invalid blog file")?;
            blogs.push(blog);
        }

        dbg!(&blogs);

        Ok(Self { blogs })
    }

    async fn index() -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        let nav = NavBuilder::new(&NAV_PAGES).active(Self::BASE_PATH).build();

        html! {
            (DOCTYPE)
            html {
                head { (head) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (nav)
                    (Self::content())
                }
            }
        }
    }

    fn content() -> Markup {
        html! {
            h1 { "Hello, blog!" }
        }
    }
}

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
            .route(Self::PATH, static_page!(self.index()))
            .route_iter(
                self.blogs
                    .into_iter()
                    .map(|blog| (blog.path(), static_page!(blog.render()))),
            )
    }

    fn content(&self) -> Markup {
        html! {
            div class="grid grid-cols-1 gap-16 m-20 lg:grid-cols-2 2xl:grid-cols-3" {
                @for entry in self.blogs.iter() {
                    (blog_entry(&entry))
                }
            }
        }
    }
}

fn blog_entry(blog: &Blog) -> Markup {
    html! {
        a href=(blog.path()) class="bg-gradient-to-br to-teal-800 rounded from-zinc-800" {
            div class="grid grid-rows-2 p-4 rounded transition-transform hover:scale-105 aspect-video" {
                span class="flex justify-center items-end m-2 text-4xl font-bold text-slate-200" {
                    (blog.title())
                }
                span class="flex justify-center items-start m-2 text-2xl italic text-slate-400" {
                    (blog.date().format("%-d %B %Y"))
                }
            }
        }
    }
}

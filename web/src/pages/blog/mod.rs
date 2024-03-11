mod entry;
mod loader;
mod renderer;

use std::path::Path;

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
            inner = inner.route(&blog.path(), get(static_page!(blog.render())));
        }

        Router::new().nest(Self::BASE_PATH, inner)
    }
}

impl BlogApp {
    pub async fn build(blogs_dir: impl AsRef<Path>) -> Result<Self> {
        let blogs = BlogLoader::read_dir(blogs_dir).await?;
        Ok(Self { blogs })
    }

    fn index(&self) -> Markup {
        let head = HeadBuilder::new(Self::TITLE).build();
        let nav = NavBuilder::new(&NAV_PAGES).active(Self::BASE_PATH).build();

        html! {
            (DOCTYPE)
            html class="min-h-full" {
                head { (head) }
                body class="w-full min-h-full bg-neutral-800" {
                    (nav)
                    div class="grid grid-cols-1 gap-16 m-20 lg:grid-cols-2 2xl:grid-cols-4" {
                        @for entry in self.blogs.iter() {
                            (blog_entry(&entry))
                        }
                    }
                }
            }
        }
    }
}

fn blog_entry(blog: &Blog) -> Markup {
    html! {
        a href=(format!("/blog{}", blog.path())) class="bg-gradient-to-br to-teal-800 rounded from-zinc-800" {
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

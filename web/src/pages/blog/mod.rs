mod entry;
mod loader;
mod renderer;

use std::path::Path;

use anyhow::Result;
use axum::Router;
use maud::{html, Markup, DOCTYPE};

use crate::{
    pages::{blog::entry::Blog, blog::loader::BlogLoader, Module},
    static_page,
};

pub struct BlogApp {
    blogs: Vec<Blog>,
}

impl Module for BlogApp {
    const PATH: &'static str = "/blog";
    const TITLE: &'static str = "Blog";

    fn app(self) -> Router {
        let mut app = Router::new().route(Self::PATH, static_page!(self.index()));

        for blog in self.blogs {
            app = app.route(&blog.path(), static_page!(blog.render()));
        }

        app
    }
}

impl BlogApp {
    pub async fn build(blogs_dir: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            blogs: BlogLoader::read_dir(blogs_dir).await?,
        })
    }

    fn index(&self) -> Markup {
        html! {
            (DOCTYPE)
            html class="min-h-full" {
                head { (self.head()) }
                body class="w-full min-h-full bg-neutral-800" {
                    (self.nav())
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

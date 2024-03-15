mod blog;
mod home;
mod projects;

pub use blog::*;
pub use home::*;
pub use projects::*;

use std::sync::atomic::{AtomicUsize, Ordering};

use axum::Router;
use maud::Markup;

use crate::components::{HeadBuilder, NavBuilder, NavEntry};

pub trait Module {
    const TITLE: &'static str;
    const PATH: &'static str;

    fn app(self) -> Router;

    fn nav(&self) -> Markup {
        NavBuilder::new(&NAV_PAGES).active(Self::PATH).build()
    }

    fn head(&self) -> Markup {
        HeadBuilder::new(Self::TITLE).build()
    }
}

const NAV_PAGES: [NavEntry; 3] = [
    (HomeApp::TITLE, HomeApp::PATH),
    (BlogApp::TITLE, BlogApp::PATH),
    (ProjectsApp::TITLE, ProjectsApp::PATH),
];

static CACHED_COUNT: AtomicUsize = AtomicUsize::new(0);
static CACHED_TOTAL: AtomicUsize = AtomicUsize::new(0);

#[macro_export]
macro_rules! static_page {
    ($x:expr) => {{
        let content = $x;

        let len = content.0.len();
        $crate::pages::CACHED_TOTAL.fetch_add(len, std::sync::atomic::Ordering::Relaxed);
        $crate::pages::CACHED_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let size = format!("{}kB", len / 1024);
        tracing::info!(%size, "Caching page in memory");

        axum::routing::get(move || async { content })
    }};
}

pub fn cached_memory() -> (usize, usize) {
    (
        CACHED_COUNT.load(Ordering::Relaxed),
        CACHED_TOTAL.load(Ordering::Relaxed),
    )
}

#[macro_export]
macro_rules! info_cached_memory {
    () => {{
        let (count, total) = $crate::pages::cached_memory();
        let size = format!("{}kB", total / 1024);
        tracing::info!(pages = count, %size, "Memory cache");
    }}
}

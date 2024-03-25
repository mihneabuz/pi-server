use axum::Router;
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::NAV_PAGES,
    static_page,
};

pub trait Module<S = ()> {
    const TITLE: &'static str;
    const PATH: &'static str;

    fn content(&self) -> Markup;

    fn app(self) -> Router<S>
    where
        Self: Sized,
        S: Clone + Send + Sync + 'static,
    {
        Router::new().route(Self::PATH, static_page!(self.index()))
    }

    fn nav(&self) -> Markup {
        NavBuilder::new(&NAV_PAGES).active(Self::PATH).build()
    }

    fn head(&self) -> Markup {
        HeadBuilder::new(Self::TITLE).build()
    }

    fn index(&self) -> Markup {
        html! {
            (DOCTYPE)
            html class="flex flex-col min-h-full" {
                head { (self.head()) }
                body class="flex flex-col min-h-full grow bg-neutral-800" {
                    (self.nav())
                    (self.content())
                }
            }
        }
    }
}

pub trait DynamicModule<S = ()> {
    fn path(&self) -> String;
    fn title(&self) -> String;
    fn content(&self) -> Markup;

    fn app(self) -> Router<S>
    where
        Self: Sized,
        S: Clone + Send + Sync + 'static,
    {
        Router::new().route(&self.path(), static_page!(self.index()))
    }

    fn nav(&self) -> Markup {
        NavBuilder::new(&NAV_PAGES).build()
    }

    fn head(&self) -> Markup {
        HeadBuilder::new(&self.title()).build()
    }

    fn index(&self) -> Markup {
        html! {
            (DOCTYPE)
            html class="flex flex-col min-h-full" {
                head { (self.head()) }
                body class="flex flex-col min-h-full grow bg-neutral-800" {
                    (self.nav())
                    (self.content())
                }
            }
        }
    }
}

impl<T> DynamicModule for T
where
    T: Module,
{
    fn path(&self) -> String {
        String::from(Self::PATH)
    }

    fn title(&self) -> String {
        String::from(Self::TITLE)
    }

    fn content(&self) -> Markup {
        Module::content(self)
    }

    fn app(self) -> Router
    where
        Self: Sized,
    {
        Module::app(self)
    }

    fn nav(&self) -> Markup {
        Module::nav(self)
    }

    fn head(&self) -> Markup {
        Module::head(self)
    }

    fn index(&self) -> Markup {
        Module::index(self)
    }
}

use chrono::NaiveDate;
use markdown::mdast::Node;
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{blog::renderer::render_markdown, NAV_PAGES},
};

#[derive(Clone, Debug)]
pub struct Blog {
    pub title: String,
    pub date: NaiveDate,
    pub ast: Node,
}

impl Blog {
    pub fn render(self) -> Markup {
        let head = HeadBuilder::new(&self.title.replace('_', " ")).build();
        let nav = NavBuilder::new(&NAV_PAGES).build();

        html! {
            (DOCTYPE)
            html class="h-full" {
                head { (head) }
                body class="flex flex-col h-full bg-neutral-800" {
                    (nav)
                    div class="m-20 text-slate-200" {
                        (render_markdown(self.ast))
                    }
                }
            }
        }
    }
}

use chrono::NaiveDate;
use markdown::mdast::Node;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tracing::warn;

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::NAV_PAGES,
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
                        (render_node(self.ast))
                    }
                }
            }
        }
    }
}

pub fn render_node(node: Node) -> Markup {
    match node {
        Node::Root(root) => html! {
            div {
                @for child in root.children {
                    (render_node(child))
                }
            }
        },

        Node::Heading(heading) => {
            let children = html! {
                @for child in heading.children {
                    (render_node(child))
                }
            };

            html! {
                @match heading.depth {
                    1 => h1 class="py-4 px-2 text-4xl font-bold" { (children) },
                    2 => h2 class="py-4 px-2 text-3xl font-bold" { (children) },
                    3 => h3 class="py-4 px-2 text-2xl font-bold" { (children) },
                    4 => h4 class="py-4 px-2 text-xl font-bold" { (children) },
                    5 => h5 class="py-4 px-2 text-lg font-bold" { (children) },
                    6 => h6 class="py-4 px-2 font-bold" { (children) },
                    _ => p {},
                }
            }
        }

        Node::Paragraph(paragraph) => html! {
            p {
                @for child in paragraph.children {
                    (render_node(child))
                }
            }
        },

        Node::Text(text) => html! {
            span { (text.value) }
        },

        Node::Strong(strong) => html! {
            strong {
                @for child in strong.children {
                    (render_node(child))
                }
            }
        },

        Node::Emphasis(emphasis) => html! {
            span {
                @for child in emphasis.children {
                    (render_node(child))
                }
            }
        },

        Node::Code(code) => html! {
            code {
                (code.value)
            }
        },

        Node::InlineCode(code) => html! {
            code {
                (code.value)
            }
        },

        Node::Math(_) => todo!(),

        Node::InlineMath(_) => todo!(),

        Node::List(list) => {
            let children = html! {
                @for child in list.children {
                    (render_node(child))
                }
            };

            html! {
                @if list.ordered {
                    ol class="list-decimal" { (children) }
                } else {
                    ul class="list-disc" { (children) }
                }
            }
        }

        Node::ListItem(list_item) => html! {
            li class="ml-8" {
                @for child in list_item.children {
                    (render_node(child))
                }
            }
        },

        Node::Table(_) => todo!(),

        Node::TableRow(_) => todo!(),

        Node::TableCell(_) => todo!(),

        Node::Toml(_) => todo!(),

        Node::Yaml(_) => todo!(),

        Node::Html(html) => html! {
            (PreEscaped(html.value))
        },

        Node::Image(image) => html! {
            image class="my-2 mx-auto w-full aspect-auto" src=(image.url);
        },

        Node::Link(link) => html! {
            a href=(link.url) class="text-teal-500 transition-all hover:text-teal-200" {
                @for child in link.children {
                    (render_node(child))
                }
            }
        },

        Node::BlockQuote(block) => html! {
            div {
                @for child in block.children {
                    (render_node(child))
                }
            }
        },

        other => {
            warn!(node = ?other, "Unimplemented MDX");
            html! {}
        }
    }
}

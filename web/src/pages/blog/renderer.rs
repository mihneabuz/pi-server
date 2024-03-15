use std::collections::HashSet;

use markdown::mdast::Node;
use maud::{html, Markup, PreEscaped};
use tracing::warn;

pub fn render_markdown(node: Node) -> (Markup, Vec<String>) {
    let mut langs = HashSet::new();
    let markup = render(node, &mut langs);
    (markup, langs.into_iter().collect())
}

fn render(node: Node, langs: &mut HashSet<String>) -> Markup {
    macro_rules! render_children {
        ($x:expr) => {
            html! {
                @for child in $x.children { (render(child, langs))
                }
            }
        };
    }

    match node {
        Node::Root(root) => html! {
            div class="text-2xl text-slate-200" {
                (render_children!(root))
            }
        },

        Node::Heading(heading) => {
            let children = render_children!(heading);

            html! {
                @match heading.depth {
                    1 => h1 class="pt-4 pb-4 text-5xl font-bold" { (children) },
                    2 => h2 class="pt-4 pb-2 text-4xl font-bold" { (children) },
                    3 => h3 class="pt-4 pb-2 text-3xl font-bold" { (children) },
                    4 => h4 class="pt-4 pb-2 text-2xl font-bold" { (children) },
                    5 => h5 class="pt-4 pb-2 text-2xl font-bold" { (children) },
                    6 => h6 class="pt-4 pb-2 text-2xl font-bold" { (children) },
                    _ => p {},
                }
            }
        }

        Node::Paragraph(paragraph) => html! {
            p class="my-4" {
                (render_children!(paragraph))
            }
        },

        Node::Text(text) => html! {
            span { (text.value) }
        },

        Node::Break(_) => html! {
            br;
        },

        Node::Strong(strong) => html! {
            strong {
                (render_children!(strong))
            }
        },

        Node::Emphasis(emphasis) => html! {
            span class="italic" {
                (render_children!(emphasis))
            }
        },

        Node::Delete(delete) => html! {
            span class="line-through" {
                (render_children!(delete))
            }
        },

        Node::Code(code) => {
            let lang = code
                .lang
                .map(|lang| {
                    langs.insert(lang.clone());
                    format!("language-{}", lang)
                })
                .unwrap_or_default();

            html! {
                pre class="my-2 text-xl" {
                    code class=(format!("rounded-lg {lang}")) {
                        (code.value)
                    }
                }
            }
        }

        Node::InlineCode(code) => html! {
            code class="px-1 text-xl rounded bg-neutral-700" {
                (code.value)
            }
        },

        Node::Math(math) => html! {
            math display="block" {
                (math.value)
            }
        },

        Node::InlineMath(math) => html! {
            math display="inline" {
                (math.value)
            }
        },

        Node::List(list) => {
            let children = render_children!(list);

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
                (render_children!(list_item))
            }
        },

        Node::Html(html) => html! {
            (PreEscaped(html.value))
        },

        Node::Image(image) => html! {
            image class="my-2 mx-auto w-full aspect-auto" src=(image.url) alt=(image.alt);
        },

        Node::Link(link) => html! {
            a href=(link.url) target="_blank" class="text-teal-500 transition-all hover:text-teal-200 has-tooltip" {
                (render_children!(link))
            }
        },

        Node::BlockQuote(block) => html! {
            div class="flex" {
                div class="mr-2 ml-1 w-1 bg-neutral-300" {}
                div class="grow text-neutral-400" {
                    (render_children!(block))
                }
            }
        },

        Node::Table(table) => html! {
            table class="my-2 table-auto bg-zinc-800" {
                (render_children!(table))
            }
        },

        Node::TableRow(row) => html! {
            tr {
                (render_children!(row))
            }
        },

        Node::TableCell(cell) => html! {
            td class="py-2 px-4 border-2 border-neutral-600" {
                (render_children!(cell))
            }
        },

        mut other => {
            if let Some(children) = other.children_mut() {
                children.clear();
            }

            warn!(node = ?other, "Unimplemented node");
            html! {}
        }
    }
}

use markdown::mdast::Node;
use maud::{html, Markup, PreEscaped};
use tracing::warn;

macro_rules! render_children {
    ($x:expr) => {
        html! {
            @for child in $x {
                (render_markdown(child))
            }
        }
    };
}

pub fn render_markdown(node: Node) -> Markup {
    match node {
        Node::Root(root) => html! {
            div {
                (render_children!(root.children))
            }
        },

        Node::Heading(heading) => {
            let children = render_children!(heading.children);

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
                (render_children!(paragraph.children))
            }
        },

        Node::Text(text) => html! {
            span { (text.value) }
        },

        Node::Strong(strong) => html! {
            strong {
                (render_children!(strong.children))
            }
        },

        Node::Emphasis(emphasis) => html! {
            span {
                (render_children!(emphasis.children))
            }
        },

        Node::Code(code) => html! {
            code {
                (code.value)
            }
        },

        Node::InlineCode(code) => html! {
            code class="px-1 rounded bg-neutral-700" {
                (code.value)
            }
        },

        Node::Math(_) => todo!(),

        Node::InlineMath(_) => todo!(),

        Node::List(list) => {
            let children = render_children!(list.children);

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
                (render_children!(list_item.children))
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
                (render_children!(link.children))
            }
        },

        Node::BlockQuote(block) => html! {
            div class="flex" {
                div class="mr-2 ml-1 w-1 bg-neutral-400" {}
                div class="grow text-neutral-400" {
                    (render_children!(block.children))
                }
            }
        },

        other => {
            warn!(node = ?other, "Unimplemented MDX");
            html! {}
        }
    }
}

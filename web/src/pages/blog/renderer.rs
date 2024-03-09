use markdown::mdast::Node;
use maud::{html, Markup, PreEscaped};
use tracing::warn;

macro_rules! render_children {
    ($x:expr) => {
        html! {
            @for child in $x { (render_markdown(child))
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
            span class="italic" {
                (render_children!(emphasis.children))
            }
        },

        Node::Delete(delete) => html! {
            span class="line-through" {
                (render_children!(delete.children))
            }
        },

        Node::Code(code) => html! {
            pre class="my-2" {
                @let lang = code.lang.map(|lang| format!("language-{}", lang)).unwrap_or_default();
                code class=(format!("rounded-lg {lang}")) {
                    (code.value)
                }
            }
        },

        Node::InlineCode(code) => html! {
            code class="px-1 rounded bg-neutral-700" {
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

        Node::Html(html) => html! {
            (PreEscaped(html.value))
        },

        Node::Image(image) => html! {
            image class="my-2 mx-auto w-full aspect-auto" src=(image.url) alt=(image.alt);
        },

        Node::Link(link) => html! {
            a href=(link.url) class="text-teal-500 transition-all hover:text-teal-200 has-tooltip" {
                (render_children!(link.children))
            }
        },

        Node::BlockQuote(block) => html! {
            div class="flex" {
                div class="mr-2 ml-1 w-1 bg-neutral-300" {}
                div class="grow text-neutral-300" {
                    (render_children!(block.children))
                }
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

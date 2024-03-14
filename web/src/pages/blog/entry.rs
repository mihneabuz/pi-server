use chrono::NaiveDate;
use markdown::mdast::Node;
use maud::{html, Markup, DOCTYPE};

use crate::{
    components::{HeadBuilder, NavBuilder},
    pages::{blog::renderer::render_markdown, NAV_PAGES},
};

#[derive(Clone, Debug)]
pub struct Blog {
    title: String,
    date: NaiveDate,
    ast: Node,
}

impl Blog {
    pub fn new(title: String, date: NaiveDate, markdown: Node) -> Self {
        Self {
            title,
            date,
            ast: markdown,
        }
    }

    pub fn render(self) -> Markup {
        let head = HeadBuilder::new(&self.title)
            .stylesheet("/public/highlight/theme.css")
            .build();

        let nav = NavBuilder::new(&NAV_PAGES).build();

        let (rendered, langs) = render_markdown(self.ast);

        html! {
            (DOCTYPE)
            html class="min-h-full" {
                head { (head) }
                body class="flex flex-col min-h-full bg-neutral-800" {
                    (nav)
                    div class="m-20 xl:mx-auto xl:w-[80%] grow" {
                        (rendered)
                    }
                    (highlight_script(&langs))
                }
            }
        }
    }

    pub fn title(&self) -> String {
        self.title.replace('_', " ")
    }

    pub fn path(&self) -> String {
        format!("/{}", self.title)
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
}

fn highlight_script(langs: &[String]) -> Markup {
    if langs.is_empty() {
        return html! {};
    }

    html! {
        script type="module" async {
            "import hljs from '/public/highlight/core.min.js';"
            @for lang in langs {
                (format!("import {lang} from '/public/highlight/languages/{lang}.min.js';"))
                (format!("hljs.registerLanguage('{lang}', {lang});"))
            }
            "hljs.highlightAll();"
        }
    }
}

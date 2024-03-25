use chrono::NaiveDate;
use markdown::mdast::Node;
use maud::{html, Markup};

use crate::{
    components::{Card, HeadBuilder},
    pages::{blog::renderer::render_markdown, BlogApp, DynamicModule, Module},
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

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn card(&self) -> Markup {
        Card::new(self.title())
            .description(self.date().format("%-d %B %Y"))
            .link_to(self.path())
            .build()
    }
}

impl DynamicModule for Blog {
    fn path(&self) -> String {
        format!("{}/{}", BlogApp::PATH, self.title)
    }

    fn title(&self) -> String {
        self.title.replace('_', " ")
    }

    fn head(&self) -> Markup {
        HeadBuilder::new(&self.title)
            .stylesheet("/public/highlight/theme.css")
            .build()
    }

    fn content(&self) -> Markup {
        let (rendered, langs) = render_markdown(&self.ast);

        html! {
            div class="m-20 xl:mx-auto xl:w-[80%] grow" {
                (rendered)
            }
            (highlight_script(&langs))
        }
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

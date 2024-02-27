use maud::{html, Markup};

const DEFAULT_FAVICON: &str = "/public/computer_favicon.png";

pub struct HeadBuilder<'a> {
    title: &'a str,
    favicon: Option<&'a str>,
}

impl<'a> HeadBuilder<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            favicon: None,
        }
    }

    pub fn icon(mut self, favicon: &'a str) -> Self {
        self.favicon.replace(favicon);
        self
    }

    pub fn build(self) -> Markup {
        let HeadBuilder { title, favicon } = self;

        html! {
            meta charset="utf-8";
            title { (title) }
            link rel="stylesheet" type="text/css" href="/public/styles.css";
            link rel="icon" type="image/x-icon" href=(favicon.unwrap_or(DEFAULT_FAVICON));
        }
    }
}

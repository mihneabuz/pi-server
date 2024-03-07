use maud::{html, Markup};

const DEFAULT_FAVICON: &str = "/public/computer_favicon.png";

pub struct HeadBuilder<'a> {
    title: &'a str,
    favicon: Option<&'a str>,
    extra: Vec<Markup>,
}

impl<'a> HeadBuilder<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            favicon: None,
            extra: Vec::new(),
        }
    }

    pub fn icon(mut self, favicon: &'a str) -> Self {
        self.favicon.replace(favicon);
        self
    }

    pub fn script(mut self, src: &'a str) -> Self {
        self.extra.push(html! {
            script src=(src);
        });
        self
    }

    pub fn stylesheet(mut self, href: &'a str) -> Self {
        self.extra.push(html! {
            link href=(href) rel="stylesheet";
        });
        self
    }

    pub fn build(self) -> Markup {
        let HeadBuilder {
            title,
            favicon,
            extra,
        } = self;

        html! {
            meta charset="utf-8";
            title { (title) }
            link rel="preconnect" href="https://rsms.me/";
            link rel="stylesheet" href="https://rsms.me/inter/inter.css";
            link rel="stylesheet" type="text/css" href="/public/styles.css";
            link rel="icon" type="image/x-icon" href=(favicon.unwrap_or(DEFAULT_FAVICON));
            @for item in extra {
                (item)
            }
        }
    }
}

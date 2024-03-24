use maud::{html, Markup};

pub struct Card {
    title: String,
    link_to: Option<String>,
    description: Option<String>,
}

impl Card {
    pub fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            link_to: None,
            description: None,
        }
    }

    pub fn link_to(mut self, href: impl ToString) -> Self {
        self.link_to = Some(href.to_string());
        self
    }

    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn build(self) -> Markup {
        html! {
            a href=(self.link_to.unwrap_or("".to_string())) class="bg-gradient-to-br to-teal-800 rounded from-zinc-800" {
                div class="grid grid-rows-2 p-4 rounded transition-transform hover:scale-105 aspect-video" {
                    span class="flex justify-center items-end m-2 text-4xl font-bold text-slate-200" {
                        (self.title)
                    }
                    span class="flex justify-center items-start m-2 text-2xl italic text-slate-400" {
                        (self.description.unwrap_or("".to_string()))
                    }
                }
            }
        }
    }
}

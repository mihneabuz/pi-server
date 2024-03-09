use maud::{html, Markup};

pub type NavEntry = (&'static str, &'static str);

pub struct NavBuilder {
    entries: &'static [NavEntry],
    active: Option<&'static str>,
}

impl NavBuilder {
    pub fn new(entries: &'static [NavEntry]) -> Self {
        Self {
            entries,
            active: None,
        }
    }

    pub fn active(mut self, path: &'static str) -> Self {
        self.active = Some(path);
        self
    }

    pub fn build(self) -> Markup {
        html! {
            div class="flex justify-between py-4 px-8" {
                h1 class="my-auto font-bold text-teal-500" { "Icon" }
                nav class="flex justify-between w-72" {
                    @for (name, path) in self.entries {
                        (Self::link(name, path, self.active.is_some_and(|a| *path == a)))
                    }
                }
            }
        }
    }

    fn link(name: &str, path: &str, active: bool) -> Markup {
        let link = html! {
            a href=(path)
                class="px-1 text-lg font-bold text-teal-500 transition-all hover:text-teal-200"
                { (name) }
        };

        match active {
            true => html! {
                div class="border-b-2 border-teal-500" { (link) }
            },
            false => link,
        }
    }
}

use maud::{html, Markup};

pub type NavEntry = (&'static str, &'static str);

pub struct Nav {
    entries: &'static [NavEntry],
    active: Option<&'static str>,
}

impl Nav {
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
            div class="flex justify-between py-4 px-8 bg-sky-950" {
                h1 class="text-teal-500 font-bold my-auto" { "Icon" }
                nav class="w-72 flex justify-between" {
                    @for (name, path) in self.entries {
                        (Self::link(name, path, self.active.is_some_and(|a| *path == a)))
                    }
                }
            }
        }
    }

    fn link(name: &str, path: &str, active: bool) -> Markup {
        if active {
            html! {
                div class="border-b-4 border-teal-500" {
                    a
                        href=(path)
                        class="px-1 text-teal-500 text-lg font-bold"
                        aria-current="page"
                        { (name) }
                }
            }
        } else {
            html! {
                a href=(path) class="px-1 text-teal-500 text-lg font-bold" { (name) }
            }
        }
    }
}

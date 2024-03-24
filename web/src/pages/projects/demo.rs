use maud::{html, Markup, PreEscaped};

use crate::pages::Module;

pub struct DemoGame;

impl DemoGame {
    const WIDTH: usize = 60;
    const HEIGHT: usize = 60;
}

impl Module for DemoGame {
    const PATH: &'static str = "/projects/demo";
    const TITLE: &'static str = "Demo";

    fn content(&self) -> Markup {
        let (width, height) = (Self::WIDTH, Self::HEIGHT);

        let script = format!(
            r#"
            import init, {{ Game }} from '/public/wasm/game_of_life.js';
            init().then(() => {{
                new Game('#game', {width}, {height})
                    .timeout(100)
                    .attach_start('#start')
                    .attach_stop('#stop');
            }});
            "#
        );

        let styles = format!(
            r#"
            #game {{
                display: grid;
                grid-template-columns: repeat({width}, 1fr);
            }}

            #game>div {{
                aspect-ratio: 1;
            }}

            #game>.alive {{
                background: #14b8a6;
            }}
            "#
        );

        html! {
            h1 class="m-20 text-5xl text-slate-200" {
                "Conway's Game of Life"
            }
            div class="flex justify-around items-center mx-20 mt-10 mb-5 w-72" {
                button #start class="text-2xl font-bold text-teal-500" { "Start" }
                button #stop  class="text-2xl font-bold text-red-500" { "Stop" }
            }
            div #game class="grid my-5 mx-20 rounded border-4 border-teal-700" {}
            style { (PreEscaped(styles)) }
            script type="module" { (PreEscaped(script)) }
        }
    }
}

use std::{cell::RefCell, ops::Deref, sync::Arc};

use game_of_life::GameOfLife;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let document = get_document();
    let body = document.body().expect("document should have a body");

    let styles = document.create_element("style")?;
    styles.set_text_content(Some(
        r#"
        #root {
            width: 100%;
            height: 100%;
            display: grid;
            grid-template-columns: repeat(10, 1fr);
        }

        #root > div {
            aspect-ratio: 1;
        }

        #root > .alive {
            background: black;
        }
    "#,
    ));
    body.append_child(&styles)?;

    let button = document.create_element("button")?;
    button.set_id("step");
    button.set_text_content(Some("STEP"));
    body.append_child(&button)?;

    let root = document.create_element("div")?;
    root.set_id("root");
    body.append_child(&root)?;

    let game = Game::new("#root", 10, 10).attach_step("#step");
    game.set(1, 1, true);
    game.set(4, 4, true);
    game.set(8, 8, true);

    Ok(())
}

fn get_document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    document
}

#[wasm_bindgen]
pub struct Game {
    inner: Arc<RefCell<GameInner>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str, width: i32, height: i32) -> Self {
        Self {
            inner: Arc::new(RefCell::new(GameInner::new(selector, width, height))),
        }
    }

    pub fn set(&self, x: i32, y: i32, alive: bool) {
        self.inner.deref().borrow_mut().set(x, y, alive);
    }

    pub fn attach_step(self, selector: &str) -> Self {
        let elem = get_document()
            .query_selector(selector)
            .expect("bad selector")
            .expect("could not find element");

        let clone = Arc::clone(&self.inner);
        let on_click = Closure::<dyn FnMut()>::new(move || {
            clone.deref().borrow_mut().step();
        });

        elem.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));

        on_click.forget();

        self
    }
}

pub struct GameInner {
    game: GameOfLife,
    root: Element,
}

impl GameInner {
    pub fn new(selector: &str, width: i32, height: i32) -> Self {
        let document = get_document();

        let root = document
            .query_selector(selector)
            .expect("bad selector")
            .expect("could not find element");

        for _ in 0..width * height {
            let cell = document
                .create_element("div")
                .expect("could not create cell");
            cell.set_class_name("dead");

            root.append_child(&cell.clone())
                .expect("could not append cell to root");
        }

        Self {
            game: GameOfLife::new(width as usize, height as usize),
            root,
        }
    }

    fn set_node(root: &Element, index: u32, alive: bool) {
        let changed = root.children().get_with_index(index).unwrap();
        changed.set_class_name(if alive { "alive" } else { "dead" });
    }

    pub fn set(&mut self, x: i32, y: i32, alive: bool) {
        if let Some(previous) = self.game.set(x as usize, y as usize, alive) {
            if previous != alive {
                Self::set_node(&self.root, (x * y) as u32, alive);
            }
        }
    }

    pub fn step(&mut self) {
        for update in self.game.step_iter() {
            Self::set_node(&self.root, (update.x * update.y) as u32, update.alive);
        }
    }
}

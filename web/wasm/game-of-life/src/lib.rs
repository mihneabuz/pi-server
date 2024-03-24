mod game;

use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use game::GameOfLife;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

fn get_window() -> Window {
    web_sys::window().expect("no global window exists")
}

fn get_document() -> Document {
    get_window()
        .document()
        .expect("should have a document on window")
}

fn query_selector(selector: &str) -> Element {
    get_document()
        .query_selector(selector)
        .expect("good selector")
        .expect("should find an element")
}

#[wasm_bindgen]
pub struct Game {
    root: Element,
    inner: Rc<RefCell<GameOfLife>>,
    interval_handle: Rc<RefCell<Option<i32>>>,
    interval_timeout: i32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(selector: &str, width: i32, height: i32) -> Self {
        let document = get_document();

        let root = query_selector(selector);

        let game = GameOfLife::new(width as usize, height as usize);
        let inner = Rc::new(RefCell::new(game));

        for y in 0..height {
            for x in 0..width {
                let cell = document
                    .create_element("div")
                    .expect("should be able to create cell");
                cell.set_class_name("dead");

                root.append_child(&cell.clone())
                    .expect("should be able to append cell to root");

                let cell_clone = cell.clone();
                let cell_ref = cell_clone.dyn_ref::<HtmlElement>().unwrap();

                let root_clone = root.clone();
                let game_clone = Rc::clone(&inner);
                let toggle = Closure::<dyn FnMut()>::new(move || {
                    let next_alive = match cell.class_name().as_str() {
                        "alive" => false,
                        "dead" => true,
                        _ => panic!("wrong class name on cell"),
                    };

                    Self::set_inner(&root_clone, game_clone.borrow_mut(), x, y, next_alive);
                });

                cell_ref.set_onclick(Some(toggle.as_ref().unchecked_ref()));

                toggle.forget();
            }
        }

        Self {
            inner,
            root,
            interval_handle: Rc::new(RefCell::new(None)),
            interval_timeout: 200,
        }
    }

    fn set_inner(root: &Element, mut game: RefMut<'_, GameOfLife>, x: i32, y: i32, alive: bool) {
        if let Some(previous) = game.set(x as usize, y as usize, alive) {
            if previous != alive {
                let index = y as usize * game.width() + x as usize;
                root.children()
                    .get_with_index(index as u32)
                    .unwrap()
                    .set_class_name(if alive { "alive" } else { "dead" });
            }
        }
    }

    fn step_inner(root: &Element, mut game: RefMut<'_, GameOfLife>) {
        let width = game.width();
        for update in game.step_iter() {
            let index = update.y * width + update.x;
            root.children()
                .get_with_index(index as u32)
                .unwrap()
                .set_class_name(if update.alive { "alive" } else { "dead" });
        }
    }

    pub fn set(&self, x: i32, y: i32, alive: bool) {
        Self::set_inner(&self.root, self.inner.borrow_mut(), x, y, alive);
    }

    pub fn step(&self) {
        Self::step_inner(&self.root, self.inner.borrow_mut());
    }

    pub fn timeout(mut self, timeout: i32) -> Self {
        self.interval_timeout = timeout;
        self
    }

    pub fn attach_step(self, selector: &str) -> Self {
        let elem = query_selector(selector);

        let root = self.root.clone();
        let game_clone = Rc::clone(&self.inner);
        let on_click = Closure::<dyn FnMut()>::new(move || {
            Self::step_inner(&root, game_clone.borrow_mut());
        });

        elem.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));

        on_click.forget();

        self
    }

    pub fn attach_start(self, selector: &str) -> Self {
        let elem = query_selector(selector);

        let root = self.root.clone();
        let game_clone = Rc::clone(&self.inner);
        let step = Closure::<dyn FnMut()>::new(move || {
            Self::step_inner(&root, game_clone.borrow_mut());
        });

        let handle = Rc::clone(&self.interval_handle);
        let timeout = self.interval_timeout;
        let on_click = Closure::<dyn FnMut()>::new(move || {
            let mut handle = handle.borrow_mut();
            if handle.is_some() {
                return;
            }

            *handle = Some(
                get_window()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        step.as_ref().unchecked_ref(),
                        timeout,
                    )
                    .expect("should be able to set interval"),
            );
        });

        elem.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));

        on_click.forget();

        self
    }

    pub fn attach_stop(self, selector: &str) -> Self {
        let elem = query_selector(selector);

        let handle = Rc::clone(&self.interval_handle);
        let on_click = Closure::<dyn FnMut()>::new(move || {
            let mut handle = handle.borrow_mut();
            if let Some(handle) = handle.as_mut() {
                get_window().clear_interval_with_handle(*handle);
            }

            *handle = None;
        });

        elem.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));

        on_click.forget();

        self
    }

    pub fn attach_toggle(self, selector: &str) -> Self {
        let elem = query_selector(selector);

        let root = self.root.clone();
        let game_clone = Rc::clone(&self.inner);
        let step = Closure::<dyn FnMut()>::new(move || {
            Self::step_inner(&root, game_clone.borrow_mut());
        });

        let handle = Rc::clone(&self.interval_handle);
        let timeout = self.interval_timeout;
        let on_click = Closure::<dyn FnMut()>::new(move || {
            let mut handle = handle.borrow_mut();
            if handle.is_none() {
                *handle = Some(
                    get_window()
                        .set_interval_with_callback_and_timeout_and_arguments_0(
                            step.as_ref().unchecked_ref(),
                            timeout,
                        )
                        .expect("should be able to set interval"),
                );
            } else {
                get_window().clear_interval_with_handle(*handle.as_ref().unwrap());
                *handle = None;
            }
        });

        elem.dyn_ref::<HtmlElement>()
            .unwrap()
            .set_onclick(Some(on_click.as_ref().unchecked_ref()));

        on_click.forget();

        self
    }
}

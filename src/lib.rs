use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod component;
pub use component::{Component, State};

mod element;
pub use element::{Element, IntoElement};

mod html;
pub use html::Html;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document.get_element_by_id("root").unwrap();

    #[derive(Serialize, Deserialize)]
    struct Counter {
        initial: u32,
    }

    impl Component for Counter {
        fn render(&self) -> Element {
            let count = State::new(self.initial);
            let setter = count.clone();

            Html::new("button")
                .on_click(move |_event| setter.set(*setter + 1))
                .child(format!("You clicked this {} time(s)", *count))
        }
    }

    let counter = Counter { initial: 0 };
    render(counter, &root);
}

pub fn render<T: IntoElement>(element: T, container: &web_sys::Element) {
    react_sys::react_dom::render(&element.into_element().create(), container);
}

mod utils;

use js_sys::Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let root = document.get_element_by_id("root").unwrap();

    let element = Element::html("h1").child("Hello World!");

    render(&element, &root);
}

pub fn render(element: &Element, container: &web_sys::Element) {
    react_sys::react_dom::render(&element.create(), container);
}

pub struct Element {
    kind: JsValue,
    children: Array,
}

impl Element {
    pub fn html(kind: impl Into<JsValue>) -> Self {
        Self {
            kind: kind.into(),
            children: Array::default(),
        }
    }

    pub fn child(self, node: impl Into<JsValue>) -> Self {
        self.children.push(&node.into());
        self
    }

    pub fn create(&self) -> react_sys::Element {
        react_sys::create_element(&self.kind, &JsValue::null(), &self.children)
    }
}

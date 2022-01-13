mod utils;

use core::ops::Deref;
use js_sys::{Array, Function, Object, Reflect};
use serde::de::DeserializeOwned;
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

    struct Counter;

    impl Component for Counter {
        fn render(&self) -> Element {
            let count = State::new(0);

            let setter = count.clone();
            let f = Closure::wrap(Box::new(move || setter.set(*setter + 1)) as Box<dyn FnMut()>);

            Element::new("button")
                .property("onClick", &f.into_js_value())
                .child(count.value)
        }
    }

    let element = Element::new_component(Counter);

    render(element, &root);
}

pub fn render(element: Element, container: &web_sys::Element) {
    react_sys::react_dom::render(&element.create(), container);
}

pub struct Element {
    kind: JsValue,
    properties: JsValue,
    children: Array,
}

impl Element {
    pub fn new(kind: impl Into<JsValue>) -> Self {
        Self {
            kind: kind.into(),
            properties: Object::new().into(),
            children: Array::default(),
        }
    }

    pub fn new_component<C: Component + 'static>(component: C) -> Self {
        let f =
            Closure::wrap(Box::new(move || component.render().create())
                as Box<dyn FnMut() -> react_sys::Element>);
        Self {
            kind: f.into_js_value(),
            properties: JsValue::null(),
            children: Array::default(),
        }
    }

    pub fn property(self, key: &str, value: &JsValue) -> Self {
        Reflect::set(&self.properties, &key.into(), value).unwrap();

        self
    }

    pub fn child(self, node: impl Into<JsValue>) -> Self {
        self.children.push(&node.into());
        self
    }

    pub fn create(self) -> react_sys::Element {
        react_sys::create_element(&self.kind, &self.properties.into(), &self.children)
    }
}

pub trait Component {
    fn render(&self) -> Element;
}

#[derive(Clone)]
pub struct State<T> {
    value: T,
    set_value: Function,
}

impl<T> State<T>
where
    T: DeserializeOwned + Into<JsValue>,
{
    pub fn new(initial: T) -> Self {
        let array = react_sys::use_state(initial.into());
        let f: Function = array[1].clone().into();
        Self {
            value: array[0].clone().into_serde().unwrap(),
            set_value: f,
        }
    }

    // TODO function as argument to set state
    pub fn set(&self, value: T) {
        self.set_value
            .call1(&JsValue::null(), &value.into())
            .unwrap();
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

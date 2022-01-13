mod utils;

use core::ops::Deref;
use js_sys::{Array, Function, Object, Reflect};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
            let f = Closure::wrap(Box::new(move || setter.set(*setter + 1)) as Box<dyn FnMut()>);

            Html::new("button")
                .property("onClick", &f.into_js_value())
                .to_element()
                .child(count.value)
        }
    }

    let element = Counter { initial: 3 }.to_element();
    render(element, &root);
}

pub fn render(element: Element, container: &web_sys::Element) {
    react_sys::react_dom::render(&element.create(), container);
}

pub struct Html {
    tag: JsValue,
    properties: JsValue,
}

impl Html {
    pub fn new(tag: impl Into<JsValue>) -> Self {
        Self {
            tag: tag.into(),
            properties: Object::new().into(),
        }
    }

    pub fn property(self, key: &str, value: &JsValue) -> Self {
        Reflect::set(&self.properties, &key.into(), value).unwrap();
        self
    }

    pub fn to_element(self) -> Element {
        self.into()
    }
}

pub struct Element {
    kind: JsValue,
    properties: JsValue,
    children: Array,
}

impl From<Html> for Element {
    fn from(html: Html) -> Self {
        Self {
            kind: html.tag,
            properties: html.properties,
            children: Array::default(),
        }
    }
}

impl Element {
    pub fn new_component<C: Component + 'static>(component: &C) -> Self {
        let f = Closure::wrap(Box::new(move |props: JsValue| {
            let c: C = props.into_serde().unwrap();
            c.render().create()
        }) as Box<dyn FnMut(JsValue) -> react_sys::Element>);

        Self {
            kind: f.into_js_value(),
            properties: JsValue::from_serde(component).unwrap(),
            children: Array::default(),
        }
    }

    pub fn child(self, node: impl Into<JsValue>) -> Self {
        self.children.push(&node.into());
        self
    }

    pub fn create(self) -> react_sys::Element {
        react_sys::create_element(&self.kind, &self.properties.into(), &self.children)
    }
}

pub trait Component: Serialize + DeserializeOwned {
    fn render(&self) -> Element;

    fn to_element(&self) -> Element {
        let f = Closure::wrap(Box::new(move |props: JsValue| {
            let c: Self = props.into_serde().unwrap();
            c.render().create()
        }) as Box<dyn FnMut(JsValue) -> react_sys::Element>);

        Element {
            kind: f.into_js_value(),
            properties: JsValue::from_serde(self).unwrap(),
            children: Array::default(),
        }
    }
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

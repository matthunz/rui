use crate::Element;
use crate::IntoElement;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::Event;

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

    pub fn on_click<F>(self, f: F) -> Self
    where
        F: FnMut(Event) + 'static,
    {
        let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut(Event)>);
        self.property("onClick", &closure.into_js_value())
    }
}

impl From<Html> for Element {
    fn from(html: Html) -> Self {
        Self::new(html.tag, html.properties, Array::default())
    }
}

impl IntoElement for Html {
    fn into_element(self) -> Element {
        self.into()
    }
}

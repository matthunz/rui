use crate::Component;
use js_sys::Array;
use wasm_bindgen::prelude::*;

pub struct Element {
    kind: JsValue,
    properties: JsValue,
    children: Array,
}

impl Element {
    pub fn new(kind: JsValue, properties: JsValue, children: Array) -> Self {
        Self {
            kind,
            properties,
            children,
        }
    }

    pub fn create(self) -> react_sys::Element {
        react_sys::create_element(&self.kind, &self.properties.into(), &self.children)
    }
}

pub trait IntoElement {
    fn into_element(self) -> Element;

    fn child(self, node: impl Into<JsValue>) -> Element
    where
        Self: Sized,
    {
        let element = self.into_element();
        element.children.push(&node.into());
        element
    }
}

impl<C> IntoElement for C
where
    C: Component,
{
    fn into_element(self) -> Element {
        self.to_element()
    }
}

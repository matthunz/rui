use crate::Element;

use js_sys::Array;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod context;
pub use context::{Context, State};

pub trait Component: Serialize + DeserializeOwned {
    fn render(&self, context: Context) -> Element;

    fn to_element(&self) -> Element {
        let f = Closure::wrap(Box::new(move |props: JsValue| {
            let component: Self = props.into_serde().unwrap();
            let element = component.render(Context::new());
            element.create()
        }) as Box<dyn FnMut(JsValue) -> react_sys::Element>);

        Element::new(
            f.into_js_value(),
            JsValue::from_serde(self).unwrap(),
            Array::default(),
        )
    }
}

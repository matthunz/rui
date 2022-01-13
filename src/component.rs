use crate::Element;
use core::ops::Deref;
use js_sys::{Array, Function};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub trait Component: Serialize + DeserializeOwned {
    fn render(&self) -> Element;

    fn to_element(&self) -> Element {
        let f = Closure::wrap(Box::new(move |props: JsValue| {
            let c: Self = props.into_serde().unwrap();
            c.render().create()
        }) as Box<dyn FnMut(JsValue) -> react_sys::Element>);

        Element::new(
            f.into_js_value(),
            JsValue::from_serde(self).unwrap(),
            Array::default(),
        )
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

use core::ops::Deref;
use js_sys::Function;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

pub struct Context {
    _private: (),
}

impl Context {
    pub(super) fn new() -> Self {
        Self { _private: () }
    }

    pub fn state<T>(&self, initial: T) -> State<T>
    where
        T: DeserializeOwned + Into<JsValue>,
    {
        let array = react_sys::use_state(initial.into());
        let f: Function = array[1].clone().into();
        State {
            value: array[0].clone().into_serde().unwrap(),
            set_value: f,
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

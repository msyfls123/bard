use wasm_bindgen::prelude::*;
use web_sys::{console};
use js_sys::{Reflect};

#[wasm_bindgen()]
pub fn log(name: &str) {
    console::log_1(&JsValue::from(&format!("Hello, {}!", name)));
}

#[wasm_bindgen()]
pub fn keys(data: &JsValue) {
    let keys = Reflect::own_keys(data).unwrap_throw();
    for key in keys.iter() {
        console::log_1(&key);
    }
}

#[macro_use] extern crate serde_json;

use app::AppProps;
use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use js_sys::{Reflect, Function};

mod app;
mod component;
mod helpers;

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

#[wasm_bindgen]
pub fn run_app(upload_file: Function) -> Result<(), JsValue> {
    let window = window().expect("window not existed");
    let document = window.document().expect("document not existed");
    let element = document.get_element_by_id("app").unwrap_throw();
    yew::start_app_with_props_in_element::<app::App>(element, AppProps { upload_file });

    Ok(())
}

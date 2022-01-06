use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use js_sys::{Reflect};

mod app;

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
pub fn run_app() -> Result<(), JsValue> {
    let window = window().expect("window not existed");
    let document = window.document().expect("document not existed");
    let element = document.get_element_by_id("app").unwrap_throw();
    yew::start_app_in_element::<app::App>(element);

    Ok(())
}

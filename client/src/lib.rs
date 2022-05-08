#[macro_use] extern crate serde_json;

use constants::app::AppProps;
use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use js_sys::{Reflect, Function, Object};
use constants::auth::User;

mod app;
mod component;
mod helpers;
mod constants;
mod container;

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
pub fn run_app(props: Object) -> Result<(), JsValue> {
    let window = window().expect("window not existed");
    let document = window.document().expect("document not existed");
    let element = document.get_element_by_id("app").unwrap_throw();
    let upload_file: Function = Reflect::get(&props, &JsValue::from_str("upload_file")).unwrap().into();
    let user_obj: JsValue = Reflect::get(&props, &JsValue::from_str("user")).unwrap();
    let user: Option<User> = helpers::user::parse_user(user_obj);
    yew::start_app_with_props_in_element::<app::Main>(element, AppProps { upload_file, user });

    Ok(())
}

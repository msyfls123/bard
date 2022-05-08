use crate::constants::auth::User;
use wasm_bindgen::{JsValue};
use web_sys::{console};

pub fn parse_user(user_obj: JsValue) -> Option<User> {
    match user_obj.into_serde::<User>() {
        Ok(u) => {
            console::log_1(&JsValue::from_serde(&u).unwrap());
            Some(u)
        },
        Err(e) => {
            console::log_1(&JsValue::from_str(&e.to_string()));
            None
        },
    }
}
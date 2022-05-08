use wasm_bindgen::{JsValue, JsCast};
use crate::constants::auth::{LoginInfo};
use wasm_bindgen_futures::{JsFuture};
use web_sys::{window, Request, RequestInit, Response};
use js_sys::JSON;

pub async fn post_login(info: &LoginInfo) -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();
    let headers = json!({
        "Accept": "application/json",
        "Content-Type": "application/json",
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    let json_obj = JsValue::from_serde(&info).unwrap();
    let json_str = JSON::stringify(&json_obj).unwrap();
    options.body(Some(&json_str));
    options.method("post");
    let req = Request::new_with_str_and_init("/login", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}

pub async fn fetch_user() -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();
    let headers = json!({
        "Accept": "application/json",
        "Content-Type": "application/json",
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    options.method("get");
    let req = Request::new_with_str_and_init("/me", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}

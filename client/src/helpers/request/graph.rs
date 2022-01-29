use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{window, Request, RequestInit, Response};

pub fn get_vertex_data<F: 'static>(callback: F) where F: FnOnce(JsValue) -> () {
    let win = window().unwrap();
    let mut options = RequestInit::new();
    let headers = json!({
        "Accept": "application/json"
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    let req = Request::new_with_str_and_init("./vertex", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    spawn_local(async {
        let raw_res = JsFuture::from(fetch_res).await.unwrap();
        let res: Response = raw_res.dyn_into().unwrap();
        let promise_data = res.json().unwrap();
        let data = JsFuture::from(promise_data).await.unwrap();
        callback(data);
    });
}
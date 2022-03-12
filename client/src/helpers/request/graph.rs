use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::{JsFuture};
use web_sys::{window, Request, RequestInit, Response};

pub async fn get_vertex_data() -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();
    let headers = json!({
        "Accept": "application/json"
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    let req = Request::new_with_str_and_init("./vertex", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}
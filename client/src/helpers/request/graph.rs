use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{window, Request, RequestInit, console, Response};

// TODO use wasm_bindgen_futures::spawn_local to pipe promise
pub fn get_vertex_data<F: 'static>(callback: F) where F: FnOnce(JsValue) -> () {
    let win = window().unwrap();
    let mut options = RequestInit::new();
    let headers = json!({
        "Accept": "application/json"
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    let req = Request::new_with_str_and_init("./vertex", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let callback1 = Closure::once(Box::new(|raw_res: JsValue| {
        console::log_1(&raw_res);
        let res: Response = raw_res.dyn_into().unwrap();
        let json_data = res.json().unwrap();
        let callback2 = Closure::once(Box::new(|data| {
            console::log_1(&data);
            callback(data);
        }));
        json_data.then(&callback2);
        callback2.forget();
    }));
    fetch_res.then(&callback1);
    callback1.forget();
}
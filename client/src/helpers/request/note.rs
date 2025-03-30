use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::{JsFuture};
use web_sys::{console, window, Request, RequestInit, Response};
use serde::{Serialize, Deserialize};
use js_sys::{JSON, Date};

#[derive(Serialize)]
pub struct NoteProperties {
    pub text: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct CreateNoteInfo {
    pub t: String,
    pub properties: NoteProperties,
}

#[derive(Serialize)]
pub struct FetchNotesQuery {
    pub query: String,
}

#[derive(Deserialize)]
pub struct  Note {
    pub id: String,
    pub text: String,
    pub timestamp: Option<String>,
}

#[derive(Deserialize)]
pub struct  NoteRes {
    pub notes: Vec<Note>,
}

#[derive(Deserialize)]
pub struct  FetchNotesResponse {
    pub data: NoteRes,
}

pub async fn creat_note(text: &str) -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();

    let now = Date::new_0();
    let timestamp = now.get_time();

    let info = CreateNoteInfo {
        t: "note".to_string(),
        properties: NoteProperties {
            text: text.to_string(),
            timestamp: timestamp.to_string(), 
        },
    };
    let json_obj = JsValue::from_serde(&info).unwrap();
    let json_str = JSON::stringify(&json_obj).unwrap();

    let headers = json!({
        "Accept": "application/json",
        "Content-Type": "application/json",
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    options.method("post");
    options.body(Some(&json_str));


    let req = Request::new_with_str_and_init("/vertex", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}

pub async fn fetch_notes(start_id: Option<String>, limit: usize) -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();

    // query: "{\n  notes(limit: 10) {\n    id\n    text\n  }\n}"
    let query = format!(
        "{{\n  notes(startId: {}, limit: {},) {{\n    id\n    text\n    timestamp\n  }}\n}}",
        start_id.map_or(String::from("null"), |v| format!(r#""{}""#, v)),
        limit
    );

    let info = FetchNotesQuery {
        query,
    };
    let json_obj = JsValue::from_serde(&info).unwrap();
    let json_str = JSON::stringify(&json_obj).unwrap();

    let headers = json!({
        "Accept": "application/json",
        "Content-Type": "application/json",
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    options.method("post");
    options.body(Some(&json_str));


    let req = Request::new_with_str_and_init("/graphql", &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}

pub async fn delete_note(id: &str) -> Result<JsValue, JsValue> {
    let win = window().unwrap();
    let mut options = RequestInit::new();

    let headers = json!({
        "Accept": "application/json",
        "Content-Type": "application/json",
    });
    options.headers(&JsValue::from_serde(&headers).unwrap());
    options.method("delete");

    let req = Request::new_with_str_and_init(&format!("/vertex/{}", id), &options).unwrap();
    let fetch_res = win.fetch_with_request(&req);
    let raw_res = JsFuture::from(fetch_res).await.unwrap();
    let res: Response = raw_res.dyn_into().unwrap();
    let promise_data = res.json().unwrap();
    JsFuture::from(promise_data).await
}

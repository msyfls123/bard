use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{console, Event, HtmlInputElement};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use yew::{ html, prelude::function_component, use_callback, use_context, use_state, Callback, Html, Properties};
use js_sys::{Promise, JSON::stringify_with_replacer_and_space};

use crate::constants::app::AppContext;

#[derive(Properties, PartialEq)]
pub struct UploaderProps {
    pub on_upload_start: Callback<()>,
    pub on_upload_end: Callback<()>,
}

#[function_component(Uploader)]
pub fn uploader(props: &UploaderProps) -> Html {
    // context
    let app_ctx = use_context::<AppContext>().expect("no ctx found");
    let cos_upload = app_ctx.upload_file;
    // state
    let upload_result = use_state(|| None);
    let is_uploading = use_state(|| false);
    // callbacks
    let upload_start = use_callback(
        (props.on_upload_start.clone(), is_uploading.clone()),
        move |_, (on_start, uploading)| {
            on_start.emit(());
            uploading.set(true);
        }
    );
    let upload_end = use_callback(
        (props.on_upload_end.clone(), upload_result.clone()),
        move |res, (on_end, res_state)| {
            res_state.set(res);
            on_end.emit(());
        }
    );

    let on_upload = use_callback(upload_end.clone(), move |e: Event, upload_cb| {
        upload_start.emit(());
        let upload_cb = upload_cb.clone();
        let result = cos_upload.call1(&JsValue::NULL, e.as_ref()).unwrap();
        let promise = Promise::from(result);
        let closure = Closure::once(Box::new(move |value: JsValue| {
            console::log_1(&value);
            upload_cb.emit(Some(value));
        }) as Box<dyn FnMut(JsValue)>);
        let promise = promise.then(&closure);
        closure.forget();
        let future = JsFuture::from(promise);
        spawn_local(async {
            match future.await {
                Ok(_res) => {},
                Err(err) => console::error_1(&err),
            };
        });
    });


    html! {
        <div>
            {if let Some(data) = upload_result.as_ref() {
                let upload_text = stringify_with_replacer_and_space(&data, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap();
                html! { <div>
                    { "Upload success" }
                    <pre>{upload_text.as_string().unwrap_or_default()}</pre>
                </div> }
            } else if *is_uploading {
                html! { <div>{ "Uploading..." }</div> }
            } else {
                html! {
                    <input type="file" onchange={move |e: Event| {
                        let target = e.target().unwrap();
                        let input_el: HtmlInputElement = target.dyn_into().unwrap();
                        let option_file = input_el.files().unwrap().item(0);
                        if let Some(file) = option_file {
                            on_upload.emit(e)
                        }
                    }} />
                }
            }}
        </div>
    }
}

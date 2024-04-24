use serde_wasm_bindgen::from_value;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{console, Event, HtmlInputElement};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use yew::{ html, prelude::function_component, use_callback, use_context, use_memo, use_state, Callback, Html, Properties, UseStateHandle};
use js_sys::{Promise, JSON::stringify_with_replacer_and_space};
use serde_json::{Value};
use serde::{Deserialize};

use crate::constants::app::AppContext;

#[derive(Deserialize, PartialEq, Clone)]
pub struct Progress {
    percent: Value,
    total: Value
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct UploadResult {
    Location: Value,
}

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
    let upload_result: UseStateHandle<Option<JsValue>> = use_state(|| None);
    let is_uploading = use_state(|| false);
    let filename = use_state(|| None);
    let progress = use_state(|| 0f64);
    // computed
    let percentage = use_memo(progress.clone(), |progress| {
        let progress = *progress.clone();
        format!("{:.1}%", progress * 100.0f64)
    });
    let circle_style = use_memo(progress.clone(), |progress| {
        let progress = *progress.clone();
        format!("--percentage: {:.1};", progress * 100.0f64)
    });
    let location = use_memo(upload_result.clone(), |res| {
        match res.as_ref() {
            Some(v) => {
                let parsed: UploadResult = from_value(v.to_owned()).unwrap();
                "https://".to_string() + parsed.Location.as_str().unwrap_or_default()
            },
            None => "".to_string()
        }
    });
    // callbacks
    let upload_start = use_callback(
        (props.on_upload_start.clone(), is_uploading.clone(), filename.clone()),
        move |e: Event, (on_start, uploading, filename)| {
            on_start.emit(());
            uploading.set(true);

            let target = e.target().unwrap();
            let input_el: HtmlInputElement = target.dyn_into().unwrap();
            let option_file = input_el.files().unwrap().item(0);
            filename.set(option_file.map(|file| file.name()));
        }
    );
    let upload_end = use_callback(
        (props.on_upload_end.clone(), upload_result.clone()),
        move |res, (on_end, res_state)| {
            res_state.set(res);
            on_end.emit(());
        }
    );

    let on_upload = use_callback(
        (upload_end.clone(), progress.clone()),
        move |e: Event, (upload_cb, progress_state)| {
            upload_start.emit(e.clone());

            let progress_state = progress_state.clone();
            let closure = Closure::wrap(Box::new(move |payload: JsValue| {
                console::log_2(&JsValue::from_str("received"), &payload);
                let payload_value: Progress = from_value(payload).unwrap();
                let percent = payload_value.percent.as_f64().unwrap();
                progress_state.set(percent)
            }) as Box<dyn FnMut(JsValue)>);

            // invoke upload
            let result = cos_upload.call2(
                &JsValue::NULL,
                e.as_ref(),
                closure.as_ref().unchecked_ref()
            ).unwrap();

            closure.forget();

            // promise.then
            let promise = Promise::from(result);
            let upload_cb = upload_cb.clone();
            let closure = Closure::once(Box::new(move |value: JsValue| {
                console::log_1(&value);
                upload_cb.emit(Some(value));
            }) as Box<dyn FnMut(JsValue)>);
            let promise = promise.then(&closure);
            closure.forget();

            // spawn promise future
            let future = JsFuture::from(promise);
            spawn_local(async {
                match future.await {
                    Ok(_res) => {},
                    Err(err) => console::error_1(&err),
                };
            });
        },
    );

    html! {
        <div class="upload-item">
            {if let Some(_) = upload_result.as_ref() {
                // let upload_text = stringify_with_replacer_and_space(&data, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap();
                html! { <div class="file-link">
                    <i class="icon-success fa-solid fa-check"></i>
                    <a target="_blank" href={<std::string::String as Clone>::clone(&location)}>
                        {if let Some(name) = filename.as_ref() {
                            html!{ <span> {{ name }} </span> }
                        } else {
                            html!{ <span> {"[Untitled]"} </span> }
                        }}
                    </a>
                </div> }
            } else if *is_uploading {
                html! {
                    <div class="progress">
                        <svg
                            class="progress-circle"
                            xmlns="http://www.w3.org/2000/svg"
                            version="1.1"
                            viewBox="0 0 100 100"
                            width="100px"
                            height="100px"
                        >
                            <circle
                                cx="50"
                                cy="50"
                                r="45"
                                stroke-linecap="round"
                                stroke="#9733EE"
                                style={<std::string::String as Clone>::clone(&*circle_style.clone())}
                            />
                        </svg>
                        <div class="outer">
                            <div class="inner">
                                <div class="num"> {{ percentage }} </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="upload-input">
                        <input type="file" onchange={move |e: Event| {
                            let target = e.target().unwrap();
                            let input_el: HtmlInputElement = target.dyn_into().unwrap();
                            let option_file = input_el.files().unwrap().item(0);
                            if let Some(file) = option_file {
                                on_upload.emit(e)
                            }
                        }} />
                    </div>
                }
            }}
        </div>
    }
}

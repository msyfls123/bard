use js_sys::{Function, Promise, JSON::stringify_with_replacer_and_space};
use wasm_bindgen::{JsValue, prelude::Closure};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::console;
use yew::{prelude::*, virtual_dom::VNode};

use crate::component::graph::Graph;

pub struct App {
    value: i64,
    upload_result: Option<JsValue>,
}

pub enum Msg {
    AddOne,
    FileChange(Event),
    Upload(JsValue),
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub upload_file: Function
}

impl Component for App {
    type Message = Msg;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0, upload_result: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            },
            Msg::Upload(value) => {
                self.upload_result = Some(value);
                true
            },
            Msg::FileChange(e) => {
                let callback = ctx.link().callback_once(|value: JsValue| {
                    Msg::Upload(value)
                });
                let func = &ctx.props().upload_file;
                let result = func.call1(&JsValue::NULL, e.as_ref()).unwrap();
                let promise = Promise::from(result);
                let closure = Closure::once(Box::new(move |value: JsValue| {
                    console::log_1(&value);
                    callback.emit(value);
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
                true
            }
        }
    }

    fn view(&self, context: &Context<Self>) -> VNode {
        let link = context.link();
        html! {
            <div>
              <p>{ "count " } { self.value } </p>
              <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
              <p>{ "Hello world!" }</p>
              <form method="post" enctype="multipart/form-data" action="/upload">
                <input type="file" name="file"/>
                <button type="submit">{"Submit"}</button>
              </form>
              <Graph/>
              <h2>{"Upload"}</h2>
              <input type="file" onchange={link.callback(|e| Msg::FileChange(e))}/>
              <p>
                { if self.upload_result.is_some() {
                    let upload_json = self.upload_result.to_owned().unwrap();
                    let upload_text = stringify_with_replacer_and_space(&upload_json, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap();
                    html! { <pre>{upload_text}</pre>}
                } else {
                    html!{}
                }}
              </p>
            </div>
        }
    }
}

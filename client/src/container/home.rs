use chrono::Utc;
use js_sys::Function;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::{prelude::*, virtual_dom::VNode,function_component};

use crate::component::graph::Graph;
use crate::component::bucket::Bucket;
use crate::component::Uploader;
use crate::constants::app::AppContext;

fn get_now() -> String {
    let now = Utc::now();
    let time: String = format!("{}", now);
    return time
}

pub struct HomeInner {
    refresh_index: i64,
    input_keys: Vec<String>,
}

pub enum Msg {
    Refresh,
    UploadAddOne,
    UploadEnd,
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub upload_file: Function
}

impl Component for HomeInner {
    type Message = Msg;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        HomeInner { refresh_index: 1, input_keys: vec!{get_now()} }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Refresh => {
                self.refresh_index += 1;
                true
            },
            Msg::UploadAddOne => {
                self.input_keys = [vec!{get_now()}, self.input_keys.to_owned()].concat();
                console::log_1(&JsValue::from_str("upload start"));
                true
            },
            Msg::UploadEnd => {
                console::log_1(&JsValue::from_str("upload end"));
                self.refresh_index += 1;
                true
            },
        }
    }

    fn view(&self, context: &Context<Self>) -> VNode {
        let link = context.link();
        let list = self.input_keys.clone();
        let upload_start = link.callback(|_| Msg::UploadAddOne);
        let upload_end = link.callback(|_| Msg::UploadEnd);

        html! {
            <div>
              <div class="hidden">
                <p>{ "refresh index " } { self.refresh_index } </p>
                <button onclick={link.callback(|_| Msg::Refresh)}>{ "+1" }</button>
                <p>{ "Hello world!" }</p>
                <form class="hidden" method="post" enctype="multipart/form-data" action="/upload">
                    <input type="file" name="file"/>
                    <button type="submit">{"Submit"}</button>
                </form>
                <Graph/>
              </div>
              <Bucket refresh_index={self.refresh_index as usize}/>
              <h2>{"Upload"}</h2>
              { list.into_iter().map(|i| html! {
                <div key={i.clone()}>
                    <Uploader
                        on_upload_start={upload_start.clone()}
                        on_upload_end={upload_end.clone()}
                    />
                </div>
            }).collect::<Html>()}
            </div>
        }
    }
}

#[function_component(Home)]
pub fn HomeHOC() -> Html {
    let app_ctx = use_context::<AppContext>().expect("no ctx found");
    html! { <HomeInner upload_file={app_ctx.upload_file}/> }
}

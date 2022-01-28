use wasm_bindgen::JsValue;
use js_sys::JSON::stringify_with_replacer_and_space;
use yew::{Context, Component, Html, prelude::html};

use crate::helpers::request::graph;

pub enum Msg {
    Fetch,
    Vertex(JsValue)
}

pub struct Graph {
    vertex_data: Option<JsValue>
}

impl Component for Graph {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { vertex_data: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().to_owned();
        match msg {
            Msg::Vertex(vertex_data) => {
                self.vertex_data = Some(vertex_data);
                true
            },
            Msg::Fetch => {
                graph::get_vertex_data(move |data| {
                    link.send_message(Msg::Vertex(data))
                });
                false
            }
            _ => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onclick_fetch_btn = link.callback(|_| {
            Msg::Fetch
        });
        html!{
            <div>
                <h2>{"Vertex"}</h2>
                <button onclick={onclick_fetch_btn}>{"fetch"}</button>
                { if self.vertex_data.is_some() {
                    let vertex_json = self.vertex_data.to_owned().unwrap();
                    let vertex_text = stringify_with_replacer_and_space(&vertex_json, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap();
                    html! { <pre>{vertex_text}</pre>}
                } else {
                    html!{}
                }}
            </div>
        }
    }
}
use js_sys::JSON::stringify_with_replacer_and_space;
use yew::{Context, Component, Html, prelude::html};
use wasm_bindgen::{JsValue};

use crate::helpers::request::graph;

pub enum Msg {
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Vertex(vertex_data) => {
                self.vertex_data = Some(vertex_data);
                true
            },
            _ => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
       
        let onclick_fetch_btn = link.callback_future(|_| {
            async {
                let result = graph::get_vertex_data().await.unwrap();
                Msg::Vertex(result)
            }
        });
        html!{
            <div>
                <h2>{"Vertex"}</h2>
                <button onclick={onclick_fetch_btn}>{"fetch"}</button>
                { if self.vertex_data.is_some() {
                    let vertex_json = self.vertex_data.to_owned().unwrap();
                    let vertex_text = stringify_with_replacer_and_space(&vertex_json, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap();
                    html! { <pre>{vertex_text.as_string().unwrap_or_default()}</pre>}
                } else {
                    html!{}
                }}
            </div>
        }
    }
}

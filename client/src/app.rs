use yew::{prelude::*, virtual_dom::VNode};

use crate::component::graph::Graph;

pub struct App {
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
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
            </div>
        }
    }
}

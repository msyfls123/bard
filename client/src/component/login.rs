use yew::prelude::{function_component, html};
use web_sys::{FocusEvent, HtmlFormElement, Element, HtmlInputElement, console, window};
use wasm_bindgen::{JsValue};
use yew::{Callback};
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use serde_json::{Value};

use crate::helpers::request::auth::post_login;
use crate::constants::auth::LoginInfo;
use crate::constants::app::{Route};

fn get_value(form:  &HtmlFormElement, name: &str) -> String {
    let input: Element = form.elements().get_with_name(name).unwrap();
    let value: &JsValue = input.as_ref();
    let input_ele: HtmlInputElement = value.to_owned().into();
    input_ele.value()
}

#[function_component(Login)]
pub fn login() -> Html {
    let history = use_history().unwrap();
    html! {
        <form onsubmit={Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let target = e.target().unwrap();
            let value: &JsValue = target.as_ref();
            let form: HtmlFormElement = value.to_owned().into();
            let email = get_value(&form, "email");
            let password = get_value(&form, "password");
            let history_clone = history.clone();
            spawn_local(async move {
                let info = LoginInfo {
                    email,
                    password,
                };
                let result = post_login(&info).await.unwrap();
                console::log_1(&result);
                let value: Value = result.into_serde().unwrap();
                if value.get("status") == Some(&json!{"success"}) {
                    history_clone.push(Route::Me);
                } else {
                    let win = window().unwrap();
                    win.alert_with_message(&value.get("message").unwrap().to_string());
                }
            });
        })}>
            <fieldset>
                <label>{"email"}</label>
                <input type="text" name="email"/>
            </fieldset>
            <fieldset>
                <label>{"password"}</label>
                <input type="password" name="password"/>
            </fieldset>
            <button type="submit" value="login" formaction="/login">{"Login"}</button>
        </form>
    }
}
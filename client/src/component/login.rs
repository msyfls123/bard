use yew::prelude::{function_component, html, use_state};
use web_sys::{FocusEvent, HtmlFormElement, Element, HtmlInputElement, console, window, Event};
use wasm_bindgen::{JsValue};
use yew::{Callback};
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use serde_json::{Value};

use crate::helpers::request::auth::post_login;
use crate::constants::auth::LoginInfo;
use crate::constants::app::{Route};

fn get_value(form: &HtmlFormElement, name: &str) -> String {
    let input: Element = form.elements().get_with_name(name).unwrap();
    let value: &JsValue = input.as_ref();
    let input_ele: HtmlInputElement = value.to_owned().into();
    input_ele.value()
}

#[function_component(Login)]
pub fn login() -> Html {
    let history = use_history().unwrap();
    let is_signup = use_state(|| true);
    let onchange_signup = {
        let is_signup = is_signup.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let value: &JsValue = target.as_ref();
            let checkbox: HtmlInputElement = value.to_owned().into();
            let checked = checkbox.checked();
            console::log_1(&JsValue::from_bool(checked));
            is_signup.set(checkbox.checked());
        })
    };

    let onsubmit = {
        let is_signup = is_signup.clone();
        let history = history.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let target = e.target().unwrap();
            let value: &JsValue = target.as_ref();
            let form: HtmlFormElement = value.to_owned().into();
            let email = get_value(&form, "email");
            let password = get_value(&form, "password");
            let is_signup = is_signup.clone();
            let history = history.clone();
            spawn_local(async move {
                let info = LoginInfo {
                    email,
                    password,
                };
                let result = post_login(&info, *is_signup).await.unwrap();
                console::log_1(&result);
                let value: Value = result.into_serde().unwrap();
                if value.get("status") == Some(&json!{"success"}) {
                    history.push(Route::Me);
                } else {
                    let win = window().unwrap();
                    win.alert_with_message(&value.get("message").unwrap().to_string());
                }
            });
        })
    };
    html! {
        <form onsubmit={onsubmit}>
            <fieldset>
                <label>{"email"}</label>
                <input type="text" name="email"/>
            </fieldset>
            <fieldset>
                <label>{"password"}</label>
                <input type="password" name="password"/>
            </fieldset>
            <fieldset>
                <label>{"Sign up?"}</label>
                <input type="checkbox" name="signup" onchange={onchange_signup} checked={*is_signup} />
            </fieldset>
            <button type="submit">{
                if *is_signup {
                    "Sign Up"
                } else {
                    "Login"
                }
            }</button>
        </form>
    }
}
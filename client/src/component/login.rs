use yew::prelude::{function_component, html};
use web_sys::{FocusEvent, HtmlFormElement, Element, HtmlInputElement, console};
use wasm_bindgen::{JsValue};
use yew::{Callback};

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <form onsubmit={Callback::from(|e: FocusEvent| {
            e.prevent_default();
            let target = e.target().unwrap();
            let value: &JsValue = target.as_ref();
            let form: HtmlFormElement = value.to_owned().into();
            let input: Element = form.elements().get_with_name("email").unwrap();
            let value: &JsValue = input.as_ref();
            let input_ele: HtmlInputElement = value.to_owned().into();
            let value = input_ele.value();
            console::log_1(&value.into());
        })}>
            <input type="text" name="email"/>
            <button type="submit" name="type" value="login">
                {"Login"}
            </button>
        </form>
    }
}
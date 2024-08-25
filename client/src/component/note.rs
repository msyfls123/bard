use std::ops::Deref;
use serde_wasm_bindgen::from_value;
use web_sys::{HtmlInputElement, console};
use yew::{html, prelude::function_component, use_effect_with, use_state, Callback, Event, Html, InputEvent, MouseEvent};
use wasm_bindgen::{JsCast, JsValue};
use crate::helpers::request::note::{creat_note, fetch_notes, FetchNotesResponse};
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[function_component(Note)]
pub fn note() -> Html {
    let input_text = use_state(|| "".to_string());
    let refresh_index = use_state(|| 0);
    let notes = use_state(|| vec![]);
    let notes_cloned = notes.clone();

    let on_change = {
        let input_text = input_text.clone();
        move |e: InputEvent| {
            let target = e.target().unwrap();
            let input_el: HtmlInputElement = target.dyn_into().unwrap();
            input_text.set(input_el.value());
        }
    };

    let on_submit = {
        let input_text_clone = input_text.clone();
        let refresh_index_clone = refresh_index.clone();
        Callback::from(move |_e: MouseEvent| {
            let input_text_cloned = input_text_clone.clone();
            let refresh_index_clone = refresh_index_clone.clone();
            spawn_local(async move {
                let payload = creat_note(&input_text_cloned.deref()).await.unwrap();
                console::log_2(&JsValue::from_str("create note"), &payload);
                refresh_index_clone.set(*refresh_index_clone + 1);
            });
        })
    };

    let update_list = move |&flag: &u32| {

        let cleanup = || {};
        let notes_clone = notes.clone();

        spawn_local(async move {
            let res = fetch_notes(None, 20).await.unwrap();
            console::log_2(&JsValue::from_str("received notes"), &res);
            let res_obj: FetchNotesResponse = from_value(res).unwrap();
            let list = res_obj.data.notes;
            notes_clone.set(list);
        });

        cleanup
    };

    use_effect_with(
        *refresh_index,
        update_list.clone(),
    );


    html! {
        <div>
            <p>{input_text.deref().clone()}</p>
            <input type="text" value={input_text.deref().clone()} oninput={on_change}/>
            <button onclick={on_submit}>{"Submit"}</button>
            <div>
                { for notes_cloned.iter().map(|note| html! {
                    <div>
                        <p>{note.text.clone()}</p>
                    </div>
                })}
            </div>
        </div>
    }
}

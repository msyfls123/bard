use std::ops::Deref;
use serde_wasm_bindgen::from_value;
use web_sys::{console, HtmlTextAreaElement};
use yew::{html, prelude::function_component, use_effect_with, use_state, Callback, classes, Html, InputEvent, MouseEvent};
use wasm_bindgen::{JsCast, JsValue};
use crate::helpers::request::note::{creat_note, delete_note, fetch_notes, FetchNotesResponse};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use js_sys::JSON::parse;

#[function_component(Note)]
pub fn note() -> Html {
    let input_text = use_state(|| "".to_string());
    let refresh_index = use_state(|| 0);
    let notes = use_state(|| vec![]);
    let notes_cloned = notes.clone();
    let highlight_id = use_state(|| "".to_string());

    let on_change = {
        let input_text = input_text.clone();
        move |e: InputEvent| {
            let target = e.target().unwrap();
            let input_el: HtmlTextAreaElement = target.dyn_into().unwrap();
            input_text.set(input_el.value());
        }
    };

    let on_submit = {
        let input_text_clone = input_text.clone();
        let refresh_index_clone = refresh_index.clone();
        Callback::from(move |_e: MouseEvent| {
            let input_text_cloned = input_text_clone.clone();
            let input_text_cloned2 = input_text_clone.clone();
            let refresh_index_clone = refresh_index_clone.clone();
            spawn_local(async move {
                let payload = creat_note(&input_text_cloned.deref()).await.unwrap();
                input_text_cloned2.set("".to_string());
                console::log_2(&JsValue::from_str("create note"), &payload);
                refresh_index_clone.set(*refresh_index_clone + 1);
            });
        })
    };

    let on_delete = {
        let refresh_index_clone = refresh_index.clone();
        move |id: String| {
            let refresh_index_clone = refresh_index_clone.clone();
            let id_clone = id.clone();
            Callback::from(move |_e: MouseEvent| {
                let refresh_index_clone = refresh_index_clone.clone();
                let id_clone = id_clone.clone();
                spawn_local(async move {
                    let res = delete_note(&id_clone).await.unwrap();
                    console::log_2(&JsValue::from_str("delete note"), &res);
                    refresh_index_clone.set(*refresh_index_clone + 1);
                });
            })
        }
    };

    let on_copy = {
        let h = highlight_id.clone();
        move |id: String, text: String| {
            let h = h.clone();
            let text = text.clone();
            let id = id.clone();
            Callback::from(move |_e: MouseEvent| {
                let navigator = web_sys::window().unwrap().navigator();
                let clipboard = navigator.clipboard();
                let promise = clipboard.write_text(&text);
                let future = JsFuture::from(promise);
                let h = h.clone();
                let id = id.clone();
                spawn_local(async move {
                    let _ = future.await;
                    h.set(id);
                });
            })
        }
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
        <div class="note">
            <textarea
                type="text"
                class="textarea"
                value={input_text.deref().clone()}
                oninput={on_change}
            >
            </textarea>
            <button class="btn btn-submit" onclick={on_submit}>{"Submit"}</button>
            <ul class="list">
                { for notes_cloned.iter().map(|note| html! {
                    <li
                        class={classes!(
                            "record",
                            if *highlight_id.clone() == note.id { "highlight" } else { "" }
                        )}
                        data-id={note.id.to_owned()}
                    >
                        
                        <span class="checked">
                            <i class="fa-solid fa-check"></i>
                        </span>
                        <p class="content">{parse(&note.text.clone()).unwrap().as_string().unwrap()}</p>
                        <div class="controls">
                            <button
                                class="btn btn-copy"
                                onclick={on_copy(note.id.clone(), parse(&note.text.clone()).unwrap().as_string().unwrap())}
                            >
                                <i class="fa-solid fa-copy"></i>
                            </button>
                            <button
                                class="btn btn-delete"
                                onclick={on_delete(note.id.clone())}
                            >
                                <i class="fa-solid fa-delete-left"></i>
                            </button>
                        </div>
                    </li>
                })}
            </ul>
        </div>
    }
}

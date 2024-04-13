use js_sys::Promise;
use serde_json::{Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::{prelude::function_component, Html, html, use_context, use_state, use_effect_with};
use web_sys::{console};
use serde_wasm_bindgen::from_value;
use serde::{Deserialize};

use crate::constants::app::AppContext;

#[derive(Deserialize, PartialEq, Clone)]
pub struct File {
    Key: Value,
    Size: Value,
}

#[derive(Deserialize)]
pub struct  BucketInfo {
    Contents: Vec<File>,
}


#[function_component(Bucket)]
pub fn bucket() -> Html {
    let list_fetch_in_content = use_context::<AppContext>()
        .expect("no ctx list_fetch_in_content found").list_bucket;
    let list = use_state(|| None);
    let fetched = use_state(|| false);

    {
        let list_fetch_in_content = list_fetch_in_content.clone();
        let list_clone = list.clone();
        let fetched_clone = fetched.clone();
        use_effect_with(
            *fetched,
            move |&fetched| {
                let cleanup = || {};
                if fetched {
                    return cleanup;
                }
                spawn_local(async move {
                    let result = list_fetch_in_content.call0(&JsValue::NULL).unwrap();
                    let promise = Promise::from(result);
                    let data = JsFuture::from(promise).await.unwrap();
                    console::log_2(&JsValue::from_str("data"), &data);
                    let info: BucketInfo = from_value(data).unwrap();
                    list_clone.set(Some(info.Contents));
                    fetched_clone.set(true);
                });
                cleanup
            },
        )
    }

    let list = list.as_ref();
    html! {
        <div>
            <h2>{"Bucket"}</h2>
            <ul>
            {
                if list.is_some() {
                    html! {
                        <table>
                            <thead>
                                <tr>
                                    <th>{"File"}</th>
                                    <th>{"Size (bytes)"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for list.unwrap().iter().map(|file| file_item(file))}
                            </tbody>
                        </table>
                        
                    }
                } else {
                    html! { "loading" }
                }
            }
                
            </ul>
        </div>
    }
}

fn file_item(file: &File) -> Html {
    let file_size = file.Size.as_str().unwrap();
    let href = format!("https://cdn.ebichu.cc/{}", file.Key.as_str().unwrap());
    let file_key2 = file.Key.clone();
    html! { <tr>
        <td>
            <a href={href.to_owned()} target="_blank">
                {file_key2.as_str().unwrap()}
            </a>
        </td>
        <td>
            {file_size}
        </td>
    </tr> }
}

use js_sys::{JsString, Number, Promise};
use serde_json::{Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::{prelude::function_component, Html, html, use_context, use_state, use_effect_with, Properties};
use web_sys::{console};
use serde_wasm_bindgen::from_value;
use serde::{Deserialize};
use chrono::{Utc, DateTime};

use crate::constants::app::AppContext;

#[derive(Deserialize, PartialEq, Clone)]
pub struct File {
    Key: Value,
    Size: Value,
    LastModified: Value,
}

#[derive(Deserialize)]
pub struct  BucketInfo {
    Contents: Vec<File>,
}

#[derive(Properties, PartialEq)]
pub struct BucketProps {
    pub refresh_index: usize
}

#[function_component(Bucket)]
pub fn bucket(props: &BucketProps) -> Html {
    let list_fetch_in_content = use_context::<AppContext>()
        .expect("no ctx list_fetch_in_content found").list_bucket;
    let list = use_state(|| None);
    let fetched = use_state(|| 0);
    let is_loading = use_state(|| false);

    {
        let list_fetch_in_content = list_fetch_in_content.clone();
        let list_clone = list.clone();
        let fetched_clone = fetched.clone();
        let is_loading_clone = is_loading.clone();
        let update_list = move |&flag: &usize| {
            let fetched_clone = fetched.clone();

            let cleanup = || {};
            if flag == 1 {
                return cleanup;
            }
            is_loading_clone.set(true);
            spawn_local(async move {
                let result = list_fetch_in_content.call0(&JsValue::NULL).unwrap();
                let promise = Promise::from(result);
                let data = JsFuture::from(promise).await.unwrap();
                console::log_2(&JsValue::from_str("data"), &data);

                let info: BucketInfo = from_value(data).unwrap();
                let mut contents = info.Contents;
                contents.sort_by_key(|file| file.LastModified.as_str().unwrap().parse::<DateTime<Utc>>().unwrap());
                contents.reverse();
                list_clone.set(Some(contents));

                fetched_clone.set(1);
                is_loading_clone.set(false);
            });
            cleanup
        };

        use_effect_with(
            *fetched_clone.clone(),
            update_list.clone(),
        );

        use_effect_with(
            props.refresh_index,
            update_list.clone(),
        );
    }

    let list = list.as_ref();
    html! {
        <div>
            <h2>{"Bucket"}</h2>
            <div>
            {
                if *is_loading {
                    html! { "loading" }
                } else if list.is_some() {
                    html! {
                        <table class="bucket">
                            <thead>
                                <tr>
                                    <th class="file">{"File"}</th>
                                    <th class="size">{"Size (bytes)"}</th>
                                    <th class="time">{"Modified Time"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for list.unwrap().iter().map(|file| file_item(file))}
                            </tbody>
                        </table>
                        
                    }
                }  else {
                    html! { "no data" }
                }
            }
                
            </div>
        </div>
    }
}

fn file_item(file: &File) -> Html {
    let file_size = file.Size.as_str().unwrap().parse::<u32>().unwrap();
    let size_display = Number::from(file_size).to_locale_string("en-US");
    let href = format!("https://cdn.ebichu.cc/{}", file.Key.as_str().unwrap());
    let file_key2 = file.Key.clone();

    let time = file.LastModified.as_str().unwrap();
    let time_parsed: DateTime<Utc> = time.parse().unwrap();
    let time_display = format!("{}", time_parsed.format("%Y-%m-%d %H:%M:%S"));

    html! { <tr>
        <td class="file">
            <a href={href.to_owned()} target="_blank">
                {file_key2.as_str().unwrap()}
            </a>
        </td>
        <td class="size">
            {size_display.as_string().unwrap_or_default()}
        </td>
        <td class="time">
            {time_display}
        </td>
    </tr> }
}

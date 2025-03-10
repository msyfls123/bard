use js_sys::{JsString, Number, Promise};
use serde_json::{Value};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::{html, prelude::function_component, use_context, use_effect_with, use_memo, use_state, Html, Properties};
use web_sys::{console, Event, HtmlSelectElement};
use serde_wasm_bindgen::from_value;
use serde::{Deserialize};
use chrono::{Utc, DateTime};

use crate::constants::app::AppContext;

const CHUNK_SIZE: usize = 10;

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
    pub refresh_index: usize,
    pub prefix: String,
}

#[function_component(Bucket)]
pub fn bucket(props: &BucketProps) -> Html {
    // context
    let list_fetch_in_content = use_context::<AppContext>()
        .expect("no ctx list_fetch_in_content found").list_bucket;
    // props
    let prefix_clone = props.prefix.clone();
    // state
    let list = use_state(|| None);
    let fetched = use_state(|| 0);
    let is_loading = use_state(|| false);

    // computed
    let page_count = list.as_ref().map(|list: &Vec<File>| {
        let len = list.len();
        len / CHUNK_SIZE + if len % CHUNK_SIZE == 0 { 0 } else { 1 }
    }).unwrap_or(0);
    let current_page = use_state(|| 0);

    // effects
    {
        let list_fetch_in_content = list_fetch_in_content.clone();
        let list_clone = list.clone();
        let fetched_clone = fetched.clone();
        let is_loading_clone = is_loading.clone();
        let current_clone = current_page.clone();
        let update_list = move |&flag: &usize| {
            let fetched_clone = fetched.clone();

            let cleanup = || {};
            if flag == 1 {
                return cleanup;
            }
            is_loading_clone.set(true);
            spawn_local(async move {
                let result = list_fetch_in_content.call1(
                    &JsValue::NULL,
                    &JsValue::from_str(&prefix_clone)
                ).unwrap();
                let promise = Promise::from(result);
                let mut data;
                match JsFuture::from(promise).await {
                    Ok(res) => {
                        console::log_2(&JsValue::from_str("data"), &res);
                        data = res;
                    },
                    Err(e) => {
                        console::log_2(&JsValue::from_str("error"), &e);
                        return
                    }
                }
                console::log_2(&JsValue::from_str("data"), &data);

                let info: BucketInfo = from_value(data).unwrap();
                let mut contents = info.Contents;
                contents.sort_by_key(|file| file.LastModified.as_str().unwrap().parse::<DateTime<Utc>>().unwrap());
                contents.reverse();
                current_clone.set(0);
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
    let current = current_page.clone();
    let list_is_empty = list.unwrap_or(&vec!{}).len() == 0;
    html! {
        <div>
            <h2>
                {"Bucket"}
                { if let Some(l) = list.as_ref() {
                    html! { <span class="item-count">{" ("} {l.len()} {")"}</span> }
                } else { html! {} }}
            </h2>
            <div class="bucket">
            {
                if *is_loading {
                    html! { "loading" }
                } else if list.is_some() {
                    html! {<>
                        <table>
                            <thead>
                                <tr>
                                    <th class="file">{"File"}</th>
                                    <th class="size">{"Size (bytes)"}</th>
                                    <th class="time">{"Modified Time"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {html! { if list.unwrap().len() > 0 {
                                {for list.unwrap()
                                    [*current_page * CHUNK_SIZE..((*current_page + 1) * CHUNK_SIZE).min(list.unwrap().len())]
                                    .iter().map(|file| file_item(file))}
                                } else {
                                    <tr>
                                        <td class="file">{"-"}</td>
                                        <td class="size">{"-"}</td>
                                        <td class="time">{"-"}</td>
                                    </tr>
                                }}}
                            </tbody>
                        </table>
                        { html! { if list_is_empty {
                            <></>
                        } else {<select
                            class="bucket-paginator"
                            onchange={move |e: Event| {
                                let target = e.target().unwrap();
                                let input_el: HtmlSelectElement = target.dyn_into().unwrap();
                                current.set(input_el.value().parse::<usize>().unwrap());
                            }}
                        >
                           {for (0..page_count).map(|i| {
                                html! {
                                    <option
                                        selected={*current_page == i}
                                        value={i.to_string()}
                                    >
                                        {i + 1}
                                    </option>
                                }
                            })}
                        </select>}} }
                    </>}
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

use wasm_bindgen_futures::spawn_local;
use yew::{prelude::{function_component, html}, use_context};
use js_sys::{JSON::stringify_with_replacer_and_space};
use yew_router::prelude::{Redirect};
use wasm_bindgen::{JsValue};
use yew::{use_state, use_effect_with_deps};

use crate::{constants::app::Route, helpers::{request::auth::fetch_user, user::parse_user}};

use crate::constants::app::AppContext;

#[function_component(Me)]
pub fn me() -> Html {
    let user_in_context = use_context::<AppContext>().expect("no ctx found").user;
    let user_state = use_state(|| user_in_context);
    let fetched = use_state(|| false);
    {
        let user_state = user_state.clone();
        let fetched_clone = fetched.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let user = fetch_user().await.unwrap();
                    let user = parse_user(user);
                    user_state.set(user);
                    fetched_clone.set(true);
                });
                || {}
            },
            *fetched,
        )
    }
    match &*fetched {
        true => {
            if !user_state.is_some() {
                return html! { <Redirect<Route> to={Route::Login} /> }
            }
        },
        _ => {},
    };
    let user_obj = match &*user_state {
        Some(u) => JsValue::from_serde(&u).unwrap(),
        None => JsValue::NULL,
    };
    html!{
        <div>
            <pre>{stringify_with_replacer_and_space(&user_obj, &JsValue::NULL, &JsValue::from_f64(4.0)).unwrap()}</pre>
        </div>
    }
}
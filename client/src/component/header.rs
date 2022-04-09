use yew::prelude::*;
use yew_router::prelude::*;

use crate::constants::app::{Route};

#[function_component(Header)]
pub fn header() -> Html {
    let location = use_location().unwrap();
    html! {
        <nav>
            <Link<Route> to={Route::Home} classes={
                classes!((location.route() == Some(Route::Home)).then(|| "active"))
            }>
                {"Home"}
            </Link<Route>>
            <Link<Route> to={Route::Login} classes={
                classes!((location.route() == Some(Route::Login)).then(|| "active"))
            }>
                {"Login"}
            </Link<Route>>
        </nav>
    }
}
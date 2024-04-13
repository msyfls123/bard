use yew::prelude::*;
use yew_router::prelude::*;

use crate::constants::app::{Route};

#[function_component(Header)]
pub fn header() -> Html {
    let location = use_location().unwrap();
    html! {
        <nav>
            <Link<Route> to={Route::Home} classes={
                classes!((location.path() == Route::Home.to_path()).then(|| "active"))
            }>
                {"Home"}
            </Link<Route>>
            <Link<Route> to={Route::Login} classes={
                classes!((location.path() == Route::Login.to_path()).then(|| "active"))
            }>
                {"Login"}
            </Link<Route>>
            <Link<Route> to={Route::Me} classes={
                classes!((location.path() == Route::Me.to_path()).then(|| "active"))
            }>
                {"Me"}
            </Link<Route>>
        </nav>
    }
}

use yew::{prelude::*};
use yew_router::prelude::*;
use crate::constants::app::{AppContext, AppProps};

use crate::container::home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { "login" },
        Route::NotFound => html! { "404" },
    }
}

#[function_component(Main)]
pub fn App(props: &AppProps) -> Html {
    html! {
        <ContextProvider<AppContext> context={AppContext {
            upload_file: props.upload_file.clone(),
        }}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<AppContext>>
    }
}
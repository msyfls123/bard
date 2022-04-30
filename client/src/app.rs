use yew::{prelude::*};
use yew_router::prelude::*;
use crate::constants::app::{AppContext, AppProps, Route};

use crate::container::{ Home, Login };
use crate::component::{ Header };



fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { <Login/> },
        Route::Me => html! { "me" },
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
                <Header/>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<AppContext>>
    }
}
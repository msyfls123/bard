use yew::{prelude::*};
use yew_router::prelude::*;
use crate::constants::app::{AppContext, AppProps, Route};

use crate::container::{ Home, Login };
use crate::component::{ Header, Me };



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { <Login/> },
        Route::Me => html! { <Me/> },
        Route::NotFound => html! { "404" },
    }
}

#[function_component(Main)]
pub fn App(props: &AppProps) -> Html {
    html! {
        <ContextProvider<AppContext> context={AppContext {
            user: props.user.clone(),
            upload_file: props.upload_file.clone(),
            list_bucket: props.list_bucket.clone(),
        }}>
            <BrowserRouter>
                <Header/>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AppContext>>
    }
}

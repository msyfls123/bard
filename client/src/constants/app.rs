use js_sys::Function;
use yew::{Properties};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub upload_file: Function
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub upload_file: Function
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
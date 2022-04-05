use js_sys::Function;
use yew::{Properties};

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub upload_file: Function
}

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub upload_file: Function
}